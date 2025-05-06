import dataclasses
from typing import Iterable, Dict, Set
from harness.core import ProcessSet, Process, StateGraphNode, StateGraphEdge

@dataclasses.dataclass
class ProcessSynchronizationPoint:
    edge: StateGraphEdge
    synchronization_process: Process
    synchronization_edge: StateGraphEdge

    def __hash__(self):
        return 31 * hash(self.edge) + 17 * hash(self.synchronization_process) + 7 * hash(self.synchronization_edge)
    
    def __eq__(self, value):
        return isinstance(value, ProcessSynchronizationPoint) and \
            value.edge == self.edge and \
            value.synchronization_process == self.synchronization_process and \
            value.synchronization_edge == self.synchronization_edge
    
    def __ne__(self, value):
        return not self.__eq__(value)

class ProcessSetAnalyzer:
    def __init__(self, processes: ProcessSet):
        self._processes = processes
        self._incoming_edges = dict()
        self._synchronization_edges = dict()
        self._scan_process_set()

    @property
    def processes(self) -> ProcessSet:
        return self._processes
    
    def incoming_edges_for(self, node: StateGraphNode) -> Iterable[StateGraphEdge]:
        yield from self._incoming_edges.get(node, ())

    def synchronization_edges_for(self, edge: StateGraphEdge) -> Iterable[StateGraphEdge]:
        yield from self._synchronization_edges.get(edge, ())

    def _scan_process_set(self):
        message_produders = dict()
        message_consumers = dict()

        for process in self.processes.processes:
            for node in process.entry_node.all_nodes:
                for edge in node.edges:
                    if edge.target not in self._incoming_edges:
                        self._incoming_edges[edge.target] = list()
                    if edge not in self._incoming_edges[edge.target]:
                        self._incoming_edges[edge.target].append(edge)
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
                    for consumer_edge in consumer_edges:
                        if (consumer_process, consumer_edge) not in self._synchronization_edges:
                            self._synchronization_edges[(consumer_process, consumer_edge)] = list()
                        if (producer_process, producer_edge) not in self._synchronization_edges[(consumer_process, consumer_edge)]:
                            self._synchronization_edges[(consumer_process, consumer_edge)].append((producer_process, producer_edge))


    def infer_past_boundary(self, process: Process, node: StateGraphNode) -> Iterable[ProcessSynchronizationPoint]:
        visited = set()
        found = set()
        queue = [node]
        while queue:
            node = queue.pop()
            if node in visited:
                continue
            visited.add(node)

            for incoming_edge in self._incoming_edges[node]:
                if incoming_edge.trigger is not None:
                    for synchronization_process, synchronization_edge in self._synchronization_edges[(process, incoming_edge)]:
                        boundary = ProcessSynchronizationPoint(
                            edge=incoming_edge,
                            synchronization_process=synchronization_process,
                            synchronization_edge=synchronization_edge
                        )
                        if boundary not in found:
                            found.add(boundary)
                            yield boundary
                else:
                    queue.append(incoming_edge.source)

    def infer_future_limit(self, process: Process, node: StateGraphNode) -> Iterable[ProcessSynchronizationPoint]:
        visited = set()
        found = set()
        queue = [node]

        while queue:
            node = queue.pop()
            if node in visited:
                continue
            visited.add(node)

            for edge in node.edges:
                if edge.trigger is not None:
                    for synchronization_process, synchronization_edge in self._synchronization_edges[(process, edge)]:
                        boundary = ProcessSynchronizationPoint(
                            edge=edge,
                            synchronization_process=synchronization_process,
                            synchronization_edge=synchronization_edge
                        )
                        if boundary not in found:
                            found.add(boundary)
                            yield boundary
                else:
                    queue.append(edge.target)

    def infer_partial_future_limit(self, process: Process, node: StateGraphNode, concurrent_process: Process, concurrent_process_states: Iterable[StateGraphNode]) -> Iterable[ProcessSynchronizationPoint]:
        queue = [node]
        visited = set()
        synchronized_states = set(concurrent_process_states)
        while queue:
            node = queue.pop()
            if node in visited:
                continue
            visited.add(node)
            for future_limit in self.infer_future_limit(process, node):
                if future_limit.synchronization_process == concurrent_process and future_limit.synchronization_edge.target not in synchronized_states:
                    yield future_limit
                else:
                    queue.append(future_limit.edge.target)

    def infer_partially_bound_past(self, process: Process, node: StateGraphNode, concurrent_process: Process) -> Iterable[StateGraphNode]:
        queue = [node]
        visited = set()
        while queue:
            current_node = queue.pop()
            if current_node in visited:
                continue
            visited.add(current_node)

            yield current_node
            for incoming_edge in self.incoming_edges_for(current_node):
                found_outbound_msg = False
                for envelope in incoming_edge.action.message_envelopes:
                    envelope = process.map_outbound_message(incoming_edge, envelope)
                    if envelope.destination.matches(concurrent_process):
                            found_outbound_msg = True
                            break
                if not found_outbound_msg:
                    queue.append(incoming_edge.source)

    def infer_concurrent_segment(self, process: Process, node: StateGraphNode, concurrent_process: Process) -> Iterable[StateGraphNode]:
        own_bounded_past = list(self.infer_partially_bound_past(process, node, concurrent_process))

        concurrent_past_boundary = list()
        for boundary in self.infer_past_boundary(process, node):
            if boundary.synchronization_process == concurrent_process:
                concurrent_past_boundary.append(boundary.synchronization_edge.target)

        if not concurrent_past_boundary:
            concurrent_past_boundary = list(concurrent_process.entry_node.all_nodes)

        concurrent_future_limit = {
            future_limit.edge
            for past_boundary in concurrent_past_boundary
            for future_limit in self.infer_partial_future_limit(concurrent_process, past_boundary, process, own_bounded_past)
        }

        queue = [*concurrent_past_boundary]
        visited = set()
        while queue:
            node = queue.pop()
            if node in visited:
                continue
            visited.add(node)

            yield node
            for edge in node.edges:
                if edge not in concurrent_future_limit:
                    queue.append(edge.target)

    def infer_concurrent_space(self, process: Process, node: StateGraphNode) -> Dict[Process, Set[StateGraphNode]]:
        concurrent_space = {
            p: set(p.entry_node.all_nodes) if p != process else {node}
            for p in self.processes.processes
        }

        fixpoint_reached = False
        while not fixpoint_reached:
            fixpoint_reached = True
            for process1 in self.processes.processes:
                for process2 in self.processes.processes:
                    if process1 != process2:
                        concurrent_segment = set()
                        for state in concurrent_space[process1]:
                            concurrent_segment = concurrent_segment.union(self.infer_concurrent_segment(process1, state, process2))
                        concurrent_segment = concurrent_segment.intersection(concurrent_space[process2])
                        if concurrent_segment.symmetric_difference(concurrent_space[process2]):
                            concurrent_space[process2] = concurrent_segment
                            fixpoint_reached = False
        return concurrent_space
