from typing import Callable, Optional
from harness.state_graph import StateGraphNode, StateGraphMessageParticipant, StateGraphMessage, StateGraphMessageDestination

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
