import dataclasses
from typing import Iterable, Dict, Set, Union, Optional, Tuple
from harness.core import ProcessSet, Process, StateGraphNode, StateGraphEdge, StateGraphMessage

# This module contains an analyzer for process set state machines. The analyzer
# goal is to determine "concurrent spaces" for process state. Concurrent space
# is defined as a mapping, that for each process in the system provides a set of
# states, such that complete and valid system state could be sampled from the
# mapping by independently selecting any provided state for each process. Thus,
# concurrent space indicates for a given process, which states other processes
# can take concurrently transitioning between them without synchronizing with
# the given process. This exposes potential concurrency within the system.
#
# The core idea of the analyzer algorithm is identification "concurrent
# segments" for each process pair. Concurrent segment for a pair of processes is
# a set of states that the second process can take assuming that the first
# process is in some certain state. Based on concurrent segments for each
# process pair, concurrent space is refined until fixpoint has been reached.
#
# The core idea of concurrent segment computation is determining a subset of all
# process states, such that any state of the subset could have been reached by
# process P1, while process P2 has reached some state S2. This is done by
# considering, with respect to state S2, when could have been the last time
# processes P1 and P2 communicated, and how far P1 could have gone since then.

# Synchronization point - a pair of edges where two processes synchronize. There
# are two types of synchronization points (type is not stored in the structure,
# but implied by control flow):
#   1. Inbound synchronization -- some process at edge "edge" requires an
#      inbound message from process "synchronization_process" edge
#      "synchronization_edge".
#   2. Outbound synchronization -- some process at edge "edge" sends an outbound
#      message, which process "synchronization_process" would consume at edge
#      "synchronization_edge".
# 
# For any edge, there might exist multiple synchronization points: consider
# cases when an edge consumes message that could have been produced in multiple
# different places.
@dataclasses.dataclass
class ProcessSynchronizationPoint:
    edge: int
    synchronization_process: int
    synchronization_edge: int

    def __hash__(self):
        return 31 * hash(self.edge) + 17 * hash(self.synchronization_process) + 7 * hash(self.synchronization_edge)
    
    def __eq__(self, value):
        return isinstance(value, ProcessSynchronizationPoint) and \
            value.edge == self.edge and \
            value.synchronization_process == self.synchronization_process and \
            value.synchronization_edge == self.synchronization_edge
    
    def __ne__(self, value):
        return not self.__eq__(value)

# Special case of a synchronization point -- implies that there could have been
# no synchronization since the system started executing.    
@dataclasses.dataclass
class ProcessUnsynchronizedPoint:
    def __hash__(self):
        return 0
    
    def __eq__(self, value):
        return isinstance(value, ProcessUnsynchronizedPoint)
    
    def __ne__(self, value):
        return not self.__eq__(value)
    
class SimpleProcessSet:
    def __init__(self, processes: ProcessSet):
        self._processes = dict()
        self._inverse_processes = dict()
        self._states = dict()
        self._inverse_states = dict()
        self._edges = dict()
        self._inverse_edges = dict()
        self._messages = dict()
        self._inverse_messages = dict()

        self._process_entry_points = dict()
        self._process_states = dict()
        self._state_forward_edges = dict()
        self._edge_source_states = dict()
        self._edge_target_states = dict()
        self._edge_triggers = dict()
        self._edge_messages = dict()
        self._initialize(processes)

    def get_real_process(self, process_id: int) -> Process:
        return self._inverse_processes[process_id]
    
    def get_real_state(self, state_id: int) -> StateGraphNode:
        return self._inverse_states[state_id]
    
    def get_real_edge(self, edge_id: int) -> StateGraphEdge:
        return self._inverse_edges[edge_id]
    
    def get_real_message(self, message_id: int) -> StateGraphMessage:
        return self._inverse_messages[message_id]

    @property
    def processes(self) -> Iterable[int]:
        yield from self._processes.values()

    def process_entry_point(self, process_id: int) -> int:
        return self._process_entry_points[process_id]
    
    def process_states(self, process_id: int) -> Iterable[int]:
        yield from self._process_states[process_id]

    def state_forward_edges(self, state_id: int) -> Iterable[int]:
        yield from self._state_forward_edges[state_id]

    def edge_source_state(self, edge_id: int) -> int:
        return self._edge_source_states[edge_id]
    
    def edge_target_state(self, edge_id: int) -> int:
        return self._edge_target_states[edge_id]
    
    def edge_trigger(self, edge_id: int) -> Optional[int]:
        return self._edge_triggers.get(edge_id, None)
    
    def edge_outbound_messages(self, edge_id: int) -> Iterable[Tuple[int, int]]:
        yield from self._edge_messages.get(edge_id, ())
    
    def _get_process_id(self, process: Process) -> int:
        process_id = self._processes.get(process, None)
        if process_id is None:
            process_id = len(self._processes)
            self._processes[process] = process_id
            self._inverse_processes[process_id] = process
        return process_id
    
    def _get_state_id(self, state: StateGraphNode) -> int:
        state_id = self._states.get(state, None)
        if state_id is None:
            state_id = len(self._states)
            self._states[state] = state_id
            self._inverse_states[state_id] = state
        return state_id
    
    def _get_edge_id(self, edge: StateGraphEdge) -> int:
        edge_id = self._edges.get(edge, None)
        if edge_id is None:
            edge_id = len(self._edges)
            self._edges[edge] = edge_id
            self._inverse_edges[edge_id] = edge
        return edge_id
    
    def _get_message_id(self, message: StateGraphMessage):
        message_id = self._messages.get(message, None)
        if message_id is None:
            message_id = len(self._messages)
            self._messages[message] = message_id
            self._inverse_messages[message_id] = message
        return message_id

    def _initialize(self, processes: ProcessSet):
        for process in processes.processes:
            process_id = self._get_process_id(process)
            self._process_entry_points[process_id] = self._get_state_id(process.entry_node)
            self._process_states[process_id] = list()
            for state in process.entry_node.all_nodes:
                state_id = self._get_state_id(state)
                self._process_states[process_id].append(state_id)
                for edge in state.edges:
                    edge_id = self._get_edge_id(edge)
                    if state_id not in self._state_forward_edges:
                        self._state_forward_edges[state_id] = list()
                    self._state_forward_edges[state_id].append(edge_id)
                    self._edge_source_states[edge_id] = self._get_state_id(edge.source)
                    self._edge_target_states[edge_id] = self._get_state_id(edge.target)
                    if edge.trigger is not None:
                        self._edge_triggers[edge_id] = self._get_message_id(edge.trigger)
                    for envelope in edge.action.message_envelopes:
                        envelope = process.map_outbound_message(edge, envelope)
                        if edge_id not in self._edge_messages:
                            self._edge_messages[edge_id] = list()
                        for dest_process in processes.processes:
                            if envelope.destination.matches(dest_process):
                                self._edge_messages[edge_id].append((self._get_process_id(dest_process), self._get_message_id(envelope.message)))

class ProcessSetAnalyzer:
    def __init__(self, processes: ProcessSet):
        self._real_processes = processes
        self._simple_processes = SimpleProcessSet(processes)
        self._real_incoming_edges = dict()
        self._real_inbound_synchronization_edges = dict() # List of producers (process + edge) for each consumer (process + edge)
        self._real_outbound_synchronization_edges = dict() # List of consumers (process + edge) for each producer (process + edge)
        self._incoming_edges = dict()
        self._inbound_synchronization_edges = dict() # List of producers (process + edge) for each consumer (process + edge)
        self._outbound_synchronization_edges = dict() # List of consumers (process + edge) for each producer (process + edge)
        # Caches of intermediate results -- considerable speedup
        self._infer_inbound_synchronized_past_boundary_cache = dict()
        self._infer_inbound_synchronized_future_limit_cache = dict()
        self._edge_sequenced_after_without_synchronization_cache = dict()
        self._infer_outbound_boundary_cache = dict()
        self._infer_relevant_outbounds_cache = dict()
        self._infer_concurrent_segment_cache = dict()
        # Preliminary scan of the state machines
        self._scan_process_set()
        self._real_scan_process_set()

        # for (consumer_process, consumer_edge), producer_edges in self._inbound_synchronization_edges.items():
        #     real_consumer_process = self._simple_processes.get_real_process(consumer_process)
        #     real_consumer_edge  = self._simple_processes.get_real_edge(consumer_edge)
        #     for (producer_process, producer_edge) in producer_edges:
        #         real_producer_process = self._simple_processes.get_real_process(producer_process)
        #         real_producer_edge = self._simple_processes.get_real_edge(producer_edge)
        #         if (real_producer_process, real_producer_edge) not in self._real_inbound_synchronization_edges[(real_consumer_process, real_consumer_edge)]:
        #             raise 'REEEEE!!!!'
        #         print(real_consumer_process, real_consumer_edge, real_producer_process, real_producer_edge)

        # for (consumer_process, consumer_edge), producer_edges in self._real_inbound_synchronization_edges.items():
        #     simple_consumer_process = self._simple_processes._get_process_id(consumer_process)
        #     simple_consumer_edge  = self._simple_processes._get_edge_id(consumer_edge)
        #     for (producer_process, producer_edge) in producer_edges:
        #         simple_producer_process = self._simple_processes._get_process_id(producer_process)
        #         simple_producer_edge = self._simple_processes._get_edge_id(producer_edge)
        #         print(simple_consumer_process, simple_consumer_edge, simple_producer_process, simple_producer_edge)
        #         if (simple_producer_process, simple_producer_edge) not in self._inbound_synchronization_edges[(simple_consumer_process, simple_consumer_edge)]:
        #             raise 'REEEEE!!!!'
                
        # for (producer_process, producer_edge), consumer_edges in self._outbound_synchronization_edges.items():
        #     real_producer_process = self._simple_processes.get_real_process(producer_process)
        #     real_producer_edge  = self._simple_processes.get_real_edge(producer_edge)
        #     for (consumer_process, consumer_edge) in consumer_edges:
        #         real_consumer_process = self._simple_processes.get_real_process(consumer_process)
        #         real_consumer_edge = self._simple_processes.get_real_edge(consumer_edge)
        #         if (real_consumer_process, real_consumer_edge) not in self._real_outbound_synchronization_edges[(real_producer_process, real_producer_edge)]:
        #             raise 'REEEEE!!!!'
        #         print(real_consumer_process, real_consumer_edge, real_producer_process, real_producer_edge)

        # for (producer_process, producer_edge), consumer_edges in self._real_outbound_synchronization_edges.items():
        #     simple_producer_process = self._simple_processes._get_process_id(producer_process)
        #     simple_producer_edge  = self._simple_processes._get_edge_id(producer_edge)
        #     for (consumer_process, consumer_edge) in consumer_edges:
        #         simple_consumer_process = self._simple_processes._get_process_id(consumer_process)
        #         simple_consumer_edge = self._simple_processes._get_edge_id(consumer_edge)
        #         print(simple_consumer_process, simple_consumer_edge, simple_producer_process, simple_producer_edge)
        #         if (simple_consumer_process, simple_consumer_edge) not in self._outbound_synchronization_edges[(simple_producer_process, simple_producer_edge)]:
        #             raise 'REEEEE!!!!'
    
    def _incoming_edges_for(self, node: StateGraphNode) -> Iterable[StateGraphEdge]:
        yield from self._incoming_edges.get(node, ())

    # Preliminary step. Scan state machine of each process:
    #   1. Determine inverse transitions.
    #   2. Match edges of message producers and consumers, such that for every
    #      producer/consumer edge we would know complete list of respective
    #      consumer/producer edges for the same message.
    def _scan_process_set(self):
        message_produders = dict()
        message_consumers = dict()

        for process in self._simple_processes.processes:
            for node in self._simple_processes.process_states(process):
                for edge in self._simple_processes.state_forward_edges(node):
                    edge_target = self._simple_processes.edge_target_state(edge)
                    if edge_target not in self._incoming_edges:
                        self._incoming_edges[edge_target] = list()
                    if edge not in self._incoming_edges[edge_target]:
                        self._incoming_edges[edge_target].append(edge)

                    edge_trigger = self._simple_processes.edge_trigger(edge)
                    if edge_trigger is not None:
                        if (process, edge_trigger) not in message_consumers:
                            message_consumers[(process, edge_trigger)] = list()
                        message_consumers[(process, edge_trigger)].append(edge)

                    for destination, message in self._simple_processes.edge_outbound_messages(edge):
                        if (process, destination, message) not in message_produders:
                            message_produders[(process, destination, message)] = list()
                        message_produders[(process, destination, message)].append(edge)

        for (producer_process, destination_process, message), producer_edges in message_produders.items():
            for (consumer_process, consumed_message), consumer_edges in message_consumers.items():
                for producer_edge in producer_edges:
                    if consumer_process != destination_process:
                        continue
                    if self._simple_processes.get_real_process(consumer_process).map_inbound_message(self._simple_processes.get_real_process(producer_process), self._simple_processes.get_real_message(message)) != self._simple_processes.get_real_message(consumed_message):
                        continue
                    if (producer_process, producer_edge) not in self._outbound_synchronization_edges:
                        self._outbound_synchronization_edges[(producer_process, producer_edge)] = list()
                    for consumer_edge in consumer_edges:
                        if (consumer_process, consumer_edge) not in self._inbound_synchronization_edges:
                            self._inbound_synchronization_edges[(consumer_process, consumer_edge)] = list()
                        if (producer_process, producer_edge) not in self._inbound_synchronization_edges[(consumer_process, consumer_edge)]:
                            self._inbound_synchronization_edges[(consumer_process, consumer_edge)].append((producer_process, producer_edge))
                        if (consumer_process, consumer_edge) not in self._outbound_synchronization_edges[(producer_process, producer_edge)]:
                            self._outbound_synchronization_edges[(producer_process, producer_edge)].append((consumer_process, consumer_edge))

    def _real_scan_process_set(self):
        message_produders = dict()
        message_consumers = dict()

        for process in self._real_processes.processes:
            for node in process.entry_node.all_nodes:
                for edge in node.edges:
                    if edge.target not in self._real_incoming_edges:
                        self._real_incoming_edges[edge.target] = list()
                    if edge not in self._real_incoming_edges[edge.target]:
                        self._real_incoming_edges[edge.target].append(edge)
                    if edge.trigger is not None:
                        if (process, edge.trigger) not in message_consumers:
                            message_consumers[(process, edge.trigger)] = list()
                        message_consumers[(process, edge.trigger)].append(edge)
                    for envelope in edge.action.message_envelopes:
                        if (process, envelope) not in message_produders:
                            message_produders[(process, envelope)] = list()
                        message_produders[(process, envelope)].append(edge)

        for (producer_process, produced_envelope), producer_edges in message_produders.items():
            for (consumer_process, consumed_message), consumer_edges in message_consumers.items():
                for producer_edge in producer_edges:
                    envelope = producer_process.map_outbound_message(producer_edge, produced_envelope)
                    if not envelope.destination.matches(consumer_process):
                        continue
                    if consumer_process.map_inbound_message(producer_process, envelope.message) != consumed_message:
                        continue
                    if (producer_process, producer_edge) not in self._real_outbound_synchronization_edges:
                        self._real_outbound_synchronization_edges[(producer_process, producer_edge)] = list()
                    for consumer_edge in consumer_edges:
                        if (consumer_process, consumer_edge) not in self._real_inbound_synchronization_edges:
                            self._real_inbound_synchronization_edges[(consumer_process, consumer_edge)] = list()
                        if (producer_process, producer_edge) not in self._real_inbound_synchronization_edges[(consumer_process, consumer_edge)]:
                            self._real_inbound_synchronization_edges[(consumer_process, consumer_edge)].append((producer_process, producer_edge))
                        if (consumer_process, consumer_edge) not in self._real_outbound_synchronization_edges[(producer_process, producer_edge)]:
                            self._real_outbound_synchronization_edges[(producer_process, producer_edge)].append((consumer_process, consumer_edge))

    # For given process "process" in given state "state", determine all
    # synchronization points where it could have inbound synchronized (i.e.
    # received messages from) with another process "synchronized_process"
    def _infer_inbound_synchronized_past_boundary(self, process: int, node: int, synchronized_process: int) -> Iterable[Union[ProcessSynchronizationPoint, ProcessUnsynchronizedPoint]]:
        visited = set()
        found = set()
        queue = [node]
        # Inverse state machine traversal starting from state "node"
        while queue:
            node = queue.pop()
            # Avoiding loops
            if node in visited:
                continue
            visited.add(node)

            for reverse_edge in self._incoming_edges[node]:
                # For each reverse edge, in case it has any triggers, see
                # whether it could have inbound synchronized with
                # "synchronization_process". In case it could, yield a
                # synchronization point.
                found_synchronization = False
                if self._simple_processes.edge_trigger(reverse_edge) is not None:
                    for synchronization_process, synchronization_edge in self._inbound_synchronization_edges[(process, reverse_edge)]:
                        if synchronization_process == synchronized_process:
                            found_synchronization = True
                            boundary = ProcessSynchronizationPoint(
                                edge=reverse_edge,
                                synchronization_process=synchronization_process,
                                synchronization_edge=synchronization_edge
                            )
                            if boundary not in found:
                                found.add(boundary)
                                yield boundary
                # Special case -- we have reached process "process" entry point
                # without synchronizing with "synchronized_process" -- this
                # means that there exist a trace where both processes go
                # unsynchronized from the beginning
                if self._simple_processes.edge_source_state(reverse_edge) == self._simple_processes.process_entry_point(process):
                    yield ProcessUnsynchronizedPoint()

                # We have not found synchronization point, continue reverse
                # traversal from here
                if not found_synchronization:
                    queue.append(self._simple_processes.edge_source_state(reverse_edge))

    def infer_inbound_synchronized_past_boundary(self, process: int, node: int, synchronized_process: int) -> Iterable[Union[ProcessSynchronizationPoint, ProcessUnsynchronizedPoint]]:
        key = (process, node, synchronized_process)
        if key not in self._infer_inbound_synchronized_past_boundary_cache:
            self._infer_inbound_synchronized_past_boundary_cache[key] = list(self._infer_inbound_synchronized_past_boundary(process, node, synchronized_process))
        yield from self._infer_inbound_synchronized_past_boundary_cache[key]

    # For given process "process" in given state "node", determine set of
    # synchronization points where it would require inbound synchronization
    # (i.e. a message from) with process "synchronized_process", assuming that
    # process "synchronized_process" has already reached any state in
    # "synchronized_process_states" by now.
    # 
    # The last assumption is particularly improtant, because process
    # "synchronized_process" reaching a state in "synchronized_process_states"
    # would imply that it may send a message to process "process", thus
    # resolving certain synchronization point already. Such synchronization
    # points shall not be considered.
    def _infer_inbound_synchronized_future_limit(self, process: int, node: int, synchronized_process: int, synchronized_process_outbound_edges) -> Iterable[ProcessSynchronizationPoint]:
        visited = set()
        found = set()
        queue = [node]

        # Forward traversal of process state machine starting from "node"
        while queue:
            node = queue.pop()
            # Avoiding loops
            if node in visited:
                continue
            visited.add(node)

            for edge in self._simple_processes.state_forward_edges(node):                        
                # For each edge, in case it has any triggers, see whether it
                # could have inbound synchronized with
                # "synchronization_process", such that the synchronization is
                # not already resolved by "synchronized_process_states". In case
                # it could, yield a synchronization point.
                found_synchronization = False
                if self._simple_processes.edge_trigger(edge) is not None:
                    for synchronization_process, synchronization_edge in self._inbound_synchronization_edges[(process, edge)]:
                        if synchronization_process == synchronized_process and synchronization_edge not in synchronized_process_outbound_edges:
                            found_synchronization = True
                            boundary = ProcessSynchronizationPoint(
                                edge=edge,
                                synchronization_process=synchronization_process,
                                synchronization_edge=synchronization_edge
                            )
                            if boundary not in found:
                                found.add(boundary)
                                yield boundary
                # There are no special cases for the future limits, as the
                # system is assumed to run indefinetely. We have not found
                # synchronization point, continue traversal from here.
                if not found_synchronization:
                    queue.append(self._simple_processes.edge_target_state(edge))

    def infer_inbound_synchronized_future_limit(self, process: int, node: int, synchronized_process: int, synchronized_process_outbound_edges) -> Iterable[ProcessSynchronizationPoint]:
        key = (process, node, synchronized_process, tuple(sorted(synchronized_process_outbound_edges, key=str)))
        if key not in self._infer_inbound_synchronized_future_limit_cache:
            self._infer_inbound_synchronized_future_limit_cache[key] = list(self._infer_inbound_synchronized_future_limit(process, node, synchronized_process, synchronized_process_outbound_edges))
        yield from self._infer_inbound_synchronized_future_limit_cache[key]

    def _edge_sequenced_after_without_synchronization(self, process: Process, edge: StateGraphEdge, other_edge: StateGraphEdge, synchronized_process: Process) -> bool:       
        visited = set()
        queue = [edge]
        while queue:
            current_edge = queue.pop()
            if current_edge in visited:
                continue
            visited.add(current_edge)

            if current_edge == other_edge:
                return True
            
            requires_synchronization = False
            for synchronization_process, _ in self._inbound_synchronization_edges.get((process, current_edge), ()):
                if synchronization_process == synchronized_process:
                    requires_synchronization = True
            
            if not requires_synchronization:
                for predecessor in self._incoming_edges[current_edge.source]:
                    queue.append(predecessor)
        return False
    
    def edge_sequenced_after_without_synchronization(self, process: Process, edge: StateGraphEdge, preceeding_edge: StateGraphEdge, synchronized_process: Process) -> bool:
        key = (process, edge, preceeding_edge, synchronized_process)
        if key not in self._edge_sequenced_after_without_synchronization_cache:
            self._edge_sequenced_after_without_synchronization_cache[key] = self._edge_sequenced_after_without_synchronization(process, edge, preceeding_edge, synchronized_process)
        return self._edge_sequenced_after_without_synchronization_cache[key]

    def _infer_outbound_boundary(self, process: int, node: int, synchronized_process: int):
        inbound_edge_index = dict()

        queue = [node]
        visited = set()

        result = list()
        while queue:
            node = queue.pop()
            if node in visited:
                continue
            visited.add(node)

            for reverse_edge in self._incoming_edges[node]:
                found_boundary = False
                for synchronization_process, _ in self._outbound_synchronization_edges.get((process, reverse_edge), ()):
                    if synchronization_process == synchronized_process:
                        found_boundary = True
                        result.append((reverse_edge, inbound_edge_index.get(node, set())))
                        break
                
                if found_boundary:
                    continue

                edge_source = self._simple_processes.edge_source_state(reverse_edge)
                if self._simple_processes.edge_trigger(reverse_edge) is not None:
                    for synchronization_process, _ in self._inbound_synchronization_edges[(process, reverse_edge)]:
                        if synchronization_process == synchronized_process:
                            inbound_edge_index[edge_source] = inbound_edge_index.get(edge_source, set()).union(inbound_edge_index.get(node, set()))
                            inbound_edge_index[edge_source].add(reverse_edge)
                
                queue.append(edge_source)
        return result
                
    def infer_outbound_boundary(self, process: int, node: int, synchronized_process: int):
        key = (process, node, synchronized_process)
        if key not in self._infer_outbound_boundary_cache:
            self._infer_outbound_boundary_cache[key] = self._infer_outbound_boundary(process, node, synchronized_process)
        yield from self._infer_outbound_boundary_cache[key]

    def _infer_relevant_outbounds(self, process: int, node: int, synchronized_process: int) -> Iterable[int]:
        for outbound_edge, inbound_edges in self.infer_outbound_boundary(process, node, synchronized_process):
            outbound_synchronization_edges = (
                synchronization_outbound_edge
                for synchronization_process, synchronization_outbound_edge in self._outbound_synchronization_edges.get((process, outbound_edge), ())
                if synchronization_process == synchronized_process
            )
            found_inbound_boundary = -1
            for outbound_synchronization_edge in outbound_synchronization_edges:
                found_any_inbound_boundary = False
                for inbound_edge_idx, inbound_edge in enumerate(inbound_edges):
                    outbound_sync_before_all_inbound_sync = True
                    inbound_synchronization_edges = (
                        synchronization_inbound_edge
                        for synchronization_process, synchronization_inbound_edge in self._inbound_synchronization_edges.get((process, inbound_edge), ())
                        if synchronization_process == synchronized_process
                    )
                    for inbound_synchronization_edge in inbound_synchronization_edges:
                        if self.edge_sequenced_after_without_synchronization(synchronized_process, outbound_synchronization_edge, inbound_synchronization_edge, process):
                            outbound_sync_before_all_inbound_sync = False
                    if outbound_sync_before_all_inbound_sync:
                        found_any_inbound_boundary = True
                        found_inbound_boundary = max(found_inbound_boundary, inbound_edge_idx)
                if not found_any_inbound_boundary:
                    found_inbound_boundary = -1
                    break

            if found_inbound_boundary == -1:
                yield outbound_edge

    def infer_relevant_outbounds(self, process: int, node: int, synchronized_process: int) -> Iterable[int]:
        key = (process, node, synchronized_process)
        if key not in self._infer_relevant_outbounds_cache:
            self._infer_relevant_outbounds_cache[key] = list(self._infer_relevant_outbounds(process, node, synchronized_process))
        yield from self._infer_relevant_outbounds_cache[key]

    def _infer_concurrent_segment(self, target_process: int, synchronized_process: int, synchronized_state: int) -> Iterable[int]:
        relevant_outbounds = set(self.infer_relevant_outbounds(synchronized_process, synchronized_state, target_process))

        target_past_boundary = set()
        for boundary in self.infer_inbound_synchronized_past_boundary(synchronized_process, synchronized_state, target_process):
            if isinstance(boundary, ProcessSynchronizationPoint):
                target_past_boundary.add(self._simple_processes.edge_target_state(boundary.synchronization_edge))
            else:
                target_past_boundary.add(self._simple_processes.process_entry_point(target_process))

        if not target_past_boundary:
            target_past_boundary = set(self._simple_processes.process_states(target_process))

        target_future_limit = {
            future_limit.edge
            for past_boundary in target_past_boundary
            for future_limit in self.infer_inbound_synchronized_future_limit(target_process, past_boundary, synchronized_process, relevant_outbounds)
        }

        queue = [*target_past_boundary]
        visited = set()
        while queue:
            current_synchronized_state = queue.pop()
            if current_synchronized_state in visited:
                continue
            visited.add(current_synchronized_state)

            yield current_synchronized_state
            for edge in self._simple_processes.state_forward_edges(current_synchronized_state):
                if edge not in target_future_limit:
                    queue.append(self._simple_processes.edge_target_state(edge))

    def infer_concurrent_segment(self, target_process: int, synchronized_process: int, synchronized_state: int) -> Iterable[int]:
        key = (target_process, synchronized_process, synchronized_state)
        if key not in self._infer_concurrent_segment_cache:
            self._infer_concurrent_segment_cache[key] = list(self._infer_concurrent_segment(target_process, synchronized_process, synchronized_state))
        yield from self._infer_concurrent_segment_cache[key]

    def infer_concurrent_space(self, process: Process, node: StateGraphNode) -> Dict[Process, Set[StateGraphNode]]:
        process_id = self._simple_processes._get_process_id(process)
        node_id = self._simple_processes._get_state_id(node)

        concurrent_space = {
            p: set(self._simple_processes.process_states(p)) if p != process_id else {node_id}
            for p in self._simple_processes.processes
        }

        fixpoint_reached = False
        while not fixpoint_reached:
            fixpoint_reached = True
            for synchronized_process in self._simple_processes.processes:
                for target_process in self._simple_processes.processes:
                    if synchronized_process != target_process:
                        concurrent_segment = set()
                        for synchronized_state in concurrent_space[synchronized_process]:
                            concurrent_segment = concurrent_segment.union(self.infer_concurrent_segment(target_process, synchronized_process, synchronized_state))
                        concurrent_segment = concurrent_segment.intersection(concurrent_space[target_process])
                        if concurrent_segment.symmetric_difference(concurrent_space[target_process]):
                            concurrent_space[target_process] = concurrent_segment
                            fixpoint_reached = False
        return {
            self._simple_processes.get_real_process(p): {
                self._simple_processes.get_real_state(s)
                for s in ss
            }
            for p, ss in concurrent_space.items()
        }
