from typing import Optional, Iterable, List
from harness.core import StateGraphEdge, StateGraphMessage, StateGraphNode, StateGraphAction
from harness.entities.messages import StateGraphProductMessage

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

class StateGraphProductNode(StateGraphNode):
    def __init__(self, subnodes: Iterable[StateGraphNode], empty_message: StateGraphMessage):
        super().__init__(mnemonic='()')
        self._subnodes = list(subnodes)
        self._edges = None
        self._empty_message = empty_message

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
    def empty_message(self) -> StateGraphMessage:
        return self._empty_message
    
    @property
    def edges(self) -> Iterable[StateGraphEdge]:
        if self._edges is None:
            self._edges = list(self._compute_edges())
        return self._edges
    
    def _compute_edges(self):
        for index, subnode in enumerate(self._subnodes):
            for edge in subnode.edges:
                product_target_node = StateGraphProductNode((), self._empty_message)
                product_message = StateGraphProductMessage([])
                for other_index, other_subnode in enumerate(self.subnodes):
                    if index == other_index:
                        product_target_node.add_subnode(edge.target)
                        product_message.add_submessage(edge.trigger)
                    else:
                        product_target_node.add_subnode(other_subnode)
                        product_message.add_submessage(self._empty_message)
                if all(submsg is self._empty_message for submsg in product_message.submessages):
                    product_message = None
                product_edge = StateGraphEdge(source=self, target=product_target_node, trigger=product_message, action=edge.action)
                yield product_edge

class StateGraphDerivedNode(StateGraphNode):
    def __init__(self, mnemonic_prefix: str, base: StateGraphNode):
        super().__init__(mnemonic=f'{mnemonic_prefix} {base.mnemonic}')
        self._mnemonic_prefix = mnemonic_prefix
        self._base = base
        self._edges = list()

    @property
    def mnemonic_prefix(self) -> str:
        return self._mnemonic_prefix

    @property
    def base(self) -> StateGraphNode:
        return self._base

    def add_edge(self, match_base: Optional[StateGraphNode], trigger: Optional[StateGraphMessage], target: StateGraphNode, action: StateGraphAction) -> 'StateGraphDerivedNode':
        self._edges.append((match_base, StateGraphEdge(source=self, target=target, trigger=trigger, action=action)))
        return self
    
    def rebase(self, new_base: StateGraphNode) -> 'StateGraphDerivedNode':
        node = StateGraphDerivedNode(mnemonic_prefix=self.mnemonic_prefix, base=new_base)
        for match_base, other_edge in self._edges:
            node.add_edge(match_base=match_base, trigger=other_edge.trigger, target=other_edge.target, action=other_edge.action)
        return node
    
    @property
    def edges(self) -> Iterable[StateGraphEdge]:
        for match_base, edge in self._edges:
            if match_base is None or self._base == match_base:
                yield edge
        for edge in self._base.edges:
            target_node = self.rebase(edge.target)
            yield StateGraphEdge(source=self, target=target_node, trigger=edge.trigger, action=edge.action)

class StateGraphPlaceholderNode(StateGraphNode):
    def __init__(self):
        super().__init__(mnemonic='?', is_placeholder=True)

    def edges(self) -> Iterable[StateGraphEdge]:
        yield from ()

    def __eq__(self, value):
        return isinstance(value, StateGraphPlaceholderNode)
