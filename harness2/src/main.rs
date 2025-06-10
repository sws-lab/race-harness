use std::{collections::{BTreeMap, HashSet}, io::Read, marker::PhantomData, path::Path, rc::Rc};

use harness::{codegen::{codegen::ControlFlowCodegen, executable::ControlFlowExecutableCodegen, goblint::ControlFlowGoblintCodegen, output::WriteCodegenOutput}, control_flow::{builder::ControlFlowBuilder, mutex::ControlFlowMutexSet}, core::{error::HarnessError, mutex::mutex::ProcessSetMutualExclusion, process::ProcessSet, state_machine::StateMachineContext}, dsl::{lua::LuaTemplateInterpreter, parser::TemplateParser}};

use crate::harness::{core::{process::ProcessID, process_state::{ProcessSetState, ProcessSetStateSpace}}, relations::db::Sqlite3RelationsDb};

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
    let interp_result = lua_interp.interpret(template.into_iter(), Some(harness_filepath.parent().unwrap().into())).unwrap();
    let harness_model_build = interp_result.model.build(&mut context, &mut process_set).unwrap();
    let codegen_template = interp_result.template.build(&harness_model_build).unwrap();

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
    if interp_result.template.is_executable() {
        let codegen = ControlFlowExecutableCodegen::new();
        codegen.format(&mut codegen_output, &context, &process_set, &codegen_template, control_flow_nodes.iter().map(| (process, node) | (*process, node)), mutex_set.get_mutexes()).unwrap();
    } else {
        let codegen = ControlFlowGoblintCodegen::new();
        codegen.format(&mut codegen_output, &context, &process_set, &codegen_template, control_flow_nodes.iter().map(| (process, node) | (*process, node)), mutex_set.get_mutexes()).unwrap();
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
