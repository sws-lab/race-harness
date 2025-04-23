from typing import Optional, Iterable, List
from harness.state_graph import StateGraphNode, StateGraphMessage, StateGraphAction, StateGraphEdge, StateGraphMessageEnvelope

class StateGraphSimpleNode(StateGraphNode):
    def __init__(self, mnemonic: str):
        super().__init__(mnemonic=mnemonic)
        self._edges = dict()

    def add_edge(self, trigger: Optional[StateGraphMessage], target: StateGraphNode, action: StateGraphAction) -> 'StateGraphSimpleNode':
        edge = StateGraphEdge(source=self, target=target, trigger=trigger, action=action)
        self._edges[(target, trigger)] = edge
        return self

    @property
    def edges(self) -> Iterable[StateGraphEdge]:
        return self._edges.values()
    
class StateGraphSimpleAction(StateGraphAction):
    def __init__(self, mnemonic: str):
        super().__init__(mnemonic=mnemonic)
        self._envelopes = list()

    @property
    def message_envelopes(self) -> Iterable[StateGraphMessageEnvelope]:
        return self._envelopes
    
    def add_envelope(self, envelope: StateGraphMessageEnvelope) -> 'StateGraphSimpleAction':
        self._envelopes.append(envelope)
        return self
    
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
    
class StateGraphProductNode(StateGraphNode):
    def __init__(self):
        super().__init__(mnemonic='()')
        self._subnodes = list()
        self._edges = None

    def add_subnode(self, subnode: StateGraphNode) -> 'StateGraphProductNode':
        self._subnodes.append(subnode)
        self._edges = None
        return self

    @property
    def mnemonic(self) -> str:
        return '({})'.format(', '.join(
            f'{subnode.mnemonic}'
            for subnode in self.subnodes
        ))
    
    @property
    def subnodes(self) -> Iterable[StateGraphNode]:
        return self._subnodes
    
    @property
    def edges(self) -> Iterable[StateGraphEdge]:
        if self._edges is None:
            self._edges = set(self._compute_edges())
        return self._edges
    
    def _compute_edges(self):
        for index, subnode in enumerate(self._subnodes):
            for edge in subnode.edges:
                product_target_node = StateGraphProductNode()
                product_message = StateGraphProductMessage([])
                for other_index, other_subnode in enumerate(self.subnodes):
                    if index == other_index:
                        product_target_node.add_subnode(edge.target)
                        product_message.add_submessage(edge.trigger)
                    else:
                        product_target_node.add_subnode(other_subnode)
                        product_message.add_submessage(None)
                if all(submsg is None for submsg in product_message.submessages):
                    product_message = None
                product_edge = StateGraphEdge(source=self, target=product_target_node, trigger=product_message, action=edge.action)
                yield product_edge

class StateGraphDerivedNode(StateGraphNode):
    def __init__(self, mnemonic_prefix: str, base: StateGraphNode):
        super().__init__(mnemonic=f'{mnemonic_prefix} {base.mnemonic}')
        self._mnemonic_prefix = mnemonic_prefix
        self._base = base
        self._edges = list()

    def add_edge(self, match_base: Optional[StateGraphNode], trigger: Optional[StateGraphMessage], target: StateGraphNode, action: StateGraphAction) -> 'StateGraphDerivedNode':
        self._edges.append((match_base, StateGraphEdge(source=self, target=target, trigger=trigger, action=action)))
        return self
    
    @property
    def edges(self) -> Iterable[StateGraphEdge]:
        for match_base, edge in self._edges:
            if match_base is None or self._base == match_base:
                yield edge
        for edge in self._base.edges:
            target_node = StateGraphDerivedNode(mnemonic_prefix=self._mnemonic_prefix, base=edge.target)
            for match_base, other_edge in self._edges:
                target_node.add_edge(match_base=match_base, trigger=other_edge.trigger, target=other_edge.target, action=other_edge.action)
            yield StateGraphEdge(source=self, target=target_node, trigger=edge.trigger, action=edge.action)
