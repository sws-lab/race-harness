from typing import Iterable
from harness.core import StateGraphAction, StateGraphMessageEnvelope, StateGraphMessage, StateGraphMessageDestination

class StateGraphSimpleAction(StateGraphAction):
    def __init__(self, mnemonic: str):
        super().__init__(mnemonic=mnemonic)
        self._envelopes = list()

    @property
    def message_envelopes(self) -> Iterable[StateGraphMessageEnvelope]:
        return self._envelopes
    
    def add_envelope(self, destination: StateGraphMessageDestination, message: StateGraphMessage) -> 'StateGraphSimpleAction':
        self._envelopes.append(StateGraphMessageEnvelope(destination=destination, message=message))
        return self
