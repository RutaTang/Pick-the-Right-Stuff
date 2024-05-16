import socket

from dotenv import load_dotenv, find_dotenv

from src.evaluations import Player
from src.models.openai_model import OpenAIModel

# TODO: test LLM's ability for this game play
# - [x] implement tcp client
# - [] let LLM play the game

load_dotenv(find_dotenv())

if __name__ == "__main__":
    client_socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    client_socket.connect(('127.0.0.1', 8080))
    model = OpenAIModel()
    player = Player(client_socket, model)
    player.play()
