import copy
import os
from typing import Dict
from openai import OpenAI

if __name__ == "__main__":
    from base_model import BaseModel
else:
    from .base_model import BaseModel


class OpenAIModel(BaseModel):

    def __init__(self):
        super().__init__()
        self.client = OpenAI(api_key=os.getenv("OPENAI_KEY"))
        self.config = {
            "model": "gpt-3.5-turbo",
            "temperature": 0,
            "max_tokens": 20,
        }
        self.historical_messages: list[Dict[str, str]] = []

    def reconfig(self, config: Dict[str, any]):
        self.config.update(config)

    def chat(self) -> str:
        chat_completion = self.client.chat.completions.create(
            messages=self.historical_messages,
            model=self.config["model"],
            max_tokens=self.config["max_tokens"],
            temperature=self.config["temperature"],
        )
        chat_message = chat_completion.choices[0].message
        message = {
            "role": "assistant",
            "content": chat_message.content,
        }
        self.historical_messages.append(message)
        return message["content"]

    def get_history(self) -> list[Dict[str, str]]:
        return copy.deepcopy(self.historical_messages)

    def set_history(self, history: list[Dict[str, str]]):
        self.historical_messages = history
