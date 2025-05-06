from typing import Iterable, Optional, Callable
from harness.core import StateGraphMessageDestination, StateGraphMessageParticipant, HarnessError
    
class StateGraphGroupMessageDestination(StateGraphMessageDestination):
    def __init__(self, recipients: Iterable[StateGraphMessageDestination]):
        super().__init__()
        self._recipients = list(recipients)
    
    @property
    def mnemonic(self) -> str:
        return '[{}]'.format(', '.join(
            str(recipient)
            for recipient in self._recipients
        ))
    
    def matches(self, destination: StateGraphMessageParticipant) -> bool:
        return any(recipient.matches(destination) for recipient in self._recipients)

class StateGraphProductResponseMessageDestination(StateGraphMessageDestination):
    def __init__(self):
        super().__init__()
    
    @property
    def mnemonic(self) -> str:
        return '%PRODUCT_RESPONSE%'
    
    def matches(self, destination: StateGraphMessageParticipant) -> bool:
        raise HarnessError(f'{self.__class__}.matches method cannot be called directly')
