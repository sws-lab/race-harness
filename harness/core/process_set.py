import io
from typing import Iterable, Dict, Tuple, Optional
from harness.core.process import Process, ProcessState
from harness.core.state_graph import StateGraphNode
from harness.core.error import HarnessError

class ProcessSetState:
    def __init__(self, state: Dict[Process, ProcessState]):
        self._state = state.copy()

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
        queue = [*self.next_states]
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
    def next_states(self) -> Iterable['ProcessSetState']:
        has_nonempty_mailbox = False
        for process, process_state in self.process_states:
            if not process_state.is_mailbox_empty:
                has_nonempty_mailbox = True
                yield from self._next_states_for(process)
        if not has_nonempty_mailbox:
            for process in self.processes:
                yield from self._next_states_for(process)

    def _next_states_for(self, process: Process) -> Iterable['ProcessSetState']:
        for next_process_state, outgoing_messages in self._state[process].next_states:
            new_state = {
                other_process: other_state if other_process != process else next_process_state
                for other_process, other_state in self.process_states
            }
            for envelope in outgoing_messages.envelopes:
                matching_destinations = (other_process for other_process in self.processes if envelope.destination.matches(destination=other_process, in_response_to=outgoing_messages.trigger.origin if outgoing_messages.trigger is not None else None))
                has_destinations = False
                for other_process in matching_destinations:
                    has_destinations = True
                    new_state[other_process] = new_state[other_process].push_message(origin=process, message=envelope.message)
                if not has_destinations:
                    raise HarnessError(f'Message {envelope.message} for {envelope.destination} from {process} has no matching destinations')
            yield ProcessSetState(state=new_state)

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
        own_processes = set(process for process in self.process_states)
        other_processes = set(process for process in self.process_states)
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
        return ProcessSetState(state={
            process: process.initial_state
            for process in self.processes
        })
    
    def __contains__(self, item) -> bool:
        return item in self._processes
    
    def __iter__(self) -> Iterable[Process]:
        yield from self.processes
