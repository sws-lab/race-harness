from typing import Iterable, List, Optional
from harness.core import StateGraphMessage, Process, StateGraphMessageParticipant, StateGraphEdge, StateGraphMessageEnvelope
from harness.entities.message_destinations import StateGraphProductResponseMessageDestination

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
    def __init__(self, submessages: Iterable[StateGraphMessage]):
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
    def submessages(self) -> Iterable[StateGraphMessage]:
        yield from self._submessages
    
    def __eq__(self, value):
        if not isinstance(value, StateGraphProductMessage):
            return False
        try:
            for own_msg, other_msg in zip(self.submessages, value.submessages, strict=True):
                if own_msg != other_msg:
                    return False
        except ValueError:
            return False
        return True
    
    def __hash__(self):
        res = 0
        for msg in self.submessages:
            res = res * 31 + hash(msg)
        return res

    @staticmethod
    def product_inbound_message_mapping_from(senders: List['Process'], empty_message: StateGraphMessage):
        def construct_product_message(index: int, message: StateGraphProductMessage):
            submessages = list()
            for i in range(len(senders)):
                if index == i:
                    submessages.append(message)
                else:
                    submessages.append(empty_message)
            return StateGraphProductMessage(submessages)
        def mapping(source: StateGraphMessageParticipant, message: StateGraphMessage) -> Optional[StateGraphProductMessage]:
            for index, sender in enumerate(senders):
                if sender == source:
                    return construct_product_message(index, message)
            return None
        return mapping

    @staticmethod
    def product_outbound_message_mapping_to(receivers: List['Process'], empty_message: StateGraphMessage):
        def mapping(edge: StateGraphEdge, envelope: StateGraphMessageEnvelope):
            if not isinstance(envelope.destination, StateGraphProductResponseMessageDestination):
                return None
            
            if edge.trigger is None:
                return None
            
            if not isinstance(edge.trigger, StateGraphProductMessage):
                return None
            
            for receiver, submsg in zip(receivers, edge.trigger.submessages, strict=True):
                if submsg != empty_message:
                    return StateGraphMessageEnvelope(
                        destination=receiver,
                        message=envelope.message
                    )
            return None
        return mapping