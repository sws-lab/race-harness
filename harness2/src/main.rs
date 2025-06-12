use std::{collections::{BTreeMap, HashMap}, io::Read, path::Path};

use harness::{codegen::{codegen::ControlFlowCodegen, executable::ControlFlowExecutableCodegen, goblint::ControlFlowGoblintCodegen, output::WriteCodegenOutput}, control_flow::{builder::ControlFlowBuilder, mutex::ControlFlowMutexSet}, core::{error::HarnessError, mutex::mutex::ProcessSetMutualExclusion}, dsl::{parser::TemplateParser}};

use crate::harness::{dsl::lua::InterpretedLuaModelTemplate, symbolic::model::HarnessModel, relations::concretization::HarnessModelConcretization};

pub mod harness;

fn main() {
    let harness_filepath = Path::new(&std::env::args().skip(1).next().expect("Expected harness filepath as a command-line argument")).to_path_buf();
    let mut harness_file = std::fs::File::open(&harness_filepath).unwrap();
    let mut harness_code = String::new();
    harness_file.read_to_string(&mut harness_code).unwrap();

    let template = TemplateParser::parse(&mut harness_code.chars().map(| x | Ok(x))).unwrap();
    let template_models = InterpretedLuaModelTemplate::new(template.into_iter(), Some(harness_filepath.parent().unwrap().into())).unwrap();
    let concrete_model = HarnessModel::new(&template_models.get_concrete_model()).unwrap();
    let codegen_template = template_models.get_template().build(concrete_model.get_symbols()).unwrap();

    let reachability = match &template_models.get_concretization() {
        Some(concretization) => {
            let mut abstract_models = HashMap::new();
            for (name, symbolic_model) in template_models.get_abstract_models() {
                let model = HarnessModel::new(symbolic_model).unwrap();
                abstract_models.insert(name, model);
            }

            let mut concretizer = HarnessModelConcretization::new();
            for (&name, abstract_model) in &abstract_models {
                concretizer.add_abstract_model(name, abstract_model);
            }
            for (name, mapping) in template_models.get_mappings() {
                let source_model = abstract_models.get(mapping.get_source_model_name()).unwrap();
                let target_model = &concrete_model;
                let mapping_build = mapping.build(source_model.get_symbols(), target_model.get_symbols()).unwrap();
                concretizer.add_mapping(name, mapping.get_source_model_name(), mapping.get_target_model_name(), mapping_build);
            }
            for query in template_models.get_queries() {
                concretizer.add_query(query);
            }

            let db = if let Ok(db_path) = std::env::var("HARNESSS_CONCRETIZATION_DB") {
                rusqlite::Connection::open(db_path).unwrap()
            } else {
                rusqlite::Connection::open_in_memory().unwrap()
            };
            concretizer.construct_reachability(&db, "concrete", concretization, &concrete_model).unwrap()
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
    if template_models.get_template().is_executable() {
        let codegen = ControlFlowExecutableCodegen::new();
        codegen.format(&mut codegen_output, concrete_model.get_context(), concrete_model.get_processes(), &codegen_template, control_flow_nodes.iter().map(| (process, node) | (*process, node)), mutex_set.get_mutexes()).unwrap();
    } else {
        let codegen = ControlFlowGoblintCodegen::new();
        codegen.format(&mut codegen_output, concrete_model.get_context(), concrete_model.get_processes(), &codegen_template, control_flow_nodes.iter().map(| (process, node) | (*process, node)), mutex_set.get_mutexes()).unwrap();
    }
}
