import io
from typing import Optional, Dict, Iterable, Tuple, List
from harness.core import ProcessSet, Process, ProcessStateInvariant, StateGraphNode
from harness.codegen.error import HarnessCodegenError
from harness.codegen.kernel_module.process_template import KernelModuleHarnessProcessTemplate
from harness.codegen.kernel_module.utils import IndentedLineGenerator, IndentedLine

class KernelModuleHarnessGenerator:
    def __init__(self, process_set: ProcessSet):
        self._process_set = process_set
        self._processes = {
            process: (None, None)
            for process in self._process_set
        }
        self._invariants: List[ProcessStateInvariant] = list()

    @property
    def process_set(self) -> ProcessSet:
        return self._process_set
    
    def set_process_template(self, process: Process, template: KernelModuleHarnessProcessTemplate, specialization: Optional[Dict] = None) -> 'KernelModuleHarnessGenerator':
        if process not in self.process_set:
            raise HarnessCodegenError(f'Process {process} is not found in kernel module harness generator process set')
        
        if not template.matches(process):
            raise HarnessCodegenError(f'Process {process} does not match the template')

        self._processes[process] = (template, specialization)
        return self
    
    def add_invariant(self, invariant: ProcessStateInvariant) -> 'KernelModuleHarnessGenerator':
        self._invariants.append(invariant)
        return self
    
    def generate(self, indent: int = 2) -> str:
        for process, (process_template, process_specialization) in self._processes.items():
            if process_template is None:
                raise HarnessCodegenError(f'Process {process} has no template defined')
            
        out = io.StringIO()
        indent_level = 0
        for line in self._generate():
            if isinstance(line, IndentedLine):
                indent_level += line.relative_indent
                if line.line is not None:
                    out.write(' ' * indent * indent_level)
                    out.write(line.line)
                    out.write('\n')
            elif isinstance(line, int):
                indent_level += line
            else:
                out.write(' ' * indent * indent_level)
                out.write(line)
                out.write('\n')
        return out.getvalue()
    
    def _generate(self) -> IndentedLineGenerator:
        process_states = {
            state: index
            for process in self._process_set.processes
            for index, state in enumerate(process.entry_node.all_nodes)
        }
        def get_invariants(process: Process, state: StateGraphNode) -> Iterable[Tuple[Process, StateGraphNode]]:
            for invariant in self._invariants:
                if invariant.process == process and invariant.state == state:
                    for invariant_state in invariant.invariant_set:
                        yield invariant.invariant_process, invariant_state
                elif invariant.invariant_process == process and state in invariant.invariant_set:
                    yield invariant.process, invariant.state

        yield 'static __harness_mutex harness_state_mutex;'
        yield ''

        for process in self._process_set.processes:
            yield f'static unsigned long process_{process.mnemonic}_state;'
        yield ''

        for process, (process_template, process_specialization) in self._processes.items():
            yield from process_template.generate(process, process_specialization, get_invariants, lambda state: process_states[state])
            yield ''

        yield 'int main(void) {'
        yield 1
        
        yield f'__harness_mutex_init(&harness_state_mutex, NULL);'
        for process in self._process_set.processes:
            yield f'process_{process.mnemonic}_state = {process_states[process.entry_node]};'
        yield ''

        for process in self._process_set.processes:
            yield f'__harness_thread process_{process.mnemonic};'
        yield ''

        for process in self._process_set.processes:
            yield f'__harness_thread_create(&process_{process.mnemonic}, NULL, {KernelModuleHarnessProcessTemplate.process_function_name(process)}, NULL);'
        yield ''
        for process in self._process_set.processes:
            yield f'__harness_thread_join(process_{process.mnemonic}, NULL);'
        
        yield 'return 0;'
        yield -1
        yield '}'
