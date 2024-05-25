import asyncio
import socket

from dotenv import load_dotenv, find_dotenv

from src.evaluations import Player, evaluate
from src.models.openai_model import OpenAIModel

load_dotenv(find_dotenv())


async def main():
    score = await evaluate(50, "127.0.0.1", 8080)
    print("Final average score:", score)


if __name__ == "__main__":
    asyncio.run(main())
