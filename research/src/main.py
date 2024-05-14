from dotenv import load_dotenv, find_dotenv
from utils.tcp import tcp_client

# TODO: test LLM's ability for this game play
# - [] implement tcp client
# - [] let LLM plat the game

load_dotenv(find_dotenv())

if __name__ == "__main__":
    tcp_client()
