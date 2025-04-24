from typing import Iterable, List, Optional
from harness.core import StateGraphMessage, Process, StateGraphMessageParticipant

class StateGraphSimpleMessage(StateGraphMessage):
    def __init__(self, mnemonic: str):
        self._mnemonic = mnemonic

    @property
    def mnemonic(self) -> str:
        return self._mnemonic
    
    def __eq__(self, value):
        return isinstance(value, StateGraphSimpleMessage) and value.mnemonic == self.mnemonic
    
    def __hash__(self):
        return hash(self.mnemonic)
    
class StateGraphProductMessage(StateGraphMessage):
    def __init__(self, submessages: Iterable[Optional[StateGraphMessage]]):
        super().__init__()
        self._submessages = list(submessages)

    def add_submessage(self, message: StateGraphMessage) -> 'StateGraphProductMessage':
        self._submessages.append(message)
        return self

    @property
    def mnemonic(self) -> str:
        return '({})'.format(', '.join(
            str(msg)
            for msg in self._submessages
        ))
    
    @property
    def submessages(self) -> List[StateGraphMessage]:
        return self._submessages.copy()
    
    def __eq__(self, value):
        if not isinstance(value, StateGraphProductMessage):
            return False
        if len(self._submessages) != len(value.submessages):
            return False
        for own_msg, other_msg in zip(self.submessages, value.submessages):
            if own_msg != other_msg:
                return False
        return True
    
    def __hash__(self):
        res = 0
        for msg in self.submessages:
            res = res * 31 + hash(msg)
        return res

    @staticmethod
    def product_message_mapping_from(senders: List['Process']):
        def construct_product_message(index: int, message: StateGraphProductMessage):
            submessages = list()
            for i in range(len(senders)):
                if index == i:
                    submessages.append(message)
                else:
                    submessages.append(None)
            return StateGraphProductMessage(submessages)
        def mapping(source: StateGraphMessageParticipant, message: StateGraphMessage) -> Optional[StateGraphProductMessage]:
            for index, sender in enumerate(senders):
                if sender == source:
                    return construct_product_message(index, message)
            return None
        return mapping
