use crate::harness::{core::{error::HarnessError, process::ProcessSet, state_machine::StateMachineContext}, system::symbolic_model::{SymbolicSystemModel, SystemModelSymbols}};

pub struct SystemModel {
    context: StateMachineContext,
    processes: ProcessSet,
    symbols: SystemModelSymbols
}

impl SystemModel {
    pub fn new(symbolic_model: &SymbolicSystemModel) -> Result<SystemModel, HarnessError> {
        let mut context = StateMachineContext::new();
        let mut processes = ProcessSet::new();
        let symbolic_build = symbolic_model.build(&mut context, &mut processes)?;
        Ok(SystemModel {
            context,
            processes,
            symbols: symbolic_build
        })
    }

    pub fn get_context(&self) -> &StateMachineContext {
        &self.context
    }

    pub fn get_processes(&self) -> &ProcessSet {
        &self.processes
    }

    pub fn get_symbols(&self) -> &SystemModelSymbols {
        &self.symbols
    }
}
