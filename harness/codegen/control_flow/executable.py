from typing import Iterable, Optional
from harness.core import Process
from harness.control_flow import ControlFlowMutex
from harness.codegen.control_flow.base import HarnessControlFlowBaseCodegen, IntOrStrIterable

class HarnessControlFlowExecutableCodegen(HarnessControlFlowBaseCodegen):
    def _prologue(self):
        yield '#include <stdlib.h>'
        yield '#include <stdio.h>'
        yield '#include <stdbool.h>'
        yield '#include <pthread.h>'

    def _declare_mutex(self, mutex: str):
        yield f'static pthread_mutex_t {mutex};'

    def _declare_init_barrier(self, _: Iterable[Process]):
        yield f'static pthread_barrier_t harness_init_barrier;'

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

    def _setup_init_barrier(self, processes: Iterable[Process]):
        yield f'pthread_barrier_init(&harness_init_barrier, NULL, {len(list(processes))});'

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

    def _init_barrier_wait(self, process: Process, other_processes: Iterable[Process]) -> IntOrStrIterable:
        yield f'pthread_barrier_wait(&harness_init_barrier);'

    def _mutex_set_transition(self, lock: Iterable[ControlFlowMutex], unlock: Iterable[ControlFlowMutex], rollback_on_failure: Optional[str]) -> IntOrStrIterable:
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
        
        if lock_mtx:
            if rollback_on_failure is not None:
                yield 'for (;;) {'
                yield 1

                for mtx_index, mtx in enumerate(lock_mtx):
                    yield f'if (pthread_mutex_trylock(&{self._mutex_name(mtx)})) {{'
                    yield 1

                    for i in range(mtx_index - 1, -1, -1):
                        yield f'pthread_mutex_unlock(&{self._mutex_name(lock_mtx[i])});'
                    if rollback_on_failure is not None:
                        yield f'goto {rollback_on_failure};'
                    else:
                        yield 'continue;'

                    yield -1
                    yield '}'
                yield 'break;'

                yield -1
                yield '}'
            else:
                for mtx in lock_mtx:
                    yield from self._lock_mutex(self._mutex_name(mtx))
        for mtx in unlock_mtx:
            yield from self._unlock_mutex(self._mutex_name(mtx))

    def _random(self, max: int) -> str:
        return f'(rand() % {max})'
