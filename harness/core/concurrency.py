from typing import Iterable, Tuple, Callable, Collection, Set, Optional
from harness.core.process import Process
from harness.core.process_set import ProcessSetStateSpace
from harness.core.state_graph import StateGraphEdge

class ProcessConcurrency:
    def __init__(self, concurrent_pairs: Iterable[Tuple[Process, StateGraphEdge, Process, StateGraphEdge]]):
        self._concurrency_index = dict()
        self._concurrent_groups = None
        self._build_index(concurrent_pairs)
    
    @property
    def process_edges(self) -> Iterable[Tuple[Process, StateGraphEdge]]:
        yield from self._concurrency_index.keys()
    
    @property
    def concurrent_groups(self) -> Iterable[Set[Tuple[Process, StateGraphEdge]]]:
        if self._concurrent_groups is None:
            self._aggregate_concurrent_groups()
        yield from self._concurrent_groups

    def is_concurrent(self, process: Process, edge: StateGraphEdge, other_process: Process, other_edge: StateGraphEdge) -> bool:
        if (process, edge) in self._concurrency_index:
            if other_edge in self._concurrency_index[(process, edge)].get(other_process, ()):
                return True
        if (other_process, other_edge) in self._concurrency_index:
            if edge in self._concurrency_index[(other_process, other_edge)].get(process, ()):
                return True
        return False
    
    def concurrent_process_edges(self, process: Process, edge: StateGraphEdge) -> Iterable[Tuple[Process, StateGraphEdge]]:
        if (process, edge) in self._concurrency_index:
            for other_process, other_edges in self._concurrency_index[(process, edge)].items():
                for other_edge in other_edges:
                    yield (other_process, other_edge)

    def _build_index(self, concurrent_pairs: Iterable[Tuple[Process, StateGraphEdge, Process, StateGraphEdge]]):
        def add_pair(process1: Process, edge1: StateGraphEdge, process2: Process, edge2: StateGraphEdge):
            if (process1, edge1) not in self._concurrency_index:
                self._concurrency_index[(process1, edge1)] = dict()
            if process2 not in self._concurrency_index[(process1, edge1)]:
                self._concurrency_index[(process1, edge1)][process2] = list()
            self._concurrency_index[(process1, edge1)][process2].append(edge2)

        for process1, edge1, process2, edge2 in concurrent_pairs:
            add_pair(process1, edge1, process2, edge2)
            add_pair(process2, edge2, process1, edge1)

    def _find_max_clique_for(self, process: Process, edge: StateGraphEdge, concurrent_edges: Iterable[Tuple[Process, StateGraphEdge]]) -> Set[Tuple[Process, StateGraphEdge]]:
        clique = {
            (process, edge)
        }
        clique_processes = {
            process
        }
        for candiate_process, candidate_edge in concurrent_edges:
            if (candiate_process, candidate_edge) not in clique and \
                candiate_process not in clique_processes and \
                all(self.is_concurrent(candiate_process, candidate_edge, clique_process, clique_edge) 
                    for clique_process, clique_edge in clique):
                clique.add((candiate_process, candidate_edge))
                clique_processes.add(candiate_process)
        return clique

    def _aggregate_concurrent_groups(self):       
        self._concurrent_groups = list()
        for (process1, edge1), concurrent_process_edges in self._concurrency_index.items():
            other_concurrent_edges = [
                (process2, edge2)
                for process2, edges in concurrent_process_edges.items()
                for edge2 in edges
            ]
            for process2, edge2 in other_concurrent_edges:
                concurrent_group = self._find_max_clique_for(process2, edge2, other_concurrent_edges)
                concurrent_group.add((process1, edge1))
                self._concurrent_groups.append(concurrent_group)

    @staticmethod
    def extract_concurrent_transition_pairs(state_space: ProcessSetStateSpace) -> Iterable[Tuple[Process, StateGraphEdge, Process, StateGraphEdge]]:
        visited = set()
        for process in state_space.process_set.processes:
            for state in process.entry_node.all_nodes:
                psstates = set(state_space.match_states(process, state))
                concurrent_transitions = dict()
                for psstate in psstates:
                    for transition_process, transition_edge, _ in psstate.next_transitions:
                        if transition_process == process:
                            concurrent_transitions[transition_edge] = state_space.match_states(process, transition_edge.target)
                for own_edge, psstates in concurrent_transitions.items():
                    for psstate in psstates:
                        for transition_process, transition_edge, _ in psstate.next_transitions:
                            if transition_process != process:
                                key = (process, own_edge, transition_process, transition_edge)
                                reverse_key = (transition_process, transition_edge, process, own_edge)
                                if key not in visited:
                                    visited.add(key)
                                    yield key
                                if reverse_key not in visited:
                                    visited.add(reverse_key)
                                    yield reverse_key

    @staticmethod
    def from_state_space(state_space: ProcessSetStateSpace) -> 'ProcessConcurrency':
        return ProcessConcurrency(
            concurrent_pairs=ProcessConcurrency.extract_concurrent_transition_pairs(state_space)
        )
