import asyncio
import os.path
import socket
import numpy as np

from dotenv import load_dotenv, find_dotenv

from src.evaluations import Player
from src.models.openai_model import OpenAIModel

load_dotenv(find_dotenv())


async def main():
    # =====para=======
    mode = "zero"  # "zero" or "finite"
    model = "gemma"

    # ======run======
    port = 8080 if mode == "zero" else 8081
    client_socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    client_socket.connect(('127.0.0.1', port))
    turns = 1
    player = Player(model, client_socket)
    scores = await player.play(n_turns=turns)

    # ======save result======
    print(scores)
    mean_scores = np.mean(scores)
    print(f"Mean Score: {mean_scores}")
    if os.path.exists("results") is False:
        os.makedirs("results")
    with open(f"results/{model}.txt", "w") as f:
        f.write(f"{mean_scores}\n")


if __name__ == "__main__":
    asyncio.run(main())

# Print Turns
# Test Phi-3-14B, GPT3.5
