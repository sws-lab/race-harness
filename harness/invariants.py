from typing import Iterable, Optional
from harness.state_graph import StateGraphNode
from harness.process import Process
from harness.process_set import ProcessSetState
from harness.graph_nodes import StateGraphDerivedNode, StateGraphProductNode, StateGraphPlaceholderNode

def filter_process_set_states_for(process: Process, state: StateGraphNode, invariant_process: Process, trace: Iterable[ProcessSetState]) -> Iterable[ProcessSetState]:
    for psstate in trace:
        if psstate.process_state(process).state == state:
            yield psstate.process_state(invariant_process).state

def find_invariant(state: StateGraphNode, other: StateGraphNode) -> Optional[StateGraphNode]:
    if state == other:
        return state
    
    if type(state) != type(other):
        return StateGraphPlaceholderNode()
    
    if type(state) == StateGraphDerivedNode:
        if state.mnemonic_prefix != other.mnemonic_prefix:
            return StateGraphPlaceholderNode()
        base = find_invariant(state.base, other.base)
        if base == StateGraphPlaceholderNode():
            return StateGraphPlaceholderNode()
        else:
            return state.rebase(base)
        
    if type(state) == StateGraphProductNode:
        if len(state.subnodes) != len(other.subnodes):
            return StateGraphPlaceholderNode()
        subnodes = [
            find_invariant(subnode, other_subnode)
            for subnode, other_subnode in zip(state.subnodes, other.subnodes)
        ]
        if any(subnode != StateGraphPlaceholderNode() for subnode in subnodes):
            return StateGraphProductNode(subnodes)
        else:
            return StateGraphPlaceholderNode()
        
    return StateGraphPlaceholderNode()

def derive_invariant_for(process: Process, state: StateGraphNode, invariant_process: Process, trace: Iterable[ProcessSetState]) -> Optional[StateGraphNode]:
    has_invariant = False
    invariant = None
    for state in filter_process_set_states_for(process, state, invariant_process, trace):
        if not has_invariant:
            invariant = state
            has_invariant = True
            continue

        invariant = find_invariant(state, invariant)
        if invariant == StateGraphPlaceholderNode():
            break
    return invariant
