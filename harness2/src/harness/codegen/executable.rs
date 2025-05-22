use crate::harness::{control_flow::mutex::ControlFlowMutexID, core::{error::HarnessError, process::ProcessID}};

use super::base::{CodegenOutput, ControlFlowCodegen};

pub struct ControlFlowExecutableCodegen;

impl ControlFlowExecutableCodegen {
    pub fn new() -> ControlFlowExecutableCodegen {
        ControlFlowExecutableCodegen {}
    }
}

impl<Output: CodegenOutput> ControlFlowCodegen<Output> for ControlFlowExecutableCodegen {
    fn generate_prologue(&self, output: &mut Output) -> Result<(), HarnessError> {
        output.write_line("#include <stdlib.h>")?;
        output.write_line("#include <stdio.h>")?;
        output.write_line("#include <stdbool.h>")?;
        output.write_line("#include <pthread.h>")?;
        Ok(())
    }
    
    fn declare_mutex(&self, output: &mut Output, mutex: ControlFlowMutexID) -> Result<(), HarnessError> {
        output.write_line(format!("static pthread_mutex_t mutex{};", Into::<u64>::into(mutex)))?;
        Ok(())
    }

    fn declare_init_barrier(&self, output: &mut Output, _: impl Iterator<Item = ProcessID>) -> Result<(), HarnessError> {
        output.write_line("static pthread_barrier_t harness_init_barrier;")?;
        Ok(())
    }

    fn begin_process_definition(&self, output: &mut Output, process: ProcessID) -> Result<(), HarnessError> {
        output.write_line(format!("void *process{}(void *arg) {{", Into::<u64>::into(process)))?;
        output.indent_up();
        output.write_line("(void) arg; // Unused")?;
        Ok(())
    }

    fn end_process_definition(&self, output: &mut Output, _: ProcessID) -> Result<(), HarnessError> {
        output.write_line("return NULL;")?;
        output.indent_down();
        output.write_line("}")?;
        Ok(())
    }

    fn begin_main_definition(&self, output: &mut Output) -> Result<(), HarnessError> {
        output.write_line("int main() {")?;
        output.indent_up();
        Ok(())
    }

    fn end_main_definition(&self, output: &mut Output) -> Result<(), HarnessError> {
        output.write_line("return 0;")?;
        output.indent_down();
        output.write_line("}")?;
        Ok(())
    }

    fn setup_init_barrier(&self, output: &mut Output, processes: impl Iterator<Item = ProcessID>) -> Result<(), HarnessError> {
        output.write_line(format!("pthread_barrier_init(&harness_init_barrier, NULL, {});",
            processes.count()))?;
        Ok(())
    }

    fn initialize_mutex(&self, output: &mut Output, mutex: ControlFlowMutexID) -> Result<(), HarnessError> {
        output.write_line(format!("pthread_mutex_init(&mutex{}, NULL);",
            Into::<u64>::into(mutex)))?;
        Ok(())
    }

    fn declare_process_thread(&self, output: &mut Output, process: ProcessID) -> Result<(), HarnessError> {
        output.write_line(format!("pthread_t process_thread{};",
            Into::<u64>::into(process)))?;
        Ok(())
    }

    fn start_process_thread(&self, output: &mut Output, process: ProcessID) -> Result<(), HarnessError> {
        output.write_line(format!("pthread_create(&process_thread{}, NULL, process{}, NULL);",
            Into::<u64>::into(process), Into::<u64>::into(process)))?;
        Ok(())
    }

    fn join_process_thread(&self, output: &mut Output, process: ProcessID) -> Result<(), HarnessError> {
        output.write_line(format!("pthread_join(process_thread{}, NULL);",
            Into::<u64>::into(process)))?;
        Ok(())
    }

    fn lock_mutex(&self, output: &mut Output, mutex: ControlFlowMutexID) -> Result<(), HarnessError> {
        output.write_line(format!("pthread_mutex_lock(&mutex{});",
            Into::<u64>::into(mutex)))?;
        Ok(())
    }

    fn unlock_mutex(&self, output: &mut Output, mutex: ControlFlowMutexID) -> Result<(), HarnessError> {
        output.write_line(format!("pthread_mutex_unlock(&mutex{});",
            Into::<u64>::into(mutex)))?;
        Ok(())
    }

    fn do_synchronization(&self, output: &mut Output, lock: impl Iterator<Item = ControlFlowMutexID>, unlock: impl Iterator<Item = ControlFlowMutexID>, rollback_on_failure: Option<&str>) -> Result<(), HarnessError> {
        if let Some(rollback_label) = rollback_on_failure {
            output.write_line("for (;;) {")?;
            output.indent_up();

            let lock = lock.collect::<Vec<_>>();
            for (index, &mutex) in lock.iter().enumerate() {
                output.write_line(format!("if (pthread_mutex_trylock(&mutex{})) {{",
                    Into::<u64>::into(mutex)))?;
                output.indent_up();

                for i in (0..index).rev() {
                    output.write_line(format!("pthread_mutex_unlock(&mutex{});",
                        Into::<u64>::into(*lock.get(i).unwrap())))?;
                }
                output.write_line(format!("goto {};", rollback_label))?;

                output.indent_down();
                output.write_line("}")?;
            }

            output.write_line("break;")?;
            output.indent_down();
            output.write_line("}")?;
        } else {
            for mutex in lock {
                self.lock_mutex(output, mutex)?;
            }
        }
        for mutex in unlock {
            self.unlock_mutex(output, mutex)?
        }
        Ok(())
    }

    fn wait_init_barrier(&self, output: &mut Output, _: ProcessID, _: impl Iterator<Item = ProcessID>) -> Result<(), HarnessError> {
        output.write_line("pthread_barrier_wait(&harness_init_barrier);")?;
        Ok(())
    }

    fn generate_random(&self, max: u32) -> String {
        format!("rand() % {}", max)
    }
}
