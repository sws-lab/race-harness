use crate::harness::{core::{error::HarnessError, process::ProcessSet, state_machine::StateMachineContext}, frontend::symbolic_model::{HarnessSymbolicModel, HarnessSymbolicModelBuild}};

pub struct HarnessModel {
    context: StateMachineContext,
    processes: ProcessSet,
    symbolic_build: HarnessSymbolicModelBuild
}

impl HarnessModel {
    pub fn new(symbolic_model: &HarnessSymbolicModel) -> Result<HarnessModel, HarnessError> {
        let mut context = StateMachineContext::new();
        let mut processes = ProcessSet::new();
        let symbolic_build = symbolic_model.build(&mut context, &mut processes)?;
        Ok(HarnessModel {
            context,
            processes,
            symbolic_build
        })
    }

    pub fn get_context(&self) -> &StateMachineContext {
        &self.context
    }

    pub fn get_processes(&self) -> &ProcessSet {
        &self.processes
    }

    pub fn get_symbolic_model_build(&self) -> &HarnessSymbolicModelBuild {
        &self.symbolic_build
    }
}
