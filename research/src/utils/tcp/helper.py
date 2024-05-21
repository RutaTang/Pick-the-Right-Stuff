import json


class Data:
    def __init__(self, require_input: bool, content: str):
        self.require_input = require_input
        self.content = content

    def to_dict(self):
        return {
            "require_input": self.require_input,
            "content": self.content
        }

    @staticmethod
    def from_dict(data: dict):
        return Data(data["require_input"], data["content"])


def read_until_separator(client_socket):
    SEPARATOR = 0x0a
    content_buffer = bytearray()

    while True:
        chunk = client_socket.recv(1)
        if len(chunk) == 0:
            raise Exception("Stream closed")

        if chunk[0] == SEPARATOR:
            break
        else:
            content_buffer.append(chunk[0])

    return content_buffer


def write_to_stream(client_socket, data: Data):
    data = data.to_dict()
    data = json.dumps(data).encode()
    data += bytes([0x0a])
    client_socket.sendall(data)
