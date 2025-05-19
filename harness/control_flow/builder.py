from typing import Set
from harness.core import StateGraphNode, StateGraphEdge, Process
from harness.control_flow.node import ControlFlowNode, ControlFlowLabel, ControlFlowBranchNode, ControlFlowStatement, ControlFlowSequence, ControlFlowLabelledNode, ControlFlowGotoNode, ControlFlowSynchronization, ControlFlowInitBarrierNode
from harness.control_flow.mutex import ControlFlowMutexSet

class ControlFlowBuilder:
    def __init__(self, root: StateGraphNode):
        self._root = root
        self._reverse_edges = dict()
        self._backward_edges = set()
        self._labelled_states = dict()
        self._scan()

    @property
    def state_graph_root(self) -> StateGraphNode:
        return self._root
    
    def control_flow(self, process: Process, mutex_set: ControlFlowMutexSet) -> ControlFlowNode:
        node = self._generate_control_flow_node(process, mutex_set, self.state_graph_root, set())
        locked_mutexes = list(mutex_set.locked_in_state(process, self.state_graph_root))
        if locked_mutexes:
            return ControlFlowSequence([
                ControlFlowSynchronization(locked_mutexes, ()),
                ControlFlowInitBarrierNode(),
                node
            ])
        else:
            return node
    
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
            if current_state not in self._labelled_states:
                self._labelled_states[current_state] = ControlFlowLabel(current_state)
            new_path = [*current_path, current_state]
            for edge in current_state.edges:
                if edge.target in new_path:
                    self._backward_edges.add(edge)
                else:
                    queue.append((edge.target, new_path))

    def _generate_control_flow_node(self, process: Process, mutex_set: ControlFlowMutexSet, state: StateGraphNode, generated_states: Set[StateGraphNode]) -> ControlFlowNode:        
        def generate_edge(edge: StateGraphEdge) -> ControlFlowNode:
            if edge in self._backward_edges or edge.target in generated_states:
                sequence = [
                    ControlFlowStatement(edge),
                    ControlFlowGotoNode(self._labelled_states[edge.target])
                ]
            else:
                sequence = [
                    ControlFlowStatement(edge),
                    self._generate_control_flow_node(process, mutex_set, edge.target, generated_states)
                ]
            
            locked_mutexes = set(mutex_set.locked_in_state(process, edge.target))
            unlocked_mutexes = set(mutex_set.locked_in_state(process, edge.source))

            hold_mutexes = locked_mutexes.intersection(unlocked_mutexes)
            unlocked_mutexes.difference_update(hold_mutexes)
            locked_mutexes.difference_update(hold_mutexes)
            if locked_mutexes or unlocked_mutexes:
                sequence.insert(0, ControlFlowSynchronization(locked_mutexes, unlocked_mutexes, rollback_on_failure=self._labelled_states[edge.source]))
            return ControlFlowSequence(sequence)
        
        generated_states.add(state)

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
