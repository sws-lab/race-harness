import io
import dataclasses
from typing import Iterable, Dict, Tuple, List, Optional
from harness.process import Process
from harness.state_graph import StateGraphNode, StateGraphMessageEnvelope, StateGraphMessage, StateGraphEdge, StateGraphMessageParticipant
from harness.error import HarnessError

@dataclasses.dataclass
class PostboxEntry:
    source: StateGraphMessageParticipant
    destination: StateGraphMessageParticipant
    message: StateGraphMessage

    def __eq__(self, value):
        return isinstance(value, PostboxEntry) and value.source == self.source and value.target == self.target and value.message == self.message
    
    def __ne__(self, value):
        return not self.__eq__(value)
    
    def __hash__(self):
        return 31 * hash(self.source) + 17 * hash(self.destination) + 13 * hash(self.message)

class ProcessSetState:
    def __init__(self, state: Dict[Process, StateGraphNode], postbox: List[PostboxEntry]):
        self._state = state.copy()
        self._postbox = postbox

    @property
    def process_states(self) -> Iterable[Tuple[Process, StateGraphNode]]:
        return self._state.items()
    
    @property
    def postbox(self) -> Iterable[PostboxEntry]:
        return self._postbox
    
    def process_state(self, process: Process) -> Optional[StateGraphNode]:
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
        visited = set()
        if self._postbox:
            entry = self._postbox[0]
            for state in self._next_triggered_states(entry, self._postbox[1:]):
                yield state
                visited.add(state)
        else:
            for state in self._next_untriggered_states():
                yield state
                visited.add(state)

    def _next_untriggered_states(self) -> Iterable['ProcessSetState']:
        for process, state in self.process_states:
            unconditional_edges = (
                edge
                for edge in state.edges
                if edge.trigger is None
            )
            yield from self._next_states_from_edges(process, unconditional_edges, self._postbox, None)

    def _next_triggered_states(self, entry: PostboxEntry, postbox: Iterable[PostboxEntry]) -> Iterable['ProcessSetState']:
        for process, state in self.process_states:
            if process == entry.destination:
                message = process.map_message(entry.source, entry.message)
                triggered_edges = (
                    edge
                    for edge in state.edges
                    if edge.trigger == message
                )
                yield from self._next_states_from_edges(process, triggered_edges, postbox, entry)

    def _next_states_from_edges(self, process: Process, edges: Iterable[StateGraphEdge], postbox: Iterable[PostboxEntry], trigger_entry: Optional[PostboxEntry]) -> Iterable['ProcessSetState']:
            def process_action_envelope(envelope: StateGraphMessageEnvelope) -> PostboxEntry:
                for destination, _ in self.process_states:
                    if envelope.destination.matches(destination=destination, in_response_to=trigger_entry.source if trigger_entry is not None else trigger_entry):
                        return PostboxEntry(source=process, destination=destination, message=envelope.message)
                raise HarnessError('Unable to find process matching the envelope')
            for edge in edges:
                new_state = {
                    other_process: edge.target if other_process == process else other_state
                    for other_process, other_state in self.process_states
                }
                new_postbox = [
                    *postbox,
                    *(
                        process_action_envelope(envelope)
                        for envelope in edge.action.message_envelopes
                    )
                ]
                yield ProcessSetState(state=new_state, postbox=new_postbox)

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
        out.write('[')
        add_comma = False
        for entry in self._postbox:
            if add_comma:
                out.write(', ')
            out.write(f'{entry.destination}: {entry.message} from {entry.source}')
            add_comma = True
        out.write(']')
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
        try:
            for own_envelope, other_envelope in zip(self.postbox, value.postbox, strict=True):
                if own_envelope != other_envelope:
                    return False
        except ValueError:
            return False
        return True
    
    def __ne__(self, value):
        return not self.__eq__(value)
    
    def __hash__(self):
        res = 0
        for process, state in self.process_states:
            res = res * 67 + hash(process) * 31 + hash(state) * 17
        for entry in self.postbox:
            res = res * 67 + hash(entry) * 13
        return res

class ProcessSet:
    def __init__(self):
        self._processes = dict()

    def add_process(self, mnemonic: str, entry_node: StateGraphNode) -> Process:
        process = Process(mnemonic=mnemonic, entry_node=entry_node)
        self._processes[mnemonic] = process
        return process
    
    @property
    def processes(self) -> Iterable[Process]:
        return self._processes.values()
    
    @property
    def initial_state(self) -> ProcessSetState:
        return ProcessSetState(state={
            process: process.entry_node
            for process in self.processes
        }, postbox=[])
