use std::collections::HashMap;

use crate::harness::{control_flow::{mutex::{ControlFlowMutex, ControlFlowMutexID}, node::{ControlFlowLabel, ControlFlowNode}}, core::{error::HarnessError, process::{ProcessID, ProcessSet}, state_machine::{StateMachineContext, StateMachineEdgeID}}};

use super::{output::CodegenOutput, template::CodegenTemplate};

pub trait ControlFlowCodegen<Output: CodegenOutput> {
    fn generate_prologue(&self, output: &mut Output) -> Result<(), HarnessError>;
    fn declare_mutex(&self, output: &mut Output, mutex: ControlFlowMutexID) -> Result<(), HarnessError>;
    fn declare_init_barrier(&self, output: &mut Output, processes: impl Iterator<Item = ProcessID>) -> Result<(), HarnessError>;
    fn begin_process_definition(&self, output: &mut Output, process: ProcessID) -> Result<(), HarnessError>;
    fn end_process_definition(&self, output: &mut Output, process: ProcessID) -> Result<(), HarnessError>;
    fn begin_main_definition(&self, output: &mut Output) -> Result<(), HarnessError>;
    fn end_main_definition(&self, output: &mut Output) -> Result<(), HarnessError>;
    fn setup_init_barrier(&self, output: &mut Output, processes: impl Iterator<Item = ProcessID>) -> Result<(), HarnessError>;
    fn initialize_mutex(&self, output: &mut Output, mutex: ControlFlowMutexID) -> Result<(), HarnessError>;
    fn declare_process_thread(&self, output: &mut Output, process: ProcessID) -> Result<(), HarnessError>;
    fn start_process_thread(&self, output: &mut Output, process: ProcessID) -> Result<(), HarnessError>;
    fn join_process_thread(&self, output: &mut Output, process: ProcessID) -> Result<(), HarnessError>;
    fn lock_mutex(&self, output: &mut Output, mutex: ControlFlowMutexID) -> Result<(), HarnessError>;
    fn unlock_mutex(&self, output: &mut Output, mutex: ControlFlowMutexID) -> Result<(), HarnessError>;
    fn do_synchronization(&self, output: &mut Output, lock: impl Iterator<Item = ControlFlowMutexID>, unlock: impl Iterator<Item = ControlFlowMutexID>, rollback_on_failure: Option<&str>) -> Result<(), HarnessError>;
    fn wait_init_barrier(&self, output: &mut Output, process: ProcessID, other_processes: impl Iterator<Item = ProcessID>) -> Result<(), HarnessError>;
    fn generate_random(&self, max: u32) -> String;

    fn embed_multiline(&self, output: &mut Output, content: &str) -> Result<(), HarnessError> {
        for line in content.split("\n") {
            output.write_line(line)?;
        }
        Ok(())
    }

    fn format<'a>(&self, output: &mut Output, context: &StateMachineContext, process_set: &ProcessSet, template: &CodegenTemplate, processes: impl Iterator<Item = (ProcessID, &'a ControlFlowNode)>, mutexes: impl Iterator<Item = &'a ControlFlowMutex<'a>>) -> Result<(), HarnessError> {
        self.generate_prologue(output)?;
        output.write_line("")?;

        if let Some(prologue) = template.get_global_prologue() {
            self.embed_multiline(output, prologue)?;
            output.write_line("")?;
        }

        let processes = processes.collect::<Vec<_>>();
        self.declare_init_barrier(output, processes.iter().map(| (process, _) | *process))?;
        output.write_line("")?;

        let mutexes = mutexes.collect::<Vec<_>>();
        for mutex in &mutexes {
            let mut comment_content = String::new();
            let mutex_content = mutex.get_segment().iter()
                    .map(| (process, state) | {
                        let process_mnemonic = process_set.get_process_mnemonic(process)
                            .ok_or(HarnessError::new("Unable to retrieve process mnemonic"))?;
                        let state_mnemonic = context.get_node_mnemonic(state)
                            .ok_or(HarnessError::new("Unable to retrieve node mnemonic"))?;
                        Ok((process_mnemonic, state_mnemonic))
                    })
                    .collect::<Result<Vec<(&str, &str)>, HarnessError>>()?;
            for (index, (process, state)) in mutex_content.iter().enumerate() {
                if index > 0 {
                    comment_content.push_str(", ");
                }
                comment_content.push_str(process);
                comment_content.push_str(": ");
                comment_content.push_str(&state);
            }
            output.write_line(format!("/* {} */", comment_content))?;
            self.declare_mutex(output, mutex.get_identifier())?;
            output.write_line("")?;
        }
        output.write_line("")?;

        for (process, root_node) in &processes {
            self.begin_process_definition(output, *process)?;
            output.write_line("")?;

            if let Some(prologue) = template.get_process_prologue(*process) {
                self.embed_multiline(output, &template.apply(*process, prologue))?;
                output.write_line("")?;
            }

            let mut label_map = HashMap::new();
            let mut get_label = | label: ControlFlowLabel |
                label_map.entry(label)
                    .or_insert(format!("label{}", Into::<u64>::into(label)))
                    .clone();
            self.format_node(output, context, process_set, template, *process, root_node, &mut get_label)?;

            output.write_line("")?;
            self.end_process_definition(output, *process)?;
            output.write_line("")?;
        }
        output.write_line("")?;

        self.begin_main_definition(output)?;

        for (process, _) in &processes {
            self.declare_process_thread(output, *process)?;
        }
        output.write_line("")?;

        self.setup_init_barrier(output, processes.iter().map(| (process, _) | *process))?;
        output.write_line("")?;

        for mutex in &mutexes {
            self.initialize_mutex(output, mutex.get_identifier())?;
        }
        output.write_line("")?;

        for (process, _) in &processes {
            self.start_process_thread(output, *process)?;
        }
        output.write_line("")?;

        for (process, _) in &processes {
            self.join_process_thread(output, *process)?;
        }
        output.write_line("")?;

        self.end_main_definition(output)?;
        output.flush()?;
        Ok(())
    }

    fn format_node<'a>(&self, output: &mut Output, context: &StateMachineContext, process_set: &ProcessSet, template: &CodegenTemplate, process: ProcessID, node: &'a ControlFlowNode, label_map: &mut impl FnMut(ControlFlowLabel) -> String) -> Result<(), HarnessError> {
        match node {
            ControlFlowNode::Statement(edge) =>
                self.format_statement(output, context, template, process, *edge)?,
            ControlFlowNode::Sequence(seq) =>
                self.format_sequence(output, context, process_set, template, process, seq.iter(), label_map)?,
            ControlFlowNode::LabelledNode(label, subnode) =>
                self.format_labelled(output, context, process_set, template, process, *label, &subnode, label_map)?,
            ControlFlowNode::Branch(branches) =>
                self.format_branch(output, context, process_set, template, process, branches, label_map)?,
            ControlFlowNode::InitBarrier =>
                self.format_init_barrier(output, process_set, process)?,
            ControlFlowNode::Goto(label) =>
                self.format_goto(output, *label, label_map)?,
            ControlFlowNode::Synchronization(lock, unlock, rollback_on_failure) =>
                self.format_synchronization(output, lock.iter().map(| x | *x), unlock.iter().map(| x | *x), *rollback_on_failure, label_map)?,
        };
        Ok(())
    }

    fn format_statement(&self, output: &mut Output, context: &StateMachineContext, template: &CodegenTemplate, process: ProcessID, edge: StateMachineEdgeID) -> Result<(), HarnessError> {
        let edge_source = context.get_edge_source(edge)
            .ok_or(HarnessError::new("Unable to retrieve edge source"))?;
        let edge_target = context.get_edge_target(edge)
            .ok_or(HarnessError::new("Unable to retrieve edge target"))?;
        let edge_action = context.get_edge_action(edge);
        
        let edge_source_mnemonic = context.get_node_mnemonic(edge_source)
            .ok_or(HarnessError::new("Unable to retrieve edge source mnemonic"))?;
        let edge_target_mnemonic = context.get_node_mnemonic(edge_target)
            .ok_or(HarnessError::new("Unable to retrieve edge source mnemonic"))?;
        output.write_line(format!("/* {} -> {} */", edge_source_mnemonic, edge_target_mnemonic))?;

        if let Some(action) = edge_action {
            if let Some(content) = template.get_action(action) {
                self.embed_multiline(output, &template.apply(process, content))?;
            }
        }
        Ok(())
    }

    fn format_sequence<'a>(&self, output: &mut Output, context: &StateMachineContext, process_set: &ProcessSet, template: &CodegenTemplate, process: ProcessID, sequence: impl Iterator<Item = &'a ControlFlowNode>, label_map: &mut impl FnMut(ControlFlowLabel) -> String) -> Result<(), HarnessError> {
        output.write_line("{")?;
        output.indent_up();

        for node in sequence {
            self.format_node(output, context, process_set, template, process, node, label_map)?;
        }

        output.indent_down();
        output.write_line("}")?;
        Ok(())
    }

    fn format_labelled(&self, output: &mut Output, context: &StateMachineContext, process_set: &ProcessSet, template: &CodegenTemplate, process: ProcessID, label: ControlFlowLabel, node: &ControlFlowNode, label_map: &mut impl FnMut(ControlFlowLabel) -> String) -> Result<(), HarnessError> {
        let label_text = label_map(label);
        output.write_line(format!("{}: ", label_text))?;
        output.skip_newline();
        self.format_node(output, context, process_set, template, process, node, label_map)?;
        Ok(())
    }

    fn format_branch(&self, output: &mut Output, context: &StateMachineContext, process_set: &ProcessSet, template: &CodegenTemplate, process: ProcessID, branches: &Vec<ControlFlowNode>, label_map: &mut impl FnMut(ControlFlowLabel) -> String) -> Result<(), HarnessError> {
        for (index, branch) in branches.iter().enumerate() {
            if index == 0 {
                output.write_line(format!("if ({} == 0) ",
                    self.generate_random(branches.len() as u32)))?;
                output.skip_newline();
            }

            self.format_node(output, context, process_set, template, process, branch, label_map)?;
            
            if index + 2 < branches.len() {
                output.skip_newline();
                output.write_line(format!("else if ({} == 0) ",
                    self.generate_random((branches.len() - index) as u32)))?;
                output.skip_newline();
            } else if index + 1 < branches.len() {
                output.skip_newline();
                output.write_line(" else ")?;
                output.skip_newline();
            }
        }
        Ok(())
    }

    fn format_init_barrier(&self, output: &mut Output, process_set: &ProcessSet, process: ProcessID) -> Result<(), HarnessError> {
        self.wait_init_barrier(output, process, process_set.iter().filter(| other_process | *other_process != process))
    }

    fn format_goto(&self, output: &mut Output, label: ControlFlowLabel, label_map: &mut impl FnMut(ControlFlowLabel) -> String) -> Result<(), HarnessError> {
        let label_text = label_map(label);
        output.write_line(format!("goto {};", label_text))?;
        Ok(())
    }

    fn format_synchronization(&self, output: &mut Output, lock: impl Iterator<Item = ControlFlowMutexID>, unlock: impl Iterator<Item = ControlFlowMutexID>, rollback_on_failure: Option<ControlFlowLabel>, label_map: &mut impl FnMut(ControlFlowLabel) -> String) -> Result<(), HarnessError> {
        let lock = {
            let mut mutexes = lock.collect::<Vec<_>>();
            mutexes.sort();
            mutexes
        };

        let unlock = {
            let mut mutexes = unlock.collect::<Vec<_>>();
            mutexes.sort();
            mutexes.reverse();
            mutexes
        };

        let rollback_on_failure = if let Some(label) = rollback_on_failure {
            let label_text = label_map(label);
            Some(label_text)
        } else {
            None
        };

        self.do_synchronization(output, lock.into_iter(), unlock.into_iter(), rollback_on_failure.as_ref().map(| label | label.as_str()))?;
        Ok(())
    }
}
