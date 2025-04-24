from typing import Callable, Union, Optional, Dict, Iterable
from harness.core import StateGraphNode, Process, StateGraphAction, StateGraphEdge
from harness.codegen.kernel_module.utils import IndentedLineGenerator, IndentedLine
from harness.codegen.error import HarnessCodegenError

class KernelModuleHarnessProcessTemplate:
    def __init__(self, entry_node: StateGraphNode, initializer: str = ''):
        self._entry_node = entry_node
        self._initializer = initializer
        self._node_enumeration = {
            node: index
            for index, node in enumerate(entry_node.all_nodes)
        }
        self._actions = dict()
    
    def matches(self, process: Process) -> bool:
        return process.entry_node == self._entry_node
    
    def define_action(self, action: StateGraphAction, content: Union[str, Callable[[StateGraphEdge], str]]) -> 'KernelModuleHarnessProcessTemplate':
        self._actions[action] = content
        return self
    
    def set_initializer(self, initializer: str) -> 'KernelModuleHarnessProcessTemplate':
        self._initializer = initializer
        return self
    
    def _random(self, max: int) -> str:
        return f'__harness_random % {max}'
    
    @staticmethod
    def process_function_name(process: Process) -> str:
        return f'harness_kernel_module_process_{process.mnemonic}'
    
    def generate(self, process: Process, specialization: Optional[Dict], invariants_getter: Callable[[Process, StateGraphNode], Iterable[str]]) -> IndentedLineGenerator:
        if not self.matches(process):
            raise HarnessCodegenError(f'Process {process.mnemonic} does not match the template')

        yield f'void *{KernelModuleHarnessProcessTemplate.process_function_name(process)}(void *harness_kernel_module_process_arg) {{'
        yield 1
        yield '(void) harness_kernel_module_process_arg; // UNUSED'
        if self._initializer:
            for line in self._initializer.split('\n'):
                if line.strip():
                    line = self._apply_specialization(line, specialization)
                    yield line
        yield f'unsigned long harness_kernel_module_process_state = {self._node_enumeration[self._entry_node]};'
        yield 'for (;;) {'
        yield 1
        yield 'switch (harness_kernel_module_process_state) {'
        yield 1
        for node, node_state_index in self._node_enumeration.items():
            yield f'case {node_state_index}: /* {node.mnemonic} */'
            yield 1
            invariants = list(invariants_getter(process, node))
            for invariant in invariants:
                yield f'__harness_mutex_lock(&{invariant});'
            node_edges = list(node.edges)
            yield f'switch ({self._random(len(node_edges))}) {{'
            yield 1
            for edge_index, edge in enumerate(node_edges):
                yield f'case {edge_index}:'
                yield 1
                action = self._actions.get(edge.action, None)
                if action is not None:
                    if not isinstance(action, str):
                        action = action(edge)
                    action = self._apply_specialization(action, specialization)
                    yield '{'
                    yield 1
                    for line in action.split('\n'):
                        if line.strip():
                            yield line
                    yield IndentedLine(relative_indent=-1, line='}')
                yield f'harness_kernel_module_process_state = {self._node_enumeration[edge.target]}; /* {edge.target.mnemonic} */'
                yield 'break;'
                yield IndentedLine(relative_indent=-1, line='')
            yield IndentedLine(relative_indent=-1, line='}')
            for invariant in reversed(invariants):
                yield f'__harness_mutex_unlock(&{invariant});'
            yield 'break;'
            yield IndentedLine(relative_indent=-1, line='')
        yield IndentedLine(relative_indent=-1, line='}')
        yield -1
        yield '}'
        yield 'return NULL;'
        yield IndentedLine(relative_indent=-1, line='}')

    def _apply_specialization(self, action: str, specialization: Optional[Dict]) -> str:
        if specialization is None:
            return action
        
        for key, value in specialization.items():
            action = action.replace(f'%{key}%', str(value))
        return action
