import copy
from typing import Dict
import ollama
from ollama import Client

if __name__ == "__main__":
    from base_model import BaseModel
else:
    from .base_model import BaseModel


class OllamaModel(BaseModel):

    def __init__(self):
        super().__init__()
        self.client = Client()
        self.config = {
            "model": "llama3:70b-instruct",
            "temperature": 0,
            "max_tokens": 20,
        }
        self.historical_messages: list[Dict[str, str]] = []

    def chat(self) -> str:
        chat_completion = self.client.chat(
            messages=self.historical_messages,
            model=self.config["model"],
            options= {
                "temperature": self.config["temperature"],
                "num_predict": self.config["max_tokens"]
            }
        )
        chat_message = chat_completion["message"]["content"]
        message = {
            "role": "assistant",
            "content": chat_message,
        }
        self.historical_messages.append(message)
        return message["content"]

    def get_history(self) -> list[Dict[str, str]]:
        return copy.deepcopy(self.historical_messages)

    def set_history(self, history: list[Dict[str, str]]):
        self.historical_messages = history

    def reconfig(self, config: Dict[str, any]):
        self.config.update(config)

