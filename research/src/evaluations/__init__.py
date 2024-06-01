import json
import re
import socket

from src.models.choose import choose_model
from src.utils.tcp.helper import write_to_stream, read_until_separator, Data


# Write play statistics: array of scores, average score
class Player:
    def __init__(self, model_name: str, client_socket: socket.socket):
        self.client_socket = client_socket
        self.model_name = model_name

    async def play(self, n_turns: int) -> [int]:
        scores = []
        for n_turn in range(n_turns):
            print("=====================================")
            print("Turn " + str(n_turn + 1) + " of " + str(n_turns) + "\n")
            model = choose_model(self.model_name)
            while True:
                try:
                    buffer = read_until_separator(self.client_socket)
                except Exception as e:
                    print("Connection closed")
                    exit(0)
                response = buffer.decode().strip()
                response = json.loads(response)
                response = Data.from_dict(response)

                if response.require_input and "Game Over!" not in response.content:
                    print(response.content + "\n")
                    history = model.get_history()
                    message = {
                        "content": response.content,
                        "role": "user"
                    }
                    history = history + [message]
                    model.set_history(history)
                    prediction = model.chat().strip()
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
                    # self.client_socket.close()
                    scores.append(int(score))
                    if n_turn == n_turns - 1:
                        response = Data.from_dict({
                            "require_input": False,
                            "content": "N"
                        })
                        write_to_stream(self.client_socket, response)
                        return scores
                    else:
                        response = Data.from_dict({
                            "require_input": False,
                            "content": "Y"
                        })
                        write_to_stream(self.client_socket, response)
                        break
                else:
                    print(response.content + "\n")
                    message = {
                        "content": response.content,
                        "role": "user"
                    }
                    history = model.get_history()
                    history = history + [message]
                    model.set_history(history)



