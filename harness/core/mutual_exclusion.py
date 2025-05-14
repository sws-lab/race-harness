from typing import Iterable, Tuple, Dict, Set, Optional
from harness.core.state_graph import StateGraphNode
from harness.core.process import Process
from harness.core.process_set import ProcessSetStateSpace

class ProcessMutualExclusionSegment:
    def __init__(self, segment: Iterable[Tuple[Process, StateGraphNode]]):
        self._states = set(segment)

    @property
    def processes(self) -> Iterable[Process]:
        return (
            process
            for process, _ in self._states
        )

    @property
    def states(self) -> Iterable[Tuple[Process, StateGraphNode]]:
        yield from self._states

    def extend(self, process: Process, state: StateGraphNode) -> 'ProcessMutualExclusionSegment':
        return self.union(((process, state),))

    def union(self, segment: Iterable[Tuple[Process, StateGraphNode]]) -> 'ProcessMutualExclusionSegment':
        return ProcessMutualExclusionSegment(
            self._states.union(segment)
        )
    
    def intersection(self, segment: Iterable[Tuple[Process, StateGraphNode]]) -> 'ProcessMutualExclusionSegment':
        return ProcessMutualExclusionSegment(
            self._states.intersection(segment)
        )
    
    def difference(self, segment: Iterable[Tuple[Process, StateGraphNode]]) -> 'ProcessMutualExclusionSegment':
        return ProcessMutualExclusionSegment(
            self._states.difference(segment)
        )
    
    def includes(self, segment: Iterable[Tuple[Process, StateGraphNode]]) -> bool:
        for process, state in segment:
            if (process, state) not in self._states:
                return False
        return True

    def __len__(self) -> int:
        return len(self.states)
    
    def __bool__(self) -> bool:
        return bool(self.states)

    def __iter__(self) -> Iterable[Tuple[Process, StateGraphNode]]:
        yield from self.states

    def __contains__(self, value) -> bool:
        return value in self._states

    def __eq__(self, value):
        return isinstance(value, ProcessMutualExclusionSegment) and len(self._states.symmetric_difference(value.states)) == 0
    
    def __ne__(self, value):
        return not self.__eq__(value)
    
    def __hash__(self):
        res = 0
        for state in self.states:
            res = res + hash(state)
        return res
    
    @staticmethod
    def empty() -> 'ProcessMutualExclusionSegment':
        return ProcessMutualExclusionSegment(())

class ProcessSetMutualExclusion:
    def __init__(self, state_space: ProcessSetStateSpace):
        self._state_space = state_space
        self._reverse_edges = dict()
        self._process_active_states = dict()
        self._process_set_state_index = dict()
        self._mutually_exclusive_states = dict()
        self._per_process_mutual_exclusion_segments = dict()
        self._mutual_exclusion_segments = None
        self._scan_state_space()

    @property
    def state_space(self) -> ProcessSetStateSpace:
        return self._state_space
    
    @property
    def mutual_exclusion_segments(self) -> Iterable[ProcessMutualExclusionSegment]:
        if self._mutual_exclusion_segments is None:
            self._mutual_exclusion_segments = self._generate_mutual_exclusion_segments()
        yield from self._mutual_exclusion_segments
    
    def process_mutual_exclusion_segments(self, process: Process) -> Iterable[ProcessMutualExclusionSegment]:
        if process not in self._per_process_mutual_exclusion_segments:
            self._per_process_mutual_exclusion_segments[process] = set(self._generate_process_mutual_exclusion_segments(process))
        yield from self._per_process_mutual_exclusion_segments[process]
    
    def _scan_state_space(self):
        self._scan_reverse_edges()
        self._scan_process_set_states()
        self._scan_mutually_exclusive_states()

    def _scan_reverse_edges(self):
        for process in self.state_space.process_set.processes:
            for state in process.entry_node.all_nodes:
                for edge in state.edges:
                    if edge.target not in self._reverse_edges:
                        self._reverse_edges[edge.target] = list()
                    self._reverse_edges[edge.target].append(edge)

    def _scan_process_set_states(self):
        for psstate in self.state_space.states:
            for process in self.state_space.process_set.processes:
                if process not in self._process_active_states:
                    self._process_active_states[process] = set()
                state = psstate.process_state(process).state
                self._process_active_states[process].add(state)

                if (process, state) not in self._process_set_state_index:
                    self._process_set_state_index[(process, state)] = list()
                self._process_set_state_index[(process, state)].append(psstate)

    def _scan_mutually_exclusive_states(self):
        for (process, state), psstates in self._process_set_state_index.items():
            if (process, state) not in self._mutually_exclusive_states:
                self._mutually_exclusive_states[(process, state)] = dict()
            for other_process in self.state_space.process_set.processes:
                if process != other_process:
                    other_process_active_states = set(
                        psstate.process_state(other_process).state
                        for psstate in psstates
                    )
                    self._mutually_exclusive_states[(process, state)][other_process] = self._process_active_states[other_process].difference(other_process_active_states)

    def _generate_mutual_exclusion_segments(self) -> Set[ProcessMutualExclusionSegment]:
        segments = set()
        for process in self.state_space.process_set.processes:
            for segment in self.process_mutual_exclusion_segments(process):
                segments.add(segment)
        return self._prune_mutual_exclusion_segments(segments)

    def _prune_mutual_exclusion_segments(self, segments: Set[ProcessMutualExclusionSegment]) -> Set[ProcessMutualExclusionSegment]:
        fixpoint_reached = False
        while not fixpoint_reached:
            fixpoint_reached = True

            new_segments = self._prune_embedded_mutual_exclusion_segments(segments)
            if new_segments is not None:
                segments = new_segments
                fixpoint_reached = False
                continue

            new_segments = self._prune_overlapping_mutual_exclusion_segments(segments)
            if new_segments is not None:
                segments = new_segments
                fixpoint_reached = False
                continue
        return segments

    def _prune_embedded_mutual_exclusion_segments(self, segments: Set[ProcessMutualExclusionSegment]) -> Optional[Set[ProcessMutualExclusionSegment]]:
        for segment_index, segment in enumerate(segments):
            for other_segment_index, other_segment in enumerate(segments):
                if segment_index == other_segment_index:
                    continue

                if segment.includes(other_segment):
                    new_segments = segments.copy()
                    new_segments.remove(other_segment)
                    return new_segments
        return None
    
    def _prune_overlapping_mutual_exclusion_segments(self, segments: Set[ProcessMutualExclusionSegment]) -> Optional[Set[ProcessMutualExclusionSegment]]:
        def segment_has_process(segment: ProcessMutualExclusionSegment, process: Process) -> bool:
            return any(p == process for p in segment.processes)

        for process in self.state_space.process_set.processes:
            for segment_index, segment in enumerate(segments):
                if not segment_has_process(segment, process):
                    continue
                
                for other_segment_index, other_segment in enumerate(segments):
                    if segment_index == other_segment_index or \
                        not segment_has_process(other_segment, process):
                        continue

                    diff = segment.difference(other_segment)
                    if any(p != process for p, _ in diff):
                        continue

                    for p, s in segment:
                        if p != process:
                            diff = diff.extend(p, s)

                    if diff == segment:
                        continue
                    
                    new_segments = segments.copy()
                    new_segments.remove(segment)
                    new_segments.add(diff)
                    return new_segments
        return None

    def _generate_process_mutual_exclusion_segments(self, process: Process) -> Iterable[ProcessMutualExclusionSegment]:
        segments = self._process_initial_mutual_exclusion_segments(process)
        segments = self._process_propagate_mutual_exclusion_segments(process, segments)
        segments = self._process_split_mutual_exclusion_segments(process, segments)
        yield from self._process_merge_state_exclusion_segments(process, segments)

    def _process_initial_mutual_exclusion_segments(self, process: Process) -> Dict[StateGraphNode, Set[ProcessMutualExclusionSegment]]:
        initial_entry_node_segment = ProcessMutualExclusionSegment.empty()
        for other_process, other_process_states in self._mutually_exclusive_states.get((process, process.entry_node), dict()).items():
            for other_process_state in other_process_states:
                initial_entry_node_segment = initial_entry_node_segment.extend(other_process, other_process_state)
        return {
            process.entry_node: {
                initial_entry_node_segment
            }
        }
    
    def _process_propagate_mutual_exclusion_segments(self, process: Process, initial_segments: Dict[StateGraphNode, Set[ProcessMutualExclusionSegment]]) -> Dict[StateGraphNode, Set[ProcessMutualExclusionSegment]]:
        segments = initial_segments
        fixpoint_reached = False
        while not fixpoint_reached:
            fixpoint_reached = True
            updated_segments = self._process_propagate_mutual_exclusion_segments_step(process, segments)
            if updated_segments is not None:
                segments = updated_segments
                fixpoint_reached = False
        return segments

    def _process_propagate_mutual_exclusion_segments_step(self, process: Process, segments: Dict[StateGraphNode, Set[ProcessMutualExclusionSegment]]) -> Optional[Dict[StateGraphNode, Set[ProcessMutualExclusionSegment]]]:
        for state in process.entry_node.all_nodes:
            state_segments = self._process_state_generate_mutual_exclusion_segments(process, state, segments)
            if state not in segments or state_segments.symmetric_difference(segments[state]):
                updated_segments = segments.copy()
                updated_segments[state] = state_segments
                return updated_segments

        return None
    
    def _process_state_generate_mutual_exclusion_segments(self, process: Process, state: StateGraphNode, segments: Dict[StateGraphNode, Set[ProcessMutualExclusionSegment]]) -> Set[ProcessMutualExclusionSegment]:
        surrounding_segments = self._process_state_collect_surrounding_segments(process, state, segments)
        mutually_exclusive_states = self._process_state_collect_mutually_exclusive_states(process, state)

        state_segments = set()
        covered_mutually_exclusive_states = set()
        for surrounding_mutex in surrounding_segments:
            mutex = surrounding_mutex.intersection(mutually_exclusive_states)
            if mutex:
                covered_mutually_exclusive_states = covered_mutually_exclusive_states.union(mutex)
                state_segments.add(mutex)
        mutually_exclusive_states.difference_update(covered_mutually_exclusive_states)

        if mutually_exclusive_states:
            mutex = ProcessMutualExclusionSegment(mutually_exclusive_states)
            state_segments.add(mutex)
        return state_segments
    
    def _process_state_collect_surrounding_segments(self, process: Process, state: StateGraphNode, segments: Dict[StateGraphNode, Set[ProcessMutualExclusionSegment]]) -> Set[ProcessMutualExclusionSegment]:
        surrounding_segments = set()
        for edge in state.edges:
            surrounding_segments = surrounding_segments.union(segments.get(edge.target, ()))

        if state != process.entry_node:
            for incoming_edge in self._reverse_edges.get(state, ()):
                surrounding_segments = surrounding_segments.union(segments.get(incoming_edge.source, ()))

        return surrounding_segments
    
    def _process_state_collect_mutually_exclusive_states(self, process: Process, state: StateGraphNode) -> Set[Tuple[Process, StateGraphNode]]:
        mutually_exclusive_states = set()
        for other_process, other_process_states in self._mutually_exclusive_states.get((process, state), dict()).items():
            for other_process_state in other_process_states:
                mutually_exclusive_states.add((other_process, other_process_state))
        return mutually_exclusive_states
    
    def _process_split_mutual_exclusion_segments(self, process: Process, mutexes: Dict[StateGraphNode, Set[ProcessMutualExclusionSegment]]) -> Dict[StateGraphNode, Set[ProcessMutualExclusionSegment]]:
        new_mutexes = dict()
        for state, mutex_segments in mutexes.items():
            new_segments = set()
            for segment in mutex_segments:
                other_processes = set(
                    p
                    for p, _ in segment
                )
                for other_process in other_processes:
                    subsegment = (
                        (p, s)
                        for p, s in segment
                        if p == other_process or p == process
                    )
                    new_segments.add(ProcessMutualExclusionSegment(subsegment))
            new_mutexes[state] = new_segments
        return new_mutexes

    def _process_merge_state_exclusion_segments(self, process: Process, mutexes: Dict[StateGraphNode, Set[ProcessMutualExclusionSegment]]) -> Iterable[ProcessMutualExclusionSegment]:
        mutex_index = dict()
        for state, state_mutexes in mutexes.items():
            for state_mutex in state_mutexes:
                if state_mutex in mutex_index:
                    mutex_index[state_mutex] = mutex_index[state_mutex].extend(process, state)
                else:
                    mutex_index[state_mutex] = state_mutex.union(((process, state),))
        yield from mutex_index.values()
