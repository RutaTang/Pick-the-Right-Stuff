import json
import re
import socket

from src.models.base_model import BaseModel
from src.models.ollama_model import OllamaModel
from src.models.openai_model import OpenAIModel
from src.utils.tcp.helper import write_to_stream, read_until_separator, Data


# Write play statistics: array of scores, average score
class Player:
    def __init__(self, model: BaseModel, client_socket: socket.socket):
        self.client_socket = client_socket
        self.model = model

    async def play(self) -> float:
        while True:
            try:
                buffer = read_until_separator(self.client_socket)
            except Exception as e:
                print("Connection closed")
                exit(0)
            response = buffer.decode().strip()
            response = json.loads(response)
            response = Data.from_dict(response)

            if response.require_input:
                print(response.content + "\n")
                history = self.model.get_history()
                message = {
                    "content": response.content,
                    "role": "user"
                }
                history = history + [message]
                self.model.set_history(history)
                prediction = self.model.chat().strip()
                try:
                    prediction = int(prediction)
                except ValueError:
                    match = re.search(r"position\s+(\d+)(?:th|st|nd|rd)?", prediction)
                    if match:
                        prediction = match.group(1)

                prediction = str(prediction)
                print("AI: " + prediction + "\n")
                prediction = Data.from_dict({
                    "require_input": False,
                    "content": prediction
                })
                write_to_stream(self.client_socket, prediction)
            elif "Game Over!" in response.content:
                match = re.search(r"Final score:\s+(\d+)", response.content)
                if match:
                    score = match.group(1)
                else:
                    raise ValueError("No match")
                print(response.content + "\n")
                self.client_socket.close()
                return float(score)
            else:
                print(response.content + "\n")
                message = {
                    "content": response.content,
                    "role": "user"
                }
                history = self.model.get_history()
                history = history + [message]
                self.model.set_history(history)


async def evaluate(n_players: int, ip: str, port: int) -> float:
    score = 0
    for i in range(n_players):
        client_socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        client_socket.connect((ip, port))
        # model = OpenAIModel()
        model = OllamaModel()
        player = Player(model, client_socket)
        score += await player.play()
    average = score / n_players
    print("Average: " + str(average) + "\n")
    return average
