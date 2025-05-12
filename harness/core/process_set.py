import io
import functools
from typing import Iterable, Dict, Tuple, Optional
from harness.core.process import Process, ProcessState, StateGraphEdge
from harness.core.state_graph import StateGraphNode
from harness.core.error import HarnessError

class ProcessSetState:
    def __init__(self, process_set: 'ProcessSet', state: Dict[Process, ProcessState]):
        self._process_set = process_set
        self._state = state.copy()
        self._next_transitions_cache = None

    @property
    def process_set(self) -> 'ProcessSet':
        return self._process_set

    @property
    def processes(self) -> Iterable[Process]:
        yield from self._state.keys()

    @property
    def process_states(self) -> Iterable[Tuple[Process, ProcessState]]:
        yield from self._state.items()
    
    def process_state(self, process: Process) -> Optional[ProcessState]:
        return self._state.get(process, None)

    def reachable_states(self, *, include_self: bool = False) -> Iterable['ProcessSetState']:
        visited = set()
        queue = list(self.next_states)
        if include_self:
            yield self
            visited.add(self)
        while len(queue) > 0:
            state = queue.pop()
            if state not in visited:
                yield state
                visited.add(state)
                queue.extend(state.next_states)

    @property
    def next_transitions(self) -> Iterable[Tuple[Process, StateGraphEdge, 'ProcessSetState']]:
        if self._next_transitions_cache is None:
            self._next_transitions_cache = list(self._next_transitions)
        yield from self._next_transitions_cache

    @property
    def next_states(self) -> Iterable['ProcessSetState']:
        for _, _, state in self.next_transitions:
            yield state

    @property
    def with_empty_mailboxes(self) -> 'ProcessSetState':
        return ProcessSetState(
            process_set=self.process_set,
            state={
                process: state.with_empty_mailbox
                for process, state in self._state.items()
            }
        )

    @property
    def _next_transitions(self) -> Iterable[Tuple[Process, StateGraphEdge, 'ProcessSetState']]:
        active_communications = set(
            (entry.origin, receiver)
            for receiver, receiver_state in self.process_states
            for entry in receiver_state.mailbox
        )
        for process in self.processes:
            yield from self._next_transitions_for(process, lambda receiver: (process, receiver) in active_communications)

    @property
    def state_space(self) -> 'ProcessSetStateSpace':
        return ProcessSetStateSpace(
            process_set=self.process_set,
            states=self.reachable_states(include_self=True)
        )

    def _next_transitions_for(self, process: Process, has_active_comms_with) -> Iterable[Tuple[Process, StateGraphEdge, 'ProcessSetState']]:
        for next_process_state, transition_edge, outgoing_messages in self._state[process].next_transitions:
            new_state = {
                other_process: other_state if other_process != process else next_process_state
                for other_process, other_state in self.process_states
            }
            blocks_on_messaging = False
            for envelope in outgoing_messages.envelopes:
                matching_destinations = (other_process for other_process in self.processes if envelope.destination.matches(destination=other_process))
                has_destinations = False
                for other_process in matching_destinations:
                    if has_active_comms_with(other_process):
                        blocks_on_messaging = True
                        break
                    has_destinations = True
                    new_state[other_process] = new_state[other_process].push_message(origin=process, message=envelope.message)
                if not has_destinations and not blocks_on_messaging:
                    raise HarnessError(f'Message {envelope.message} for {envelope.destination} from {process} has no matching destinations')
            if not blocks_on_messaging:
                yield process, transition_edge, ProcessSetState(process_set=self.process_set, state=new_state)

    def __str__(self) -> str:
        out = io.StringIO()
        out.write('{')
        add_comma = False
        for process, state in self.process_states:
            if add_comma:
                out.write(', ')
            out.write(f'{process}: {state}')
            add_comma = True
        out.write('}')
        return out.getvalue()
    
    def __repr__(self):
        return str(self)
    
    def __eq__(self, value):
        if not isinstance(value, ProcessSetState):
            return False
        own_processes = set(process for process, _ in self.process_states)
        other_processes = set(process for process, _ in self.process_states)
        if own_processes.symmetric_difference(other_processes):
            return False
        for process in own_processes:
            if self.process_state(process) != value.process_state(process):
                return False
        return True
    
    def __ne__(self, value):
        return not self.__eq__(value)
    
    def __hash__(self):
        res = 0
        for process, state in self.process_states:
            res = res * 67 + hash(process) * 31 + hash(state) * 17
        return res

class ProcessSet:
    def __init__(self):
        self._processes = dict()

    def add_process(self, mnemonic: str, entry_node: StateGraphNode) -> Process:
        process = Process(mnemonic=mnemonic, entry_node=entry_node)
        self._processes[process] = process
        return process
    
    @property
    def processes(self) -> Iterable[Process]:
        return self._processes.keys()
    
    @property
    def initial_state(self) -> ProcessSetState:
        return ProcessSetState(process_set=self, state={
            process: process.initial_state
            for process in self.processes
        })
    
    @property
    def state_space(self) -> 'ProcessSetStateSpace':
        return self.initial_state.state_space
    
    def __contains__(self, item) -> bool:
        return item in self._processes
    
    def __iter__(self) -> Iterable[Process]:
        yield from self.processes

class ProcessSetStateSpace:
    def __init__(self, process_set: ProcessSet, states: Iterable[ProcessSetState]):
        self._process_set = process_set
        self._states = { # For ordering stability
            state: state
            for state in states
        }

    @property
    def process_set(self) -> ProcessSet:
        return self._process_set

    @property
    def states(self) -> Iterable[ProcessSetState]:
        yield from self._states.keys()

    def __len__(self) -> int:
        return len(self._states)

    def __contains__(self, value):
        return value in self._states
    
    def __iter__(self) -> Iterable[ProcessSetState]:
        return self.states
    
    def match_states(self, process: Process, state: StateGraphNode) -> Iterable[ProcessSetState]:
        for psstate in self._states.keys():
            if psstate.process_state(process).state == state:
                yield psstate

    def collect_concurrent_transitions(self) -> Iterable[Tuple[Process, StateGraphEdge, Process, StateGraphEdge]]:
        visited = set()
        for process in self.process_set.processes:
            for state in process.entry_node.all_nodes:
                psstates = set(self.match_states(process, state))
                concurrent_transitions = dict()
                for psstate in psstates:
                    for transition_process, transition_edge, _ in psstate.next_transitions:
                        if transition_process == process:
                            concurrent_transitions[transition_edge] = self.match_states(process, transition_edge.target)
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
    
    def derive_invariant(self, process: Process, state: StateGraphNode, invariant_process: Process) -> 'ProcessStateInvariant':
        from harness.core.invariants import ProcessStateInvariant
        return ProcessStateInvariant.derive(psstates=self, process=process, state=state, invariant_process=invariant_process)

    @functools.cached_property
    def all_invariants(self) -> Iterable['ProcessStateInvariant']:
        for process1 in self.process_set.processes:
            for process2 in self.process_set.processes:
                if process1 == process2:
                    continue
                for state in process1.entry_node.all_nodes:
                    yield self.derive_invariant(process=process1, state=state, invariant_process=process2)
