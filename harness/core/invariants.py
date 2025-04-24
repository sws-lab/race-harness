from typing import Iterable
from harness.core import Process, ProcessSetState, StateGraphNode

class ProcessStateInvariant:
    def __init__(self, process: Process, state: StateGraphNode, invariant_process: Process, invariant_set: Iterable[StateGraphNode]):
        self._process = process
        self._state = state
        self._invariant_process = invariant_process
        self._invariant_set = { # Using dictionary to stabilize order
            invariant: invariant
            for invariant in invariant_set
        }

    @property
    def process(self) -> Process:
        return self._process
    
    @property
    def state(self) -> StateGraphNode:
        return self._state
    
    @property
    def invariant_process(self) -> Process:
        return self._invariant_process
    
    @property
    def invariant_set(self) -> Iterable[StateGraphNode]:
        yield from self._invariant_set.keys()

    @property
    def is_empty(self) -> bool:
        return not bool(self._invariant_set)

    def __contains__(self, value) -> bool:
        return value in self._invariant_set

    def __str__(self):
        return '{}: {} => {}: {{{}}}'.format(
            self.process.mnemonic,
            self.state.mnemonic,
            self.invariant_process.mnemonic,
            '; '.join(
                invariant.mnemonic
                for invariant in self.invariant_set
            )
        )
    
    @staticmethod
    def derive(psstates: Iterable[ProcessSetState], process: Process, state: StateGraphNode, invariant_process: Process) -> 'ProcessStateInvariant':
        invariant_set = (
            psstate.process_state(invariant_process).state
            for psstate in psstates
            if psstate.process_state(process).state == state
        )
        return ProcessStateInvariant(process=process, state=state, invariant_process=invariant_process, invariant_set=invariant_set)
