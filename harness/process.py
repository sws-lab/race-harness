from typing import Callable, List, Optional
from harness.state_graph import StateGraphNode, StateGraphMessageParticipant, StateGraphMessage, StateGraphMessageDestination
from harness.graph_nodes import StateGraphProductMessage

class Process(StateGraphMessageParticipant, StateGraphMessageDestination):
    def __init__(self, mnemonic: str, entry_node: StateGraphNode):
        super().__init__()
        self._mnemonic = mnemonic
        self._entry_node = entry_node
        self._message_mappers = list()

    @property
    def mnemonic(self) -> str:
        return self._mnemonic
    
    def matches(self, destination: StateGraphMessageParticipant, in_response_to: Optional[StateGraphMessageParticipant]):
        return self == destination
    
    @property
    def entry_node(self) -> StateGraphNode:
        return self._entry_node
    
    def add_message_mapping(self, mapping: Callable[[StateGraphMessageParticipant, StateGraphMessage], StateGraphMessage]):
        self._message_mappers.append(mapping)

    def map_message(self, source: StateGraphMessageParticipant, message: StateGraphMessage) -> StateGraphMessage:
        for mapping in self._message_mappers:
            mapped_msg = mapping(source, message)
            if mapped_msg is not None:
                return mapped_msg
        return message
    
    def __str__(self) -> str:
        return self.mnemonic
    
    def __eq__(self, value) -> bool:
        return isinstance(value, Process) and value.mnemonic == self.mnemonic
    
    def __ne__(self, value) -> bool:
        return not self.__eq__(value)
    
    def __hash__(self) -> int:
        return hash(self.mnemonic)

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

