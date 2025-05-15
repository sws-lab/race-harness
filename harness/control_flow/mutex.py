from typing import Iterable, Optional
from harness.core import ProcessMutualExclusionSegment, StateGraphNode, Process

class ControlFlowMutex:
    def __init__(self, identifier: int, segment: ProcessMutualExclusionSegment):
        self._identifier = identifier
        self._segment = segment

    @property
    def identifier(self) -> int:
        return self._identifier
    
    @property
    def segment(self) -> ProcessMutualExclusionSegment:
        return self._segment
    
    def __eq__(self, value):
        return isinstance(value, ControlFlowMutex) and value.identifier == self.identifier
    
    def __ne__(self, value):
        return not self.__eq__(value)
    
    def __hash__(self):
        return hash(self.identifier)
    
class ControlFlowMutexSet:
    def __init__(self, segments: Iterable[ProcessMutualExclusionSegment]):
        self._mutexes = dict()
        for segment in segments:
            if segment not in self._mutexes:
                self._mutexes[segment] = ControlFlowMutex(len(self._mutexes), segment)
    
    @property
    def mutexes(self) -> Iterable[ControlFlowMutex]:
        yield from self._mutexes.values()

    def mutex_for(self, segment: ProcessMutualExclusionSegment) -> Optional[ControlFlowMutex]:
        return self._mutexes.get(segment, None)
    
    def locked_in_state(self, process: Process, state: StateGraphNode) -> Iterable[ControlFlowMutex]:
        for segment, mutex in self._mutexes.items():
            if segment.has(process, state):
                yield mutex

