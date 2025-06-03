use std::{collections::BTreeMap, io::Read, path::Path};

use harness::{codegen::{codegen::ControlFlowCodegen, executable::ControlFlowExecutableCodegen, goblint::ControlFlowGoblintCodegen, output::WriteCodegenOutput}, control_flow::{builder::ControlFlowBuilder, mutex::ControlFlowMutexSet}, core::{error::HarnessError, mutex::mutex::ProcessSetMutualExclusion, process::ProcessSet, state_machine::StateMachineContext}, dsl::{lua::LuaTemplateInterpreter, parser::TemplateParser}};

pub mod harness;

fn main() {
    let harness_filepath = Path::new(&std::env::args().skip(1).next().expect("Expected harness filepath as a command-line argument")).to_path_buf();
    let mut harness_file = std::fs::File::open(&harness_filepath).unwrap();
    let mut harness_code = String::new();
    harness_file.read_to_string(&mut harness_code).unwrap();

    let mut context = StateMachineContext::new();
    let mut process_set = ProcessSet::new();
    let template = TemplateParser::parse(&mut harness_code.chars().map(| x | Ok(x))).unwrap();
    let mut lua_interp = LuaTemplateInterpreter::new();
    let (harness_model, harness_template) = lua_interp.interpret(template.into_iter(), Some(harness_filepath.parent().unwrap().into())).unwrap();
    let harness_model_build = harness_model.build(&mut context, &mut process_set).unwrap();
    let codegen_template = harness_template.build(&harness_model_build).unwrap();

    let state_space = process_set.get_state_space(&context).unwrap();
    let mutual_exclusion = ProcessSetMutualExclusion::new(&context, &process_set, &state_space.derive_reachability()).unwrap();
    let mutex_set = ControlFlowMutexSet::new(mutual_exclusion.iter());
    let control_flow_nodes = process_set.get_processes()
        .map(| process | {
            let root = process_set.get_process_entry_node(process).ok_or(HarnessError::new("Unable to find process entry node"))?;
            let node = ControlFlowBuilder::new(&context, root)?.build(&context, &process_set, process, &mutex_set)?;
            Ok((process, node.canonicalize()))
        }).collect::<Result<BTreeMap<_, _>, HarnessError>>().unwrap();

    let mut stdout = std::io::stdout();
    let mut codegen_output = WriteCodegenOutput::new(&mut stdout);
    if harness_template.is_executable() {
        let codegen = ControlFlowExecutableCodegen::new();
        codegen.format(&mut codegen_output, &context, &process_set, &codegen_template, control_flow_nodes.iter().map(| (process, node) | (*process, node)), mutex_set.get_mutexes()).unwrap();
    } else {
        let codegen = ControlFlowGoblintCodegen::new();
        codegen.format(&mut codegen_output, &context, &process_set, &codegen_template, control_flow_nodes.iter().map(| (process, node) | (*process, node)), mutex_set.get_mutexes()).unwrap();
    }

}
