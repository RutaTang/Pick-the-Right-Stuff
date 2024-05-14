from abc import ABC, abstractmethod
from typing import Dict

class BaseModel(ABC):
    def __init__(self):
        pass

    @abstractmethod
    def chat(self, message):
        pass

    @abstractmethod
    def get_history(self) -> list[Dict[str, str]]:
        pass