from typing import Iterable, Optional
from harness.core import StateGraphMessageDestination, StateGraphMessageParticipant

class StateGraphResponseMessageDestination(StateGraphMessageDestination):
    def __init__(self):
        super().__init__()
    
    @property
    def mnemonic(self) -> str:
        return '%RESPONSE%'
    
    def matches(self, destination: StateGraphMessageParticipant, in_response_to: Optional[StateGraphMessageParticipant]) -> bool:
        return destination == in_response_to
    
class StateGraphResponseGroupDestination(StateGraphMessageDestination):
    def __init__(self, recipients: Iterable[StateGraphResponseMessageDestination]):
        super().__init__()
        self._recipients = list(recipients)
    
    @property
    def mnemonic(self) -> str:
        return '[{}]'.format(', '.join(
            str(recipient)
            for recipient in self._recipients
        ))
    
    def matches(self, destination: StateGraphMessageParticipant, in_response_to: Optional[StateGraphMessageParticipant]) -> bool:
        return any(recipient.matches(destination, in_response_to) for recipient in self._recipients)
