import abc
import io
from typing import Dict, Iterable, Union, Any
from harness.core import Process, StateGraphAction
from harness.control_flow import ControlFlowNode, ControlFlowMutexSet, ControlFlowStatement, ControlFlowLabelledNode, ControlFlowLabel, ControlFlowSequence, ControlFlowBranchNode, ControlFlowGotoNode, ControlFlowSynchronization, ControlFlowMutex
from harness.codegen.error import HarnessCodegenError

IntOrStrIterable = Iterable[Union[int, str]]
NoNewline = object()

class HarnessControlFlowBaseCodegen(abc.ABC):
    def __init__(self, *, indent: str = '  '):
        self._indent = indent
        self._global_prologue = None
        self._process_parameters = dict()
        self._process_prologues = dict()
        self._actions = dict()

    def set_global_prologue(self, prologue: str):
        self._global_prologue = prologue

    def set_process_parameters(self, process: Process, parameters: Dict[str, Any]):
        self._process_parameters[process] = parameters

    def set_process_prologue(self, process: Process, prologue: str):
        self._process_prologues[process] = prologue

    def set_action(self, action: StateGraphAction, content: str):
        self._actions[action] = content

    def format(self, processes: Dict[Process, ControlFlowNode], mutexes: ControlFlowMutexSet) -> str:
        out = io.StringIO()
        indent_level = 0
        no_newline = False
        first_line = True
        for entry in self._format(processes, mutexes):
            if isinstance(entry, int):
                indent_level += entry
            elif entry is NoNewline:
                no_newline = True
            else:
                if not no_newline:
                    if not first_line:
                        out.write('\n')
                    out.write(self._indent * indent_level)
                out.write(entry)
                no_newline = False
            first_line = False
        return out.getvalue()
    
    def _embed_multiline(self, content: str) -> IntOrStrIterable:
        for line in content.split('\n'):
            yield line

    def _parameterize_template(self, process: Process, content: str) -> str:
        if process in self._process_parameters:
            for key, value in self._process_parameters[process].items():
                content = content.replace(f'%{key}%', str(value))
        return content

    def _format(self, processes: Dict[Process, ControlFlowNode], mutexes: ControlFlowMutexSet) -> IntOrStrIterable:
        yield from self._prologue()
        yield ''

        if self._global_prologue is not None:
            yield from self._embed_multiline(self._global_prologue)
            yield ''

        yield from self._declare_barrier(f'process_set_start_barrier')
        for mutex in mutexes.mutexes:
            yield '/* {} */'.format(
                ', '.join(
                    f'{process.mnemonic}: {state.mnemonic}'
                    for process, state in mutex.segment
                )
            )
            yield from self._declare_mutex(self._mutex_name(mutex))
            yield ''
        yield ''

        for process, root_node in processes.items():
            yield from self._open_process_definition(process)
            yield ''
            if process in self._process_prologues:
                yield from self._embed_multiline(self._parameterize_template(process, self._process_prologues[process]))
                yield ''
            label_map = dict()
            yield from self._format_node(process, mutexes, root_node, label_map)
            yield ''
            yield from self._close_process_definition(process)
            yield ''

        yield ''
        yield from self._open_main_definition()

        for process in processes.keys():
            yield from self._declare_process(process)
        yield ''

        yield from self._initialize_barrier('process_set_start_barrier', len(processes))
        for mutex in mutexes.mutexes:
            yield from self._initialize_mutex(self._mutex_name(mutex))
        yield ''

        for process in processes.keys():
            yield from self._start_process(process)
        yield ''
        
        for process in processes.keys():
            yield from self._join_process(process)
        yield ''

        yield from self._close_main_definition()

    def _format_node(self, process: Process, mutexes: ControlFlowMutexSet, node: ControlFlowNode, label_map: Dict[ControlFlowLabel, str]) -> IntOrStrIterable:
        if node.as_statement():
            yield from self._format_statement(process, node.as_statement())
        elif node.as_labelled_node():
            yield from self._format_labelled_node(process, mutexes, node.as_labelled_node(), label_map)
        elif node.as_sequence():
            yield from self._format_sequence_node(process, mutexes, node.as_sequence(), label_map)
        elif node.as_branch():
            yield from self._format_branch(process, mutexes, node.as_branch(), label_map)
        elif node.as_goto():
            yield from self._format_goto(node.as_goto(), label_map)
        elif node.as_synchronization():
            yield from self._format_synchronization(node.as_synchronization())
        elif node.as_init_barrier():
            yield from self._format_init_barrier()
        else:
            raise HarnessCodegenError(f'Unknown control flow node type {type(node)}')
        
    def _get_label(self, label: ControlFlowLabel, label_map: Dict[ControlFlowLabel, str]) -> str:
        if label not in label_map:
            label_map[label] = f'label{len(label_map)}'
        return label_map[label]
        
    def _format_statement(self, process: Process, statement: ControlFlowStatement) -> IntOrStrIterable:
        yield f'/* {statement.state_graph_edge} */'
        if statement.state_graph_edge.action in self._actions:
            yield from self._embed_multiline(self._parameterize_template(process, self._actions[statement.state_graph_edge.action]))

    def _format_labelled_node(self, process: Process, mutexes: ControlFlowMutexSet, labelled_node: ControlFlowLabelledNode, label_map: Dict[ControlFlowLabel, str]) -> IntOrStrIterable:
        label_name = self._get_label(labelled_node.label, label_map)
        entries = self._format_node(process, mutexes, labelled_node.body, label_map)
        first = next(entries, None)
        if first is None:
            yield f'{label_name} /* {labelled_node.label.node.mnemonic} */: ;'
        elif isinstance(first, str):
            yield f'{label_name} /* {labelled_node.label.node.mnemonic} */: {first}'
            yield from entries
        else:
            yield f'{label_name} /* {labelled_node.label.node.mnemonic} */: ;'
            yield from entries

    def _format_sequence_node(self, process: Process, mutexes: ControlFlowMutexSet, sequence: ControlFlowSequence, label_map: Dict[ControlFlowLabel, str]) -> IntOrStrIterable:
        yield '{'
        yield 1
        for entry in sequence.sequence:
            yield from self._format_node(process, mutexes, entry, label_map)
        yield -1
        yield '}'

    def _format_branch(self, process: Process, mutexes: ControlFlowMutexSet, branch: ControlFlowBranchNode, label_map: Dict[ControlFlowLabel, str]) -> IntOrStrIterable:
        branches = list(branch.branches)
        for index, branch_node in enumerate(branches):
            if index == 0:
                yield f'if ({self._random(len(branches))} == 0) '
                yield NoNewline
            yield from self._format_node(process, mutexes, branch_node, label_map)
            if index + 2 < len(branches):
                yield NoNewline
                yield f' else if ({self._random(len(branches) - index)} == 0) '
                yield NoNewline
            elif index + 1 < len(branches):
                yield NoNewline
                yield ' else '
                yield NoNewline

    def _format_goto(self, goto: ControlFlowGotoNode, label_map: Dict[ControlFlowLabel, str]) -> IntOrStrIterable:
        label_name = self._get_label(goto.label, label_map)
        yield f'goto {label_name}; /* {goto.label.node.mnemonic} */'

    def _format_synchronization(self, synchonization: ControlFlowSynchronization) -> IntOrStrIterable:
        lock = sorted(
            (
                mtx
                for mtx in synchonization.lock
            ), key=lambda mtx: mtx.identifier
        )
        unlock = reversed(sorted(
            (
                mtx
                for mtx in synchonization.unlock
            ), key=lambda mtx: mtx.identifier
        ))

        yield from self._mutex_set_transition(lock, unlock)

    def _format_init_barrier(self) -> IntOrStrIterable:
        yield from self._barrier_wait('process_set_start_barrier')

    def _mutex_name(self, mutex: ControlFlowMutex) -> str:
        return f'mutex{mutex.identifier}'

    @abc.abstractmethod
    def _prologue(self) -> IntOrStrIterable: pass

    @abc.abstractmethod
    def _declare_mutex(self, mutex: str) -> IntOrStrIterable: pass

    @abc.abstractmethod
    def _declare_barrier(self, barrier: str): pass

    @abc.abstractmethod
    def _open_process_definition(self, process: Process) -> IntOrStrIterable: pass

    @abc.abstractmethod
    def _close_process_definition(self, process: Process) -> IntOrStrIterable: pass

    @abc.abstractmethod
    def _open_main_definition(self, process: Process) -> IntOrStrIterable: pass

    @abc.abstractmethod
    def _close_main_definition(self) -> IntOrStrIterable: pass

    @abc.abstractmethod
    def _initialize_barrier(self, barrier: str, threads: int): pass

    @abc.abstractmethod
    def _initialize_mutex(self, mutex: str): pass

    @abc.abstractmethod
    def _declare_process(self, process: Process): pass

    @abc.abstractmethod
    def _start_process(self, process: Process): pass

    @abc.abstractmethod
    def _join_process(self, process: Process): pass

    @abc.abstractmethod
    def _lock_mutex(self, mutex: str): pass

    @abc.abstractmethod
    def _unlock_mutex(self, mutex: str): pass

    @abc.abstractmethod
    def _mutex_set_transition(self, lock: Iterable[ControlFlowMutex], unlock: Iterable[ControlFlowMutex]): pass

    @abc.abstractmethod
    def _barrier_wait(self, barrier: str) -> IntOrStrIterable: pass

    @abc.abstractmethod
    def _random(self, max: int) -> str: pass

class HarnessControlFlowUserspaceCodegen(HarnessControlFlowBaseCodegen):
    def _prologue(self):
        yield '#include <stdlib.h>'
        yield '#include <stdio.h>'
        yield '#include <stdbool.h>'
        yield '#include <pthread.h>'

    def _declare_mutex(self, mutex: str):
        yield f'static pthread_mutex_t {mutex};'

    def _declare_barrier(self, barrier: str):
        yield f'static pthread_barrier_t {barrier};'

    def _open_process_definition(self, process: Process):
        yield f'void *process_{process.mnemonic}(void *arg) {{'
        yield 1
        yield '(void) arg; // Unused'

    def _close_process_definition(self, process):
        yield 'return NULL;'
        yield -1
        yield '}'

    def _open_main_definition(self):
        yield 'int main() {'
        yield 1

    def _close_main_definition(self):
        yield 'return 0;'
        yield -1
        yield '}'

    def _initialize_mutex(self, mutex: str):
        yield f'pthread_mutex_init(&{mutex}, NULL);'

    def _initialize_barrier(self, barrier: str, threads: int):
        yield f'pthread_barrier_init(&{barrier}, NULL, {threads});'

    def _declare_process(self, process: Process):
        yield f'pthread_t process_thread_{process.mnemonic};'

    def _start_process(self, process: Process):
        yield f'pthread_create(&process_thread_{process.mnemonic}, NULL, process_{process.mnemonic}, NULL);'

    def _join_process(self, process: Process):
        yield f'pthread_join(process_thread_{process.mnemonic}, NULL);'

    def _lock_mutex(self, mutex: str):
        yield f'pthread_mutex_lock(&{mutex});'

    def _unlock_mutex(self, mutex: str):
        yield f'pthread_mutex_unlock(&{mutex});'

    def _barrier_wait(self, barrier: str) -> IntOrStrIterable:
        yield f'pthread_barrier_wait(&{barrier});'

    def _mutex_set_transition(self, lock: Iterable[ControlFlowMutex], unlock: Iterable[ControlFlowMutex]) -> IntOrStrIterable:
        lock_mtx = sorted(
            (
                mtx
                for mtx in lock
            ), key=lambda mtx: mtx.identifier
        )
        unlock_mtx = reversed(sorted(
            (
                mtx
                for mtx in unlock
            ), key=lambda mtx: mtx.identifier
        ))

        for mtx in unlock_mtx:
            yield from self._unlock_mutex(self._mutex_name(mtx))
        for mtx in lock_mtx:
            yield from self._lock_mutex(self._mutex_name(mtx))

    def _random(self, max: int) -> str:
        return f'(rand() % {max})'
    
class HarnessControlFlowGoblintUserspaceCodegen(HarnessControlFlowUserspaceCodegen):
    def _prologue(self):
        yield from self._embed_multiline('''
extern _Atomic int RANDOM;
typedef unsigned int __harness_thread_t;
typedef unsigned int __harness_thread_mutex_t;
extern void __harness_thread_create(__harness_thread_t *, void *, void *(*)(void *), void *);
extern void __harness_thread_join(__harness_thread_t, void **);
extern void __harness_mutex_init(__harness_thread_mutex_t *, void *);
extern void __harness_mutex_lock(__harness_thread_mutex_t *);
extern void __harness_mutex_unlock(__harness_thread_mutex_t *);
''')

    def _declare_barrier(self, barrier: str):
        yield f'static pthread_mutex_t {barrier}_mtx;'
        yield f'static unsigned long {barrier}_epoch;'
        yield f'static unsigned long {barrier}_waiting;'
        yield f'static unsigned long {barrier}_size;'
        yield ''
        yield from self._embed_multiline('''
static void %barrier%_wait() {
  pthread_mutex_lock(&%barrier%_mtx);
  const unsigned int epoch = %barrier%_epoch;
  %barrier%_waiting++;
  pthread_mutex_unlock(&%barrier%_mtx);

  for (int wait = 1; wait;) {
    pthread_mutex_lock(&%barrier%_mtx);
    if (epoch != %barrier%_epoch) {
      wait = 0;
    } else if (%barrier%_waiting == %barrier%_size) {
      %barrier%_epoch++;
      %barrier%_waiting = 0;
      wait = 0;
    }
    pthread_mutex_unlock(&%barrier%_mtx);
  }
}
'''.replace('%barrier%', barrier))

    def _initialize_barrier(self, barrier: str, threads: int):
        yield f'pthread_mutex_init(&{barrier}_mtx, NULL);'
        yield f'{barrier}_epoch = 0;'
        yield f'{barrier}_waiting = 0;'
        yield f'{barrier}_size = {threads};'
    
    def _barrier_wait(self, barrier: str) -> IntOrStrIterable:
        yield f'{barrier}_wait();'
    
    def _mutex_set_transition(self, lock: Iterable[ControlFlowMutex], unlock: Iterable[ControlFlowMutex]) -> IntOrStrIterable:
        lock_mtx = sorted(
            (
                mtx
                for mtx in lock
            ), key=lambda mtx: mtx.identifier
        )
        unlock_mtx = reversed(sorted(
            (
                mtx
                for mtx in unlock
            ), key=lambda mtx: mtx.identifier
        ))

        for mtx in lock_mtx:
            yield from self._lock_mutex(self._mutex_name(mtx))
        for mtx in unlock_mtx:
            yield from self._unlock_mutex(self._mutex_name(mtx))

    def _random(self, max):
        return f'(RANDOM % {max})'