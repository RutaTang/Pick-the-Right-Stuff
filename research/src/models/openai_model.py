import copy
import os
from typing import Dict
from openai import OpenAI
from openai.types.chat import ChatCompletionMessage
from dotenv import load_dotenv, find_dotenv
import unittest
if __name__ == "__main__":
    from base_model import BaseModel
else:
    from .base_model import BaseModel


class OpenAIModel(BaseModel):
    def __init__(self):
        self.client = OpenAI(api_key=os.getenv("OPENAI_KEY"))
        self.model = "gpt-3.5-turbo"
        self.historical_messages: list[Dict[str,str]] = []

    def chat(self, message: str) -> ChatCompletionMessage:
        message: ChatCompletionMessage = {
            "role": "user",
            "content": message,
        }
        self.historical_messages.append(message)
        chat_completion = self.client.chat.completions.create(
            messages=self.historical_messages + [message],
            model=self.model,
            max_tokens=10,
        )
        message = chat_completion.choices[0].message
        message = {
            "role": "assistant",
            "content": message.content,
        }
        self.historical_messages.append(message)
        return message

    def get_history(self) -> list[Dict[str, str]]:
        return copy.deepcopy(self.historical_messages)


class TestOpenAIModel(unittest.TestCase):
    def test_chat(self):
        load_dotenv(find_dotenv())
        llm = OpenAIModel()
        message = llm.chat("hello")
        self.assertTrue(len(message["content"]) > 0)
        self.assertTrue(len(llm.get_history()) == 2)
        print(llm.get_history())


if __name__ == "__main__":
    unittest.main()
