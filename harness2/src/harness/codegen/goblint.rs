use crate::harness::{control_flow::mutex::ControlFlowMutexID, core::{error::HarnessError, process::ProcessID}};

use super::base::{CodegenOutput, ControlFlowCodegen};

pub struct ControlFlowGoblintCodegen;

impl ControlFlowGoblintCodegen {
    pub fn new() -> ControlFlowGoblintCodegen {
        ControlFlowGoblintCodegen {}
    }
}

impl<Output: CodegenOutput> ControlFlowCodegen<Output> for ControlFlowGoblintCodegen {
    fn generate_prologue(&self, output: &mut Output) -> Result<(), HarnessError> {
        self.embed_multiline(output, r#"
extern _Atomic long _harness_random;

typedef unsigned int __harness_thread_t;
typedef unsigned int __harness_mutex_t;
extern void __harness_thread_create(__harness_thread_t *, void *, void *(*)(void *), void *);
extern void __harness_thread_join(__harness_thread_t, void **);
extern void __harness_mutex_init(__harness_mutex_t *, void *);
extern void __harness_mutex_lock(__harness_mutex_t *);
extern void __harness_mutex_unlock(__harness_mutex_t *);
"#)?;
        Ok(())
    }
    
    fn declare_mutex(&self, output: &mut Output, mutex: ControlFlowMutexID) -> Result<(), HarnessError> {
        output.write_line(format!("static __harness_mutex_t mutex{};", Into::<u64>::into(mutex)))?;
        Ok(())
    }

    fn declare_init_barrier(&self, output: &mut Output, processes: impl Iterator<Item = ProcessID>) -> Result<(), HarnessError> {
        for process in processes {
            output.write_line(format!("static _Atomic unsigned int process{}_init_barrier;",
                Into::<u64>::into(process)))?;
        }
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
        for process in processes {
            output.write_line(format!("process{}_init_barrier = 0;",
                Into::<u64>::into(process)))?;
        }
        Ok(())
    }

    fn initialize_mutex(&self, output: &mut Output, mutex: ControlFlowMutexID) -> Result<(), HarnessError> {
        output.write_line(format!("__harness_mutex_init(&mutex{}, NULL);",
            Into::<u64>::into(mutex)))?;
        Ok(())
    }

    fn declare_process_thread(&self, output: &mut Output, process: ProcessID) -> Result<(), HarnessError> {
        output.write_line(format!("__harness_thread_t process_thread{};",
            Into::<u64>::into(process)))?;
        Ok(())
    }

    fn start_process_thread(&self, output: &mut Output, process: ProcessID) -> Result<(), HarnessError> {
        output.write_line(format!("__harness_thread_create(&process_thread{}, NULL, process{}, NULL);",
            Into::<u64>::into(process), Into::<u64>::into(process)))?;
        Ok(())
    }

    fn join_process_thread(&self, output: &mut Output, process: ProcessID) -> Result<(), HarnessError> {
        output.write_line(format!("__harness_thread_join(process_thread{}, NULL);",
            Into::<u64>::into(process)))?;
        Ok(())
    }

    fn lock_mutex(&self, output: &mut Output, mutex: ControlFlowMutexID) -> Result<(), HarnessError> {
        output.write_line(format!("__harness_mutex_lock(&mutex{});",
            Into::<u64>::into(mutex)))?;
        Ok(())
    }

    fn unlock_mutex(&self, output: &mut Output, mutex: ControlFlowMutexID) -> Result<(), HarnessError> {
        output.write_line(format!("__harness_mutex_unlock(&mutex{});",
            Into::<u64>::into(mutex)))?;
        Ok(())
    }

    fn do_synchronization(&self, output: &mut Output, lock: impl Iterator<Item = ControlFlowMutexID>, unlock: impl Iterator<Item = ControlFlowMutexID>, _: Option<&str>) -> Result<(), HarnessError> {
        for mutex in lock {
            self.lock_mutex(output, mutex)?;
        }
        for mutex in unlock {
            self.unlock_mutex(output, mutex)?
        }
        Ok(())
    }

    fn wait_init_barrier(&self, output: &mut Output, process: ProcessID, other_processes: impl Iterator<Item = ProcessID>) -> Result<(), HarnessError> {
        output.write_line(format!("process{}_init_barrier = 1;",
            Into::<u64>::into(process)))?;

        let mut condition = String::new();
        for (index, other_process) in other_processes.enumerate() {
            if index > 0 {
                condition.push_str(" || ");
            }
            condition.push_str(&format!("!process{}_init_barrier",
                Into::<u64>::into(other_process)));
        }

        output.write_line(format!("while ({});", condition))?;
        Ok(())
    }

    fn generate_random(&self, max: u32) -> String {
        format!("(_harness_random % {})", max)
    }
}
