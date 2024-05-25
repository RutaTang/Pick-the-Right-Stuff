from unittest import TestCase

from src.models.ollama_model import OllamaModel


class TestOllama(TestCase):
    def test_predict(self):
        model = OllamaModel()
        model.set_history([
            {
                "role": "user",
                "content": "Hello"
            }
        ])
        content = model.chat()
        print(content)
        self.assertTrue(len(content) > 0)