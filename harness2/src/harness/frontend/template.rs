use std::collections::HashMap;

use crate::harness::{codegen::template::CodegenTemplate, core::error::HarnessError};

use super::model::{HarnessModelBuild, HarnessModelSymbol};

struct HarnessProcessTemplate {
    parameters: HashMap<String, String>,
    prologue: Option<String>
}

struct HarnessActionTemplate {
    content: Option<String>
}

pub struct HarnessTemplate {
    action_templates: HashMap<HarnessModelSymbol, HarnessActionTemplate>,
    process_templates: HashMap<HarnessModelSymbol, HarnessProcessTemplate>,
    global_prologue: Option<String>,
    executable: bool
}

impl HarnessTemplate {
    pub fn new() -> HarnessTemplate {
        HarnessTemplate {
            action_templates: HashMap::new(),
            process_templates: HashMap::new(),
            global_prologue: None,
            executable: false
        }
    }

    pub fn set_executable(&mut self, executable: bool) {
        self.executable = executable;
    }

    pub fn is_executable(&self) -> bool {
        self.executable
    }

    pub fn append_global_prologue(&mut self, prologue: String) {
        match &mut self.global_prologue {
            Some(head) => {
                head.push_str(&prologue);
            }

            None => self.global_prologue = Some(prologue)
        }
    }

    pub fn append_process_prologue(&mut self, process: HarnessModelSymbol, prologue: String) -> Result<(), HarnessError> {
        let process_template = self.get_process_template_mut(process);
        match &mut process_template.prologue {
            Some(head) => {
                head.push_str(&prologue);
            }

            None => process_template.prologue = Some(prologue)
        }
        Ok(())
    }

    pub fn set_process_parameter(&mut self, process: HarnessModelSymbol, key: String, value: String) -> Result<(), HarnessError> {
        self.get_process_template_mut(process).parameters.insert(key, value);
        Ok(())
    }

    pub fn set_action_content(&mut self, action: HarnessModelSymbol, content: String) -> Result<(), HarnessError> {
        self.get_action_template_mut(action).content = Some(content);
        Ok(())
    }

    pub fn build(&self, build: &HarnessModelBuild) -> Result<CodegenTemplate, HarnessError> {
        let mut template = CodegenTemplate::new();

        if let Some(prologue) = &self.global_prologue {
            template.set_global_prologue(Some(prologue));
        }

        for (&action_symbol, action_tempalte) in &self.action_templates {
            let action = build.get_action(action_symbol).ok_or(HarnessError::new("Unable to find action by symbol"))?;
            if let Some(content) = &action_tempalte.content {
                template.define_action(action, content);
            }
        }
        
        for (&process_symbol, process_template) in &self.process_templates {
            let process = build.get_process(process_symbol).ok_or(HarnessError::new("Unable to find process by symbol"))?;
            if let Some(prologue) = &process_template.prologue {
                template.set_process_prologue(process, prologue);
            }
            for (key, value) in &process_template.parameters {
                template.set_process_parameter(process, &key, value);
            }
        }

        Ok(template)
    }

    fn get_process_template_mut(&mut self, process: HarnessModelSymbol) -> &mut HarnessProcessTemplate {
        self.process_templates.entry(process).or_insert(HarnessProcessTemplate {
            parameters: HashMap::new(),
            prologue: None
        })
    }

    fn get_action_template_mut(&mut self, action: HarnessModelSymbol) -> &mut HarnessActionTemplate {
        self.action_templates.entry(action).or_insert(HarnessActionTemplate { content: None })
    }
}
