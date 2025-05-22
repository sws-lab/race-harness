use std::collections::BTreeMap;

use harness::{codegen::{codegen::ControlFlowCodegen, output::WriteCodegenOutput}, control_flow::{builder::ControlFlowBuilder, mutex::ControlFlowMutexSet}, core::{error::HarnessError, mutex::mutex::ProcessSetMutualExclusion, process::ProcessSet, state_machine::StateMachineContext}, examples::{base::HarnessExample, pcspkr::PcspkrExample, ttyprintk::TtyPrintkExample}};

pub mod harness;

fn generate(example: &impl HarnessExample) -> Result<(), HarnessError> {
    let mut context = StateMachineContext::new();
    let mut process_set = ProcessSet::new();
    let model = example.build_model(&mut context, &mut process_set)?;

    let state_space = process_set.get_state_space(&context)?;
    let mutual_exclusion = ProcessSetMutualExclusion::new(&context, &process_set, &state_space)?;
    let mutex_set = ControlFlowMutexSet::new(mutual_exclusion.iter());
    let control_flow_nodes = process_set.get_processes()
        .map(| process | {
            let root = process_set.get_process_entry_node(process).ok_or(HarnessError::new("Unable to find process entry node"))?;
            let node = ControlFlowBuilder::new(&context, root)?.build(&context, &process_set, process, &mutex_set)?;
            Ok((process, node.canonicalize()))
        }).collect::<Result<BTreeMap<_, _>, HarnessError>>()?;

    let mut stdout = std::io::stdout();
    let mut stderr = std::io::stderr();

    let mut codegen_output = WriteCodegenOutput::new(&mut stdout);
    let (template, codegen) = example.executable_codegen(&model)?;
    codegen.format(&mut codegen_output, &context, &process_set, &template, control_flow_nodes.iter().map(| (process, node) | (*process, node)), mutex_set.get_mutexes())?;

    let mut codegen_output = WriteCodegenOutput::new(&mut stderr);
    let (template, codegen) = example.goblint_codegen(&model)?;
    codegen.format(&mut codegen_output, &context, &process_set, &template, control_flow_nodes.iter().map(| (process, node) | (*process, node)), mutex_set.get_mutexes())?;
    Ok(())
}

fn main() {
    match std::env::var("HARNESS_EXAMPLE").as_deref().unwrap_or("ttyprintk") {
        "ttyprintk" => generate(&TtyPrintkExample::new( 5)),
        "pcspkr" => generate(&PcspkrExample::new(5, 5)),
        example => panic!("Unknown HARNESS_EXAMPLE={}", example)

    }.unwrap()
}
