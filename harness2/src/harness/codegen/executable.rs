use crate::harness::{control_flow::mutex::ControlFlowMutexID, core::{error::HarnessError, process::ProcessID}};

use super::base::{CodegenOutput, ControlFlowBaseCodegen};

pub trait ControlFlowExecutableCodegen: CodegenOutput {}

impl<T: ControlFlowExecutableCodegen> ControlFlowBaseCodegen for T {
    fn generate_prologue(&mut self) -> Result<(), HarnessError> {
        self.write_line("#include <stdlib.h>");
        self.write_line("#include <stdio.h>");
        self.write_line("#include <stdbool.h>");
        self.write_line("#include <pthread.h>");
        Ok(())
    }
    
    fn declare_mutex(&mut self, mutex: ControlFlowMutexID) -> Result<(), HarnessError> {
        self.write_line(format!("static pthread_mutex_t mutex{};", Into::<u64>::into(mutex)));
        Ok(())
    }

    fn declare_init_barrier(&mut self, _: impl Iterator<Item = ProcessID>) -> Result<(), HarnessError> {
        self.write_line("static pthread_barrier_t harness_init_barrier;");
        Ok(())
    }

    fn begin_process_definition(&mut self, process: ProcessID) -> Result<(), HarnessError> {
        self.write_line(format!("void *process{}(void *arg) {{", Into::<u64>::into(process)));
        self.indent_up();
        self.write_line("(void) arg; // Unused");
        Ok(())
    }

    fn end_process_definition(&mut self, _: ProcessID) -> Result<(), HarnessError> {
        self.write_line("return NULL;");
        self.indent_down();
        self.write_line("}");
        Ok(())
    }

    fn begin_main_definition(&mut self) -> Result<(), HarnessError> {
        self.write_line("int main() {");
        self.indent_up();
        Ok(())
    }

    fn end_main_definition(&mut self) -> Result<(), HarnessError> {
        self.write_line("return 0;");
        self.indent_down();
        self.write_line("}");
        Ok(())
    }

    fn setup_init_barrier(&mut self, processes: impl Iterator<Item = ProcessID>) -> Result<(), HarnessError> {
        self.write_line(format!("pthread_barrier_init(&harness_init_barrier, NULL, {});",
            processes.count()));
        Ok(())
    }

    fn initialize_mutex(&mut self, mutex: ControlFlowMutexID) -> Result<(), HarnessError> {
        self.write_line(format!("pthread_mutex_init(&mutex{}, NULL);",
            Into::<u64>::into(mutex)));
        Ok(())
    }

    fn declare_process_thread(&mut self, process: ProcessID) -> Result<(), HarnessError> {
        self.write_line(format!("pthread_t process_thread{};",
            Into::<u64>::into(process)));
        Ok(())
    }

    fn start_process_thread(&mut self, process: ProcessID) -> Result<(), HarnessError> {
        self.write_line(format!("pthread_create(&process_thread{}, NULL, process{}, NULL);",
            Into::<u64>::into(process), Into::<u64>::into(process)));
        Ok(())
    }

    fn join_process_thread(&mut self, process: ProcessID) -> Result<(), HarnessError> {
        self.write_line(format!("pthread_join(process_thread{}, NULL);",
            Into::<u64>::into(process)));
        Ok(())
    }

    fn lock_mutex(&mut self, mutex: ControlFlowMutexID) -> Result<(), HarnessError> {
        self.write_line(format!("pthread_mutex_lock(&mutex{});",
            Into::<u64>::into(mutex)));
        Ok(())
    }

    fn unlock_mutex(&mut self, mutex: ControlFlowMutexID) -> Result<(), HarnessError> {
        self.write_line(format!("pthread_mutex_unlock(&mutex{});",
            Into::<u64>::into(mutex)));
        Ok(())
    }

    fn do_synchronization(&mut self, lock: impl Iterator<Item = ControlFlowMutexID>, unlock: impl Iterator<Item = ControlFlowMutexID>, rollback_on_failure: Option<&str>) -> Result<(), HarnessError> {
        if let Some(rollback_label) = rollback_on_failure {
            self.write_line("for (;;) {");
            self.indent_up();

            let lock = lock.collect::<Vec<_>>();
            for (index, &mutex) in lock.iter().enumerate() {
                self.write_line(format!("if (pthread_mutex_trylock(&mutex{})) {{",
                    Into::<u64>::into(mutex)));
                self.indent_up();

                for i in (0..index).rev() {
                    self.write_line(format!("pthread_mutex_unlock(&mutex{});",
                        Into::<u64>::into(*lock.get(i).unwrap())));
                }
                self.write_line(format!("goto {};", rollback_label));

                self.indent_down();
                self.write_line("}");
            }

            self.write_line("break;");
            self.indent_down();
            self.write_line("}");
        } else {
            for mutex in lock {
                self.lock_mutex(mutex)?;
            }
        }
        for mutex in unlock {
            self.unlock_mutex(mutex)?
        }
        Ok(())
    }

    fn wait_init_barrier(&mut self, _: ProcessID, _: impl Iterator<Item = ProcessID>) -> Result<(), HarnessError> {
        self.write_line("pthread_barrier_wait(&harness_init_barrier);");
        Ok(())
    }

    fn generate_random(&self, max: u32) -> String {
        format!("rand() % {}", max)
    }
}
