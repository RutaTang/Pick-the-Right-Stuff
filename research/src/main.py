import asyncio
import os.path
import socket
import time

import numpy as np

from dotenv import load_dotenv, find_dotenv

from src.evaluations import Player
from src.models.openai_model import OpenAIModel

load_dotenv(find_dotenv())


async def main():
    # =====para=======
    mode = "zero"  # "zero" or "finite"
    model = "gpt-3.5-turbo"
    turns = 60

    # ======run======
    port = 8080 if mode == "zero" else 8081
    client_socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    client_socket.connect(('127.0.0.1', port))
    player = Player(model, client_socket)
    start_time = time.time()
    scores = await player.play(n_turns=turns)
    end_time = time.time()
    print("Run in", end_time - start_time, "seconds.")

    # ======save result======
    print(scores)
    mean_scores = np.mean(scores)
    print(f"Mean Score: {mean_scores}")
    if os.path.exists("results") is False:
        os.makedirs("results")
    with open(f"results/{mode}_{model}_{turns}.txt", "w") as f:
        f.write(f"{mean_scores}\n")


if __name__ == "__main__":
    asyncio.run(main())
