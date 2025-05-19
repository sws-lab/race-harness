from typing import Iterable
from harness.core import Process
from harness.control_flow import ControlFlowMutex
from harness.codegen.control_flow.base import HarnessControlFlowBaseCodegen, IntOrStrIterable

class HarnessControlFlowGoblintCodegen(HarnessControlFlowBaseCodegen):
    def _prologue(self):
        yield from self._embed_multiline('''
extern _Atomic long _harness_random;

typedef unsigned int __harness_thread_t;
typedef unsigned int __harness_mutex_t;
extern void __harness_thread_create(__harness_thread_t *, void *, void *(*)(void *), void *);
extern void __harness_thread_join(__harness_thread_t, void **);
extern void __harness_mutex_init(__harness_mutex_t *, void *);
extern void __harness_mutex_lock(__harness_mutex_t *);
extern void __harness_mutex_unlock(__harness_mutex_t *);
''')

    def _declare_mutex(self, mutex: str):
        yield f'static __harness_mutex_t {mutex};'

    def _declare_init_barrier(self, processes: Iterable[Process]):
        for process in processes:
            yield f'static _Atomic unsigned int process_{process.mnemonic}_init_barrier;'

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

    def _setup_init_barrier(self, processes: Iterable[Process]):
        for process in processes:
            yield f'process_{process.mnemonic}_init_barrier = 0;'
    
    def _init_barrier_wait(self, process: Process, other_processes: Iterable[Process]) -> IntOrStrIterable:
        yield f'process_{process.mnemonic}_init_barrier = 1;'
        yield 'while ({}); // Wait for other processes'.format(
            ' || '.join(
                f'!process_{other_process.mnemonic}_init_barrier'
                for other_process in other_processes
            )
        )

    def _initialize_mutex(self, mutex: str):
        yield f'__harness_mutex_init(&{mutex}, NULL);'

    def _declare_process(self, process: Process):
        yield f'__harness_thread_t process_thread_{process.mnemonic};'

    def _start_process(self, process: Process):
        yield f'__harness_thread_create(&process_thread_{process.mnemonic}, NULL, process_{process.mnemonic}, NULL);'

    def _join_process(self, process: Process):
        yield f'__harness_thread_join(process_thread_{process.mnemonic}, NULL);'

    def _lock_mutex(self, mutex: str):
        yield f'__harness_mutex_lock(&{mutex});'

    def _unlock_mutex(self, mutex: str):
        yield f'__harness_mutex_unlock(&{mutex});'
    
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
        return f'(_harness_random % {max})'
