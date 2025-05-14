from typing import Iterable
from harness.core import StateGraphNode, StateGraphEdge
from harness.control_flow.node import ControlFlowNode, ControlFlowLabel, ControlFlowBranchNode, ControlFlowStatement, ControlFlowSequence, ControlFlowLabelledNode, ControlFlowGotoNode

class ControlFlowBuilder:
    def __init__(self, root: StateGraphNode):
        self._root = root
        self._control_flow_node = None
        self._reverse_edges = dict()
        self._backward_edges = set()
        self._labelled_states = dict()
        self._scan()

    @property
    def state_graph_root(self) -> StateGraphNode:
        return self._root
    
    @property
    def control_flow(self) -> ControlFlowNode:
        if self._control_flow_node is None:
            self._control_flow_node = self._generate_control_flow_node(self.state_graph_root)
        return self._control_flow_node
    
    def _scan(self):
        self._scan_reverse_edges()
        self._scan_backward_edges()

    def _scan_reverse_edges(self):
        for state in self.state_graph_root.all_nodes:
            for edge in state.edges:
                if edge.target not in self._reverse_edges:
                    self._reverse_edges[edge.target] = list()
                self._reverse_edges[edge.target].append(edge)

    def _scan_backward_edges(self):
        visited = set()
        queue = [(self._root, list())]
        while queue:
            current_state, current_path = queue.pop()
            if current_state in visited:
                continue
            visited.add(current_state)
            new_path = [*current_path, current_state]
            for edge in current_state.edges:
                if edge.target in new_path:
                    self._backward_edges.add(edge)
                    if edge.target not in self._labelled_states:
                        self._labelled_states[edge.target] = ControlFlowLabel(edge.target.mnemonic)
                else:
                    queue.append((edge.target, new_path))

    def _generate_control_flow_node(self, state: StateGraphNode) -> ControlFlowNode:        
        def generate_edge(edge: StateGraphEdge) -> ControlFlowNode:
            if edge in self._backward_edges:
                return ControlFlowSequence(
                    [
                        ControlFlowStatement(edge),
                        ControlFlowGotoNode(self._labelled_states[edge.target])
                    ]
                )
            else:
                return ControlFlowSequence(
                    [
                        ControlFlowStatement(edge),
                        self._generate_control_flow_node(edge.target)
                    ]
                )
        
        edge_nodes = list()
        for edge in state.edges:
            edge_nodes.append(generate_edge(edge))

        if len(edge_nodes) > 1:
            node = ControlFlowBranchNode(edge_nodes)
        elif len(edge_nodes) == 1:
            node = edge_nodes[0]
        else:
            node = ControlFlowSequence(())

        node_label = self._labelled_states.get(state, None)
        if node_label is None:
            return node
        else:
            return ControlFlowLabelledNode(
                node_label,
                node
            )
