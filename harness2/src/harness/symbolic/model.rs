use crate::harness::{core::{error::HarnessError, process::ProcessSet, state_machine::StateMachineContext}, symbolic::symbolic_model::{HarnessSymbolicModel, HarnessModelSymbols}};

pub struct HarnessModel {
    context: StateMachineContext,
    processes: ProcessSet,
    symbols: HarnessModelSymbols
}

impl HarnessModel {
    pub fn new(symbolic_model: &HarnessSymbolicModel) -> Result<HarnessModel, HarnessError> {
        let mut context = StateMachineContext::new();
        let mut processes = ProcessSet::new();
        let symbolic_build = symbolic_model.build(&mut context, &mut processes)?;
        Ok(HarnessModel {
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

    pub fn get_symbols(&self) -> &HarnessModelSymbols {
        &self.symbols
    }
}
