import asyncio
import socket
import numpy as np

from dotenv import load_dotenv, find_dotenv

from src.evaluations import Player
from src.models.openai_model import OpenAIModel

load_dotenv(find_dotenv())


async def main():
    client_socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    client_socket.connect(('127.0.0.1', 8080))
    player = Player("gemma", client_socket)
    scores = await player.play(n_turns=2)
    print(scores)
    print(np.sum(scores) / 2)


if __name__ == "__main__":
    asyncio.run(main())
