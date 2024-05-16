import socket

from src.models.base_model import BaseModel
from src.utils.tcp.helper import write_to_stream, read_until_separator
from src.models.openai_model import OpenAIModel


class Player:
    def __init__(self, client_socket: socket.socket, model: BaseModel):
        self.client_socket = client_socket
        self.model = model

    def play(self):
        while True:
            try:
                buffer = read_until_separator(self.client_socket)
            except Exception as e:
                print("Connection closed")
                exit(0)
            response = buffer.decode().strip()

            if "[user input]" in response:
                response = response.replace("[user input]", "")
                history = self.model.get_history()
                message = {
                    "content": response,
                    "role": "user"
                }
                history = history + [message]
                self.model.set_history(history)
                prediction = self.model.chat().strip()
                print("AI: " + prediction + "\n")
                write_to_stream(self.client_socket, prediction, True)
            elif "Game Over!" in response:
                print(response + "\n")
            else:
                message = {
                    "content": response,
                    "role": "user"
                }
                history = self.model.get_history()
                history = history + [message]
                self.model.set_history(history)
