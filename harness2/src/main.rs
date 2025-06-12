use std::{collections::{BTreeMap, HashMap}, io::Read, path::Path};

use harness::{codegen::{codegen::ControlFlowCodegen, executable::ControlFlowExecutableCodegen, goblint::ControlFlowGoblintCodegen, output::WriteCodegenOutput}, control_flow::{builder::ControlFlowBuilder, mutex::ControlFlowMutexSet}, core::{error::HarnessError, mutex::mutex::ProcessSetMutualExclusion}, dsl::{parser::DSLParser}};

use crate::harness::{dsl::lua::DSLInterpreter, relations::concretization::HarnessModelConcretization, symbolic::model::HarnessModel};

pub mod harness;

fn main() {
    let harness_filepath = Path::new(&std::env::args().skip(1).next().expect("Expected harness filepath as a command-line argument")).to_path_buf();
    let mut harness_file = std::fs::File::open(&harness_filepath).unwrap();
    let mut harness_code = String::new();
    harness_file.read_to_string(&mut harness_code).unwrap();

    let template = DSLParser::parse(&mut harness_code.chars().map(| x | Ok(x))).unwrap();
    let symbolic_harness = DSLInterpreter::new().interpret(template.into_iter(), Some(harness_filepath.parent().unwrap().into())).unwrap();
    let (concrete_model_name, concrete_symbolic_model) = symbolic_harness.get_model(symbolic_harness.get_concrete_model_id()).unwrap();
    let concrete_model = HarnessModel::new(concrete_symbolic_model).unwrap();
    let codegen_template = symbolic_harness.get_template().build(concrete_model.get_symbols()).unwrap();

    let reachability = match symbolic_harness.get_concretization() {
        Some(concretization) => {
            let mut abstract_models = HashMap::new();
            for abstract_model_id in concretization.get_abstract_model_ids() {
                let (abstract_model_name, abstract_model) = symbolic_harness.get_model(*abstract_model_id).unwrap();
                let model = HarnessModel::new(abstract_model).unwrap();
                abstract_models.insert(abstract_model_name, model);
            }

            let mut concretizer = HarnessModelConcretization::new();
            for (&name, abstract_model) in &abstract_models {
                concretizer.add_abstract_model(name, abstract_model);
            }
            for (name, mapping) in concretization.get_state_mappings() {
                let (source_model_name, _) = symbolic_harness.get_model(mapping.get_source_model()).unwrap();
                let (target_model_name, _) = symbolic_harness.get_model(mapping.get_target_model()).unwrap();
                let source_model = abstract_models.get(source_model_name).unwrap();
                let target_model = if target_model_name == concrete_model_name {
                    &concrete_model
                } else {
                    abstract_models.get(target_model_name).unwrap()
                };
                let mapping_build = mapping.build(source_model.get_symbols(), target_model.get_symbols()).unwrap();
                concretizer.add_mapping(name, source_model_name, target_model_name, mapping_build);
            }
            for query in concretization.get_queries() {
                concretizer.add_query(query);
            }

            let db = if let Ok(db_path) = std::env::var("HARNESSS_CONCRETIZATION_DB") {
                rusqlite::Connection::open(db_path).unwrap()
            } else {
                rusqlite::Connection::open_in_memory().unwrap()
            };
            concretizer.construct_reachability(&db, concrete_model_name, concretization.get_concretization_relation(), &concrete_model).unwrap()
        },
        None => concrete_model.get_processes().get_state_space(concrete_model.get_context()).unwrap().derive_reachability()
    };

    let mutual_exclusion = ProcessSetMutualExclusion::new(concrete_model.get_context(), concrete_model.get_processes(), &reachability).unwrap();
    let mutex_set = ControlFlowMutexSet::new(mutual_exclusion.iter());
    let control_flow_nodes = concrete_model.get_processes().get_processes()
        .map(| process | {
            let root = concrete_model.get_processes().get_process_entry_node(process).ok_or(HarnessError::new("Unable to find process entry node"))?;
            let node = ControlFlowBuilder::new(concrete_model.get_context(), root)?.build(concrete_model.get_context(), concrete_model.get_processes(), process, &mutex_set)?;
            Ok((process, node.canonicalize()))
        }).collect::<Result<BTreeMap<_, _>, HarnessError>>().unwrap();

    let mut stdout = std::io::stdout();
    let mut codegen_output = WriteCodegenOutput::new(&mut stdout);
    if symbolic_harness.get_template().is_executable() {
        let codegen = ControlFlowExecutableCodegen::new();
        codegen.format(&mut codegen_output, concrete_model.get_context(), concrete_model.get_processes(), &codegen_template, control_flow_nodes.iter().map(| (process, node) | (*process, node)), mutex_set.get_mutexes()).unwrap();
    } else {
        let codegen = ControlFlowGoblintCodegen::new();
        codegen.format(&mut codegen_output, concrete_model.get_context(), concrete_model.get_processes(), &codegen_template, control_flow_nodes.iter().map(| (process, node) | (*process, node)), mutex_set.get_mutexes()).unwrap();
    }
}
