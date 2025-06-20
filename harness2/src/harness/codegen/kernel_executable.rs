use crate::harness::{control_flow::mutex::ControlFlowMutexID, core::{error::HarnessError, process::{ProcessID, ProcessSet}}};

use super::{codegen::ControlFlowCodegen, output::CodegenOutput};

pub struct ControlFlowKernelExecutableCodegen;

impl ControlFlowKernelExecutableCodegen {
    pub fn new() -> ControlFlowKernelExecutableCodegen {
        ControlFlowKernelExecutableCodegen {}
    }
}

impl<Output: CodegenOutput> ControlFlowCodegen<Output> for ControlFlowKernelExecutableCodegen {
    fn generate_prologue(&self, output: &mut Output) -> Result<(), HarnessError> {
        output.write_line(r#"
typedef struct {
  long arr[32];
} pthread_barrier_t;

typedef struct {
  long arr[32];
} pthread_mutex_t;

typedef struct {
  long arr[64];
} pthread_t;

typedef struct {
  long arr[32];
} pthread_attr_t;

typedef struct {
  long arr[32];
} pthread_barrierattr_t;

typedef struct {
  long arr[32];
} pthread_mutexattr_t;

int rand(void);
void sleep(unsigned long);

int pthread_barrier_wait(pthread_barrier_t *);
int pthread_mutex_init(pthread_mutex_t *, pthread_mutexattr_t *);
int pthread_mutex_lock(pthread_mutex_t *);
int pthread_mutex_trylock(pthread_mutex_t *);
int pthread_mutex_unlock(pthread_mutex_t *);
int pthread_create(pthread_t *, const pthread_attr_t *, void *(*)(void*), void *);
int pthread_join(pthread_t, void **);
int pthread_barrier_init(pthread_barrier_t *restrict, const pthread_barrierattr_t *restrict, unsigned count);
"#)?;
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

    fn begin_process_definition(&self, output: &mut Output, process_set: &ProcessSet, process: ProcessID) -> Result<(), HarnessError> {
        let process_mnemonic = process_set.get_process_mnemonic(process).ok_or(HarnessError::new("Unable to retrieve process mnemonic"))?;
        output.write_line(format!("static void *process_{}(void *arg) {{", process_mnemonic))?;
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

    fn declare_process_thread(&self, output: &mut Output, process_set: &ProcessSet, process: ProcessID) -> Result<(), HarnessError> {
        let process_mnemonic = process_set.get_process_mnemonic(process).ok_or(HarnessError::new("Unable to retrieve process mnemonic"))?;
        output.write_line(format!("pthread_t process_{}_thread;",
            process_mnemonic))?;
        Ok(())
    }

    fn start_process_thread(&self, output: &mut Output, process_set: &ProcessSet, process: ProcessID) -> Result<(), HarnessError> {
        let process_mnemonic = process_set.get_process_mnemonic(process).ok_or(HarnessError::new("Unable to retrieve process mnemonic"))?;
        output.write_line(format!("pthread_create(&process_{}_thread, NULL, process_{}, NULL);",
            process_mnemonic, process_mnemonic))?;
        Ok(())
    }

    fn join_process_thread(&self, output: &mut Output, process_set: &ProcessSet, process: ProcessID) -> Result<(), HarnessError> {
        let process_mnemonic = process_set.get_process_mnemonic(process).ok_or(HarnessError::new("Unable to retrieve process mnemonic"))?;
        output.write_line(format!("for (;;) sleep(1);"))?;
        output.write_line(format!("pthread_join(process_{}_thread, NULL);",
            process_mnemonic))?;
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
            let lock = lock.collect::<Vec<_>>();
            for (index, &mutex) in lock.iter().enumerate() {
                output.write_line(format!("if (pthread_mutex_trylock(&mutex{})) {{",
                    Into::<u64>::into(mutex)))?;
                output.indent_up();

                for i in (0..index).rev() {
                    output.write_line(format!("pthread_mutex_unlock(&mutex{});",
                        Into::<u64>::into(*lock.get(i).expect("Expected mutex at index to exist"))))?;
                }
                output.write_line(format!("goto {};", rollback_label))?;

                output.indent_down();
                output.write_line("}")?;
            }
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

    fn wait_init_barrier(&self, output: &mut Output, _: &ProcessSet, _: ProcessID, _: impl Iterator<Item = ProcessID>) -> Result<(), HarnessError> {
        output.write_line("pthread_barrier_wait(&harness_init_barrier);")?;
        Ok(())
    }

    fn generate_random(&self, max: u32) -> String {
        format!("rand() % {}", max)
    }
}
