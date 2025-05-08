import abc
import dataclasses
from typing import Iterable, Optional

class StateGraphMessage(abc.ABC):
    @property
    @abc.abstractmethod
    def mnemonic(self) -> str: pass

    def __str__(self):
        return self.mnemonic
    
    def __repr__(self):
        return str(self)
    
    @abc.abstractmethod
    def __eq__(self, value): pass
    
    def __ne__(self, value):
        return not self.__eq__(value)
    
    @abc.abstractmethod
    def __hash__(self): pass

class StateGraphMessageParticipant(abc.ABC):
    @property
    @abc.abstractmethod
    def mnemonic(self) -> str: pass

    def __eq__(self, value):
        return isinstance(value, StateGraphMessageParticipant) and value.mnemonic == self.mnemonic

    def __ne__(self, value):
        return not self.__eq__(value)

    def __hash__(self):
        return hash(self.mnemonic)
    
class StateGraphMessageDestination(abc.ABC):
    @property
    @abc.abstractmethod
    def mnemonic(self) -> str: pass

    def matches(self, destination: StateGraphMessageParticipant) -> bool:  pass

    def __str__(self):
        return self.mnemonic

@dataclasses.dataclass(frozen=True)
class StateGraphMessageEnvelope:
    destination: StateGraphMessageDestination
    message: StateGraphMessage

    def __str__(self):
        return f'[{self.destination}: {self.message}]'
    
class StateGraphAction(abc.ABC):
    def __init__(self, mnemonic: str):
        super().__init__()
        self._mnemonic = mnemonic

    @property
    def mnemonic(self) -> str:
        return self._mnemonic

    @property
    @abc.abstractmethod
    def message_envelopes(self) -> Iterable[StateGraphMessageEnvelope]: pass

    def __str__(self):
        return self.mnemonic
    
    def __eq__(self, value):
        return isinstance(value, StateGraphAction) and value.mnemonic == self.mnemonic
    
    def __ne__(self, value):
        return not self.__eq__(value)
    
    def __hash__(self):
        return hash(self.mnemonic)

@dataclasses.dataclass
class StateGraphEdge:
    source: 'StateGraphNode'
    target: 'StateGraphNode'
    trigger: Optional[StateGraphMessage]
    action: StateGraphAction

    def __str__(self):
        if self.trigger is not None:
            return f'({self.source} -> {self.target} on {self.trigger})'
        else:
            return f'({self.source} -> {self.target})'
        
    def __repr__(self):
        return str(self)

    def __eq__(self, value):
        return isinstance(value, StateGraphEdge) and \
            value.source == self.source and \
            value.target == self.target and \
            value.trigger == self.trigger
    
    def __ne__(self, value):
        return not self.__eq__(value)
    
    def __hash__(self):
        if not hasattr(self, '_cache_hash'):
            setattr(self, '_cache_hash', hash(self.source) * 31 + hash(self.target) * 17 + hash(self.trigger))
        return self._cache_hash

class StateGraphNode(abc.ABC):
    def __init__(self, mnemonic: str, is_placeholder: bool = False):
        super().__init__()
        self._mnemonic = mnemonic
        self._is_placeholder = is_placeholder
    
    @property
    def mnemonic(self) -> str:
        return self._mnemonic
    
    @property
    def is_placeholder(self) -> bool:
        return self._is_placeholder

    @property
    @abc.abstractmethod
    def edges(self) -> Iterable[StateGraphEdge]: pass

    @property
    def all_nodes(self) -> Iterable['StateGraphNode']:
        visited = set()
        pending = [self]
        while len(pending) > 0:
            node = pending.pop()
            if node not in visited:
                yield node
                visited.add(node)
                for edge in node.edges:
                    pending.append(edge.target)

    def matches(self, value: 'StateGraphNode'):
        return self.is_placeholder or value.is_placeholder or self == value

    def __str__(self):
        return self.mnemonic

    def __eq__(self, value):
        return isinstance(value, StateGraphNode) and self.mnemonic == value.mnemonic
    
    def __ne__(self, value):
        return not self.__eq__(value)
    
    def __hash__(self):
        return hash(self.mnemonic)
    
    def __repr__(self):
        return str(self)
