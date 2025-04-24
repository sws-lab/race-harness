import io
from typing import Optional, Dict, Iterable
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
        self._invariants = list()

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
        for index, _ in enumerate(self._invariants):
            yield f'static __harness_mutex harness_kernel_module_invariant_{index};'
        if self._invariants:
            yield ''

        def get_invariants(process: Process, state: StateGraphNode) -> Iterable[str]:
            for index, invariant in enumerate(self._invariants):
                if (invariant.process == process and invariant.state == state) or \
                    invariant.invariant_process == process and state not in invariant:
                    yield f'harness_kernel_module_invariant_{index}'

        for process, (process_template, process_specialization) in self._processes.items():
            yield from process_template.generate(process, process_specialization, get_invariants)
            yield ''

        yield 'int main(void) {'
        yield 1
        for index, _ in enumerate(self._processes.keys()):
            yield f'__harness_thread process{index};'
        for index, _ in enumerate(self._invariants):
            yield f'__harness_mutex_init(&harness_kernel_module_invariant_{index});'
        yield ''
        for index, process in enumerate(self._processes.keys()):
            yield f'__harness_thread_create(&process{index}, NULL, {KernelModuleHarnessProcessTemplate.process_function_name(process)}, NULL);'
        yield ''
        for index, _ in enumerate(self._processes.keys()):
            yield f'__harness_thread_join(&process{index}, NULL);'
        yield ''
        yield 'return 0;'
        yield -1
        yield '}'
