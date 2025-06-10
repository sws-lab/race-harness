use std::{collections::BTreeMap, io::Read, path::Path};

use harness::{codegen::{codegen::ControlFlowCodegen, executable::ControlFlowExecutableCodegen, goblint::ControlFlowGoblintCodegen, output::WriteCodegenOutput}, control_flow::{builder::ControlFlowBuilder, mutex::ControlFlowMutexSet}, core::{error::HarnessError, mutex::mutex::ProcessSetMutualExclusion}, dsl::{parser::TemplateParser}};

use crate::harness::{dsl::lua::InterpretedLuaModelTemplate, frontend::model::HarnessModel};

pub mod harness;

fn main() {
    let harness_filepath = Path::new(&std::env::args().skip(1).next().expect("Expected harness filepath as a command-line argument")).to_path_buf();
    let mut harness_file = std::fs::File::open(&harness_filepath).unwrap();
    let mut harness_code = String::new();
    harness_file.read_to_string(&mut harness_code).unwrap();

    let template = TemplateParser::parse(&mut harness_code.chars().map(| x | Ok(x))).unwrap();
    let template_models = InterpretedLuaModelTemplate::new(template.into_iter(), Some(harness_filepath.parent().unwrap().into())).unwrap();
    let concrete_model = HarnessModel::new(&template_models.get_concrete_model()).unwrap();
    let codegen_template = template_models.get_template().build(concrete_model.get_symbolic_model_build()).unwrap();

    let state_space = match &template_models.get_concretization() {
        Some(_) => {
            // TODO
            concrete_model.get_processes().get_state_space(concrete_model.get_context()).unwrap()
        },
        None => concrete_model.get_processes().get_state_space(concrete_model.get_context()).unwrap()
    };

    let mutual_exclusion = ProcessSetMutualExclusion::new(concrete_model.get_context(), concrete_model.get_processes(), &state_space.derive_reachability()).unwrap();
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

    // let conn = rusqlite::Connection::open_in_memory().unwrap();
    // let db = Sqlite3RelationsDb::new(&conn).unwrap();
    // let db_abstract_model = db.new_model(&process_set, &context, "abstract").unwrap();
    // let _db_abstract_state_space = db_abstract_model.new_state_space(&state_space, "abstract1").unwrap();
    
    // let mut query = conn.prepare(r#"
    //     SELECT n1.Name AS tty_driver, n2.Name AS tty_client1, n3.Name AS tty_client2, n4.Name AS tty_client3 FROM (
    //         SELECT DISTINCT a1.tty_driver AS tty_driver, a1.tty_client1 AS tty_client1, a2.tty_client1 AS tty_client2, a3.tty_client1 AS tty_client3
    //         FROM abstract AS a1
    //             INNER JOIN abstract AS a2 ON a1.tty_driver = a2.tty_driver
    //             INNER JOIN abstract AS a3 ON a1.tty_driver = a3.tty_driver
    //             INNER JOIN abstract AS a4 ON a1.tty_driver = a4.tty_driver
    //     ) AS concrete
    //     INNER JOIN Node AS n1 ON n1.ID = concrete.tty_driver
    //     INNER JOIN Node AS n2 ON n2.ID = concrete.tty_client1
    //     INNER JOIN Node AS n3 ON n3.ID = concrete.tty_client2
    //     INNER JOIN Node AS n4 ON n4.ID = concrete.tty_client3
    // "#).unwrap();
    // let mut cursor = query.query(()).unwrap();
    // loop {
    //     let row = match cursor.next().unwrap() {
    //         Some(row) => row,
    //         None => break
    //     };

    //     println!("{:?}", row);
    // }
}
