from abc import ABC, abstractmethod
from typing import Dict


class BaseModel(ABC):
    def __init__(self):
        pass

    @abstractmethod
    def chat(self) -> str:
        pass

    @abstractmethod
    def get_history(self) -> list[Dict[str, str]]:
        pass

    @abstractmethod
    def set_history(self, history: list[Dict[str, str]]):
        pass

    @abstractmethod
    def reconfig(self, config: Dict[str, any]):
        pass

