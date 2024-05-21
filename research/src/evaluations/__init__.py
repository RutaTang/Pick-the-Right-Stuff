import json
import socket

from src.models.base_model import BaseModel
from src.utils.tcp.helper import write_to_stream, read_until_separator, Data


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
                print("AI: " + prediction + "\n")
                prediction = Data.from_dict({
                    "require_input": False,
                    "content": prediction
                })
                write_to_stream(self.client_socket, prediction)
            elif "Game Over!" in response.content:
                print(response.content + "\n")
            else:
                print(response.content + "\n")
                message = {
                    "content": response.content,
                    "role": "user"
                }
                history = self.model.get_history()
                history = history + [message]
                self.model.set_history(history)
