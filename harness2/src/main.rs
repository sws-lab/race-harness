use std::{collections::BTreeMap, io::Read, path::Path};

use harness::{codegen::{codegen::ControlFlowCodegen, executable::ControlFlowExecutableCodegen, goblint::ControlFlowGoblintCodegen, output::WriteCodegenOutput}, control_flow::{builder::ControlFlowBuilder, mutex::ControlFlowMutexSet}, core::{error::HarnessError, mutex::mutex::ProcessSetMutualExclusion}};

use crate::harness::{dsl::{dsl::DSLInterpreter, lua::interpreter::LuaDSLInterpreter}, harness::harness::Harness, relations::concretization::HarnessModelConcretization};

pub mod harness;

fn main() {
    let harness_filepath = Path::new(&std::env::args().skip(1).next().expect("Expected harness filepath as a command-line argument")).to_path_buf();
    let mut harness_file = std::fs::File::open(&harness_filepath).unwrap();
    let mut harness_code = String::new();
    harness_file.read_to_string(&mut harness_code).unwrap();

    let dsl_interpreter = LuaDSLInterpreter::new();
    let template = dsl_interpreter.parse(&mut harness_code.chars().map(| x | Ok(x))).unwrap();
    let symbolic_harness = dsl_interpreter.interpret(&template, Some(harness_filepath.parent().unwrap().into())).unwrap();
    let harness = Harness::new(&symbolic_harness).unwrap();

    let reachability = match harness.get_concretization() {
        Some(concretization) => {
            let db = if let Ok(db_path) = std::env::var("HARNESSS_CONCRETIZATION_DB") {
                rusqlite::Connection::open(db_path).unwrap()
            } else {
                rusqlite::Connection::open_in_memory().unwrap()
            };

            let concretizer = HarnessModelConcretization::new_from(concretization).unwrap();
            concretizer.construct_reachability(&db, harness.get_concrete_model_name(), concretization.get_concretization_relation(), harness.get_concrete_model()).unwrap()
        },
        None => harness.get_concrete_model().get_processes().get_state_space(harness.get_concrete_model().get_context()).unwrap().derive_reachability()
    };

    let mutual_exclusion = ProcessSetMutualExclusion::new(harness.get_concrete_model().get_context(), harness.get_concrete_model().get_processes(), &reachability).unwrap();
    let mutex_set = ControlFlowMutexSet::new(mutual_exclusion.iter());
    let control_flow_nodes = harness.get_concrete_model().get_processes().get_processes()
        .map(| process | {
            let root = harness.get_concrete_model().get_processes().get_process_entry_node(process).ok_or(HarnessError::new("Unable to find process entry node"))?;
            let node = ControlFlowBuilder::new(harness.get_concrete_model().get_context(), root)?.build(harness.get_concrete_model().get_context(), harness.get_concrete_model().get_processes(), process, &mutex_set)?;
            Ok((process, node.canonicalize()))
        }).collect::<Result<BTreeMap<_, _>, HarnessError>>().unwrap();

    let mut stdout = std::io::stdout();
    let mut codegen_output = WriteCodegenOutput::new(&mut stdout);
    if symbolic_harness.get_template().is_executable() {
        let codegen = ControlFlowExecutableCodegen::new();
        codegen.format(&mut codegen_output, harness.get_concrete_model().get_context(), harness.get_concrete_model().get_processes(), harness.get_template(), control_flow_nodes.iter().map(| (process, node) | (*process, node)), mutex_set.get_mutexes()).unwrap();
    } else {
        let codegen = ControlFlowGoblintCodegen::new();
        codegen.format(&mut codegen_output, harness.get_concrete_model().get_context(), harness.get_concrete_model().get_processes(), harness.get_template(), control_flow_nodes.iter().map(| (process, node) | (*process, node)), mutex_set.get_mutexes()).unwrap();
    }
}
