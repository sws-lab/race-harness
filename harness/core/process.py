import io
import dataclasses
from typing import Callable, Optional, List, Iterable, Tuple
from harness.core.state_graph import StateGraphNode, StateGraphMessageParticipant, StateGraphMessage, StateGraphMessageDestination, StateGraphEdge, StateGraphMessageEnvelope

@dataclasses.dataclass
class ProcessMailboxEntry:
    origin: StateGraphMessageParticipant
    message: StateGraphMessage

    def __hash__(self) -> int:
        return hash(self.origin) * 11 + hash(self.message)
    
    def __eq__(self, value):
        return isinstance(value, ProcessMailboxEntry) and value.origin == self.origin and value.message == self.message
    
    def __ne__(self, value):
        return not self.__eq__(value)

@dataclasses.dataclass
class OutgoingMessageBatch:
    trigger: Optional[ProcessMailboxEntry]
    envelopes: List[StateGraphMessageEnvelope]

class ProcessState:
    def __init__(self, process: 'Process', state: StateGraphNode, mailbox: Iterable[ProcessMailboxEntry]):
        self._process = process
        self._state = state
        self._mailbox = list(mailbox)

    @property
    def process(self) -> 'Process':
        return self._process

    @property
    def state(self) -> StateGraphNode:
        return self._state
    
    @property
    def mailbox(self) -> Iterable[ProcessMailboxEntry]:
        yield from self._mailbox

    @property
    def is_mailbox_empty(self) -> bool:
        return len(self._mailbox) == 0
    
    @property
    def with_empty_mailbox(self) -> 'ProcessState':
        return ProcessState(process=self.process, state=self.state, mailbox=())

    def push_message(self, origin: StateGraphMessageParticipant, message: StateGraphMessage) -> 'ProcessState':
        mapped_message = self.process.map_inbound_message(origin, message)
        return ProcessState(
            process=self.process,
            state=self.state,
            mailbox=[*self.mailbox, ProcessMailboxEntry(origin=origin, message=mapped_message)]
        )
    
    @property
    def next_states(self) -> Iterable[Tuple['ProcessState', OutgoingMessageBatch]]:
        has_triggerred_states = False
        if self._mailbox:
            for trigger in self._mailbox:
                new_mailbox = self._mailbox.copy()
                new_mailbox.remove(trigger)
                for state in self._next_triggered_states(trigger, new_mailbox):
                    has_triggerred_states = True
                    yield state
        if not has_triggerred_states:
            yield from self._next_untrigerred_states()

    def _next_triggered_states(self, trigger: ProcessMailboxEntry, mailbox: Iterable[ProcessMailboxEntry]) -> Iterable[Tuple['ProcessState', OutgoingMessageBatch]]:
        triggered_edges = (
            edge
            for edge in self.state.edges
            if edge.trigger == trigger.message
        )
        yield from self._next_states_from_edges(triggered_edges, mailbox, trigger)

    def _next_untrigerred_states(self) -> Iterable[Tuple['ProcessState', OutgoingMessageBatch]]:
        unconditional_edges = (
            edge
            for edge in self.state.edges
            if edge.trigger is None
        )
        yield from self._next_states_from_edges(unconditional_edges, self._mailbox, None)

    def _next_states_from_edges(self, edges: Iterable[StateGraphEdge], mailbox: Iterable[ProcessMailboxEntry], trigger: Optional[ProcessMailboxEntry]) -> Iterable[Tuple['ProcessState', OutgoingMessageBatch]]:                
        for edge in edges:
            yield ProcessState(process=self.process, state=edge.target, mailbox=mailbox), OutgoingMessageBatch(trigger=trigger, envelopes=[
                self.process.map_outbound_message(edge, envelope)
                for envelope in edge.action.message_envelopes
            ])
    
    def __str__(self) -> str:
        out = io.StringIO()
        out.write(f'{self.state}')
        if not self.is_mailbox_empty:
            out.write(' [')
            add_comma = False
            for entry in self.mailbox:
                if add_comma:
                    out.write(', ')
                out.write(f'{entry.message} from {entry.origin}')
                add_comma = True
            out.write(']')
        return out.getvalue()
    
    def __eq__(self, value) -> bool:
        if not isinstance(value, ProcessState):
            return False
        
        if value.state != self.state:
            return False
        
        try:
            for entry1, entry2 in zip(value.mailbox, self.mailbox, strict=True):
                if entry1 != entry2:
                    return False
        except ValueError:
            return False
        
        return True
    
    def __ne__(self, value) -> bool:
        return not self.__eq__(value)
    
    def __hash__(self) -> int:
        res = hash(self.state) * 37
        for entry in self.mailbox:
            res = res * 13 + hash(entry)
        return res

class Process(StateGraphMessageParticipant, StateGraphMessageDestination):
    def __init__(self, mnemonic: str, entry_node: StateGraphNode):
        super().__init__()
        self._mnemonic = mnemonic
        self._entry_node = entry_node
        self._inbound_mappers = list()
        self._outbound_mappers = list()

    @property
    def mnemonic(self) -> str:
        return self._mnemonic
    
    def matches(self, destination: StateGraphMessageParticipant):
        return self == destination
    
    @property
    def entry_node(self) -> StateGraphNode:
        return self._entry_node
    
    @property
    def initial_state(self) -> ProcessState:
        return ProcessState(process=self, state=self.entry_node, mailbox=())
    
    def add_inbound_message_mapping(self, mapping: Callable[[StateGraphMessageParticipant, StateGraphMessage], Optional[StateGraphMessage]]):
        self._inbound_mappers.append(mapping)

    def add_outbound_message_mapping(self, mapping: Callable[[StateGraphEdge, StateGraphMessageEnvelope], Optional[StateGraphMessageEnvelope]]):
        self._outbound_mappers.append(mapping)

    def map_inbound_message(self, source: StateGraphMessageParticipant, message: StateGraphMessage) -> StateGraphMessage:
        for mapping in self._inbound_mappers:
            mapped_msg = mapping(source, message)
            if mapped_msg is not None:
                return mapped_msg
        return message
    
    def map_outbound_message(self, edge: StateGraphEdge, envelope: StateGraphMessageEnvelope) -> StateGraphMessageEnvelope:
        for mapping in self._outbound_mappers:
            mapped_envelope = mapping(edge, envelope)
            if mapped_envelope is not None:
                return mapped_envelope
        return envelope
    
    def __str__(self) -> str:
        return self.mnemonic
    
    def __eq__(self, value) -> bool:
        return isinstance(value, Process) and value.mnemonic == self.mnemonic
    
    def __ne__(self, value) -> bool:
        return not self.__eq__(value)
    
    def __hash__(self) -> int:
        return hash(self.mnemonic)
