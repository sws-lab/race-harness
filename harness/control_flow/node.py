import abc
from typing import Iterable, Optional
from harness.core import StateGraphEdge, StateGraphNode
from harness.control_flow.mutex import ControlFlowMutex

class ControlFlowNode(abc.ABC):
    def as_statement(self) -> Optional['ControlFlowStatement']:
        return None
    
    def as_sequence(self) -> Optional['ControlFlowSequence']:
        return None
    
    def as_labelled_node(self) -> Optional['ControlFlowLabelledNode']:
        return None
    
    def as_branch(self) -> Optional['ControlFlowBranchNode']:
        return None
    
    def as_goto(self) -> Optional['ControlFlowGotoNode']:
        return None
    
    def as_synchronization(self) -> Optional['ControlFlowSynchronization']:
        return None
    
    def as_init_barrier(self) -> Optional['ControlFlowInitBarrierNode']:
        return None
    
    def canonicalize(self) -> 'ControlFlowNode':
        return self

class ControlFlowStatement(ControlFlowNode):
    def __init__(self, edge: StateGraphEdge):
        super().__init__()
        self._edge = edge

    @property
    def state_graph_edge(self) -> StateGraphEdge:
        return self._edge
    
    def as_statement(self):
        return self
    
class ControlFlowSequence(ControlFlowNode):
    def __init__(self, sequence: Iterable[ControlFlowNode]):
        super().__init__()
        self._sequence = list(sequence)

    @property
    def sequence(self) -> Iterable[ControlFlowNode]:
        yield from self._sequence

    def as_sequence(self):
        return self
    
    def canonicalize(self):
        sequence = list()
        for node in self.sequence:
            canonicalized = node.canonicalize()
            if isinstance(canonicalized, ControlFlowSequence):
                sequence.extend(canonicalized.sequence)
            else:
                sequence.append(canonicalized)
        if len(sequence) == 1:
            return sequence[0]
        else:
            return ControlFlowSequence(sequence)

    def __len__(self) -> int:
        return len(self._sequence)
    
    def __bool__(self) -> bool:
        return bool(self._sequence)
    
    def __getitem__(self, index) -> ControlFlowNode:
        if not isinstance(index):
            raise ValueError(f'Expected control flow sequence index to be an integer')
        if index >= len(self._sequence):
            raise ValueError(f'Control flow sequence index is out of bounds')
        return self._sequence[index]
    
class ControlFlowBranchNode(ControlFlowNode):
    def __init__(self, branches: Iterable[ControlFlowNode]):
        super().__init__()
        self._branches = list(branches)

    @property
    def branches(self) -> Iterable[ControlFlowNode]:
        yield from self._branches

    def as_branch(self):
        return self
    
    def canonicalize(self):
        return ControlFlowBranchNode(
            node.canonicalize()
            for node in self.branches
        )
    
    def __len__(self) -> int:
        return len(self._branches)
    
    def __bool__(self) -> bool:
        return bool(self._branches)
    
class ControlFlowSynchronization(ControlFlowNode):
    def __init__(self, lock: Iterable[ControlFlowMutex], unlock: Iterable[ControlFlowMutex], rollback_on_failure: Optional['ControlFlowLabel'] = None):
        super().__init__()
        self._lock = list(lock)
        self._unlock = list(unlock)
        self._rollback_label = rollback_on_failure

    @property
    def lock(self) -> Iterable[ControlFlowMutex]:
        yield from self._lock

    @property
    def unlock(self) -> Iterable[ControlFlowMutex]:
        yield from self._unlock

    @property
    def rollback_on_failure(self) -> Optional['ControlFlowLabel']:
        return self._rollback_label

    def as_synchronization(self):
        return self
    
class ControlFlowLabel:
    def __init__(self, node: StateGraphNode):
        self._node = node
    
    @property
    def node(self) -> StateGraphNode:
        return self._node
    
    def __str__(self) -> str:
        return self.node.mnemonic
    
    def __eq__(self, value) -> bool:
        return isinstance(value, ControlFlowLabel) and self.node == value.node
    
    def __ne__(self, value) -> bool:
        return not self.__eq__(value)
    
    def __hash__(self) -> int:
        return hash(self.node)
    
class ControlFlowLabelledNode(ControlFlowNode):
    def __init__(self, label: ControlFlowLabel, body: ControlFlowNode):
        super().__init__()
        self._label = label
        self._body = body

    @property
    def label(self) -> ControlFlowLabel:
        return self._label
    
    @property
    def body(self) -> ControlFlowNode:
        return self._body

    def as_labelled_node(self):
        return self
    
    def canonicalize(self):
        return ControlFlowLabelledNode(
            self.label,
            self.body.canonicalize()
        )
    
class ControlFlowGotoNode(ControlFlowNode):
    def __init__(self, label: ControlFlowLabel):
        super().__init__()
        self._label = label

    @property
    def label(self) -> ControlFlowLabel:
        return self._label
    
    def as_goto(self):
        return self
    
class ControlFlowInitBarrierNode(ControlFlowNode):
    def __init__(self):
        super().__init__()
    
    def as_init_barrier(self):
        return self