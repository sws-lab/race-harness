use crate::harness::{codegen::{codegen::ControlFlowCodegen, output::CodegenOutput, template::CodegenTemplate}, core::{error::HarnessError, process::ProcessSet, state_machine::StateMachineContext}};

pub trait HarnessExample {
    type Model;

    fn build_model(&self, context: &mut StateMachineContext, process_set: &mut ProcessSet) -> Result<Self::Model, HarnessError>;
    fn executable_codegen<Output: CodegenOutput>(&self, model: &Self::Model) -> Result<(CodegenTemplate, impl ControlFlowCodegen<Output>), HarnessError>;
    fn goblint_codegen<Output: CodegenOutput>(&self, model: &Self::Model) -> Result<(CodegenTemplate, impl ControlFlowCodegen<Output>), HarnessError>;
}
