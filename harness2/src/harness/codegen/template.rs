use std::collections::HashMap;

use crate::harness::core::{process::ProcessID, state_machine::StateMachineActionID};


pub struct CodegenTemplate {
    global_prologue: Option<String>,
    process_parameters: HashMap<ProcessID, HashMap<String, String>>,
    process_prologues: HashMap<ProcessID, String>,
    actions: HashMap<StateMachineActionID, String>
}

impl CodegenTemplate {
    pub fn new() -> CodegenTemplate {
        CodegenTemplate {
            global_prologue: None,
            process_parameters: HashMap::new(),
            process_prologues: HashMap::new(),
            actions: HashMap::new()
        }
    }

    pub fn set_global_prologue<T>(&mut self, prologue: Option<T>) -> &mut Self
        where T: Into<String> {
        self.global_prologue = prologue.map(| content | content.into());
        self
    }

    pub fn set_process_parameter<T>(&mut self, process: ProcessID, name: &str, value: T) -> &mut Self
        where T: Into<String> {
        self.process_parameters.entry(process)
            .or_default()
            .insert(name.into(), value.into());
        self
    }

    pub fn set_process_prologue<T>(&mut self, process: ProcessID, prologue: T) -> &mut Self
        where T: Into<String> {
        self.process_prologues.insert(process, prologue.into());
        self
    }

    pub fn define_action<T>(&mut self, action: StateMachineActionID, content: T) -> &mut Self
        where T: Into<String> {
        self.actions.insert(action, content.into());
        self
    }

    pub fn apply(&self, process: ProcessID, content: &str) -> String {
        let mut content = content.to_owned();
        if let Some(parameters) = self.process_parameters.get(&process) {
            for (key, value) in parameters {
                content = content.replace(&format!("%{}%", key), &value);
            }
        }
        content
    }

    pub fn get_global_prologue(&self) -> Option<&str> {
        self.global_prologue.as_deref()
    }

    pub fn get_process_prologue(&self, process: ProcessID) -> Option<&str> {
        self.process_prologues.get(&process).map(|x| x.as_str())
    }

    pub fn get_action(&self, action: StateMachineActionID) -> Option<&str> {
        self.actions.get(&action).map(|x| x.as_str())
    }
}
