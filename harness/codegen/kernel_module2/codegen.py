import io
from typing import Iterable, Union, Tuple
from harness.core import ProcessSet, Process, StateGraphNode, ProcessStateInvariant, StateGraphAction

class KernelModuleHarnessGenerator:
    def __init__(self, process_set: ProcessSet):
        self._process_set = process_set
        self._actions = dict()
        self._invariants = list()

    def define_action(self, action: StateGraphAction, content: str) -> 'KernelModuleHarnessGenerator':
        self._actions[action] = content
        return self

    def add_invariant(self, invariant: ProcessStateInvariant) -> 'KernelModuleHarnessGenerator':
        self._invariants.append(invariant)

    def generate(self, indent: int = 2) -> str:
        out = io.StringIO()
        indent_level = 0
        for line in self._generate():
            if isinstance(line, int):
                indent_level += line
            else:
                out.write(' ' * indent * indent_level)
                out.write(line)
                out.write('\n')
        
        return out.getvalue()

    def _generate(self) -> Iterable[Union[int, str]]:
        state_enumeration = {
            state: index
            for process in self._process_set.processes
            for index, state in enumerate(process.entry_node.all_nodes)
        }
        # process_enumeration = {
        #     process: index
        #     for index, process in enumerate(self._process_set.processes)
        # }

        def get_invariants(process: Process, state: StateGraphNode) -> Iterable[Tuple[Process, StateGraphNode]]:
            for invariant in self._invariants:
                if invariant.process == process and invariant.state == state:
                    for invariant_state in invariant.invariant_set:
                        yield invariant.invariant_process, invariant_state
                elif invariant.invariant_process == process and state in invariant.invariant_set:
                    yield invariant.process, invariant.state

        yield 'void *process_noop(void *arg) {'
        yield 1
        yield '(void) arg; // Unused'
        yield 'return NULL;'
        yield -1
        yield '}'
        yield ''

        for action, action_content in self._actions.items():
            yield f'void *action_{action.mnemonic}(void *arg) {{'
            yield 1
            yield '(void) arg; // Unused'
            yield ''

            for line in action_content.split('\n'):
                if line.strip():
                    yield line
            yield ''
            
            yield 'return NULL;'
            yield -1
            yield '}'
            yield ''

        yield 'int main(void) {'
        yield 1

        for process in self._process_set.processes:
            yield f'unsigned long process_{process.mnemonic}_state = {state_enumeration[process.entry_node]};'
        yield ''

        for process in self._process_set.processes:
            yield f'__harness_thread process_{process.mnemonic}[{len(list(process.entry_node.all_nodes))}];'
        yield ''

        for process in self._process_set.processes:
            yield f'__harness_thread_create(&process_{process.mnemonic}[{state_enumeration[process.entry_node]}], NULL, process_noop, NULL);'
        yield ''

        yield 'for (;;) {'
        yield 1
        yield f'const unsigned int schedule_process = __harness_random % {len(self._process_set.processes)};'
        yield 'unsigned long next_state;'
        yield '__goblint_split_begin(schedule_process);'
        yield 'switch (schedule_process) {'
        yield 1

        for process_index, process in enumerate(self._process_set.processes):
            yield f'case {process_index}: /* {process.mnemonic} */'
            yield 1
            yield f'__goblint_split_begin(process_{process.mnemonic}_state);'
            yield f'switch (process_{process.mnemonic}_state) {{'
            yield 1

            for state in process.entry_node.all_nodes:
                yield f'case {state_enumeration[state]}: /* {state.mnemonic} */'
                yield 1

                edges = list(state.edges)
                yield f'next_state = __harness_random % {len(edges)};'
                yield f'__goblint_split_begin(next_state);'
                yield f'switch (next_state) {{'
                yield 1
                for edge_index, edge in enumerate(edges):
                    yield f'case {edge_index}: /* {edge.target.mnemonic} */'
                    yield 1

                    invariants = dict()
                    for invariant_process, invariant_state in get_invariants(process, edge.target):
                        if invariant_process not in invariants:
                            invariants[invariant_process] = set()
                        invariants[invariant_process].add(invariant_state)
                    transition_guard = ' && '.join(
                        '({})'.format(
                            ' || '.join(
                                f'process_{process}_state == {state_enumeration[state]}'
                                for state in states
                            )
                        )
                        for process, states in invariants.items()
                    )
                    yield f'if ({transition_guard}) {{'
                    yield 1
                    yield f'__harness_thread_join(process_{process.mnemonic}[{state_enumeration[edge.source]}], NULL);'
                    yield f'process_{process.mnemonic}_state = {state_enumeration[edge.target]};'
                    if edge.action in self._actions:
                        yield f'__harness_thread_create(&process_{process.mnemonic}[{state_enumeration[edge.target]}], NULL, action_{edge.action.mnemonic}, NULL);'
                    else:
                        yield f'__harness_thread_create(&process_{process.mnemonic}[{state_enumeration[edge.target]}], NULL, process_noop, NULL);'
                    yield -1
                    yield '}'

                    yield 'break;'
                    yield -1
                    yield ''
                yield -1
                yield '}'
                yield f'__goblint_split_end(next_state);'
                yield 'break;'
                yield -1
                yield ''

            yield -1
            yield '}'
            yield f'__goblint_split_end(process_{process.mnemonic}_state);'
            yield 'break;'
            yield -1
            yield ''

        yield -1
        yield '}'
        yield '__goblint_split_end(schedule_process);'
        yield -1
        yield '}'

        yield 'return 0;'
        yield -1
        yield '}'