use std::collections::HashMap;

use crate::harness::{core::{error::HarnessError, process::{ProcessID, ProcessSet}, process_state::ProcessSetStateSpace, state_machine::{StateMachineContext, StateMachineNodeID}}, relations::{error::Sqlite3RelationsDbError, state_space::Sqlite3StateSpaceDatabase}};

pub struct Sqlite3ModelDatabase<'a> {
    connection: &'a rusqlite::Connection,
    processes: &'a ProcessSet,
    harness_db_id: i64,
    process_db_ids: HashMap<ProcessID, i64>,
    node_db_ids: HashMap<StateMachineNodeID, i64>,
    reverse_node_db_ids: HashMap<i64, StateMachineNodeID>
}

impl<'a> Sqlite3ModelDatabase<'a> {
    pub fn new(connection: &'a rusqlite::Connection, processes: &'a ProcessSet, context: &'a StateMachineContext, name: String) -> Result<Sqlite3ModelDatabase<'a>, Sqlite3RelationsDbError> {
        let mut harness_query = connection.prepare(r#"
            INSERT INTO Model (Name) VALUES (?) RETURNING ID
        "#)?;
        let mut process_query = connection.prepare(r#"
            INSERT INTO Process (Name, ModelID) VALUES (?, ?) RETURNING ID
        "#)?;
        let mut node_query = connection.prepare(r#"
            INSERT INTO Node (Name, ModelID) VALUES (?, ?) RETURNING ID
        "#)?;

        let harness_db_id: i64 = harness_query.query([name.as_str()])?
            .next()?
            .expect("Expected harness identifier to exist")
            .get(0)?;

        let mut process_db_ids = HashMap::new();
        for process in processes.get_processes() {
            let process_mnemonic = processes.get_process_mnemonic(process)
                .ok_or(HarnessError::new("Unable to retrieve process mnemonic"))?;
            let process_id: i64 = process_query.query((process_mnemonic, harness_db_id))?
                .next()?
                .expect("Expected process identifier to exist")
                .get(0)?;
            process_db_ids.insert(process, process_id);
        }

        let mut node_db_ids = HashMap::new();
        let mut reverse_node_db_ids = HashMap::new();
        for node in context.get_all_nodes() {
            let node_mnemonic = context.get_node_mnemonic(node)
                .ok_or(HarnessError::new("Unable to retrieve process mnemonic"))?;
            let node_id: i64 = node_query.query((node_mnemonic, harness_db_id))?
                .next()?
                .expect("Expected node identifier to exist")
                .get(0)?;
            node_db_ids.insert(node, node_id);
            reverse_node_db_ids.insert(node_id, node);
        }

        let model = Sqlite3ModelDatabase {
            connection,
            processes,
            harness_db_id,
            process_db_ids,
            node_db_ids,
            reverse_node_db_ids
        };
        Ok(model)
    }

    pub fn new_state_space(&self, state_space: &ProcessSetStateSpace, name: &str) -> Result<Sqlite3StateSpaceDatabase, Sqlite3RelationsDbError> {
        Sqlite3StateSpaceDatabase::new(self.connection, self, state_space, name)
    }

    pub fn get_processes(&self) -> &'a ProcessSet {
        self.processes
    }

    pub fn get_harness_db_id(&self) -> i64 {
        self.harness_db_id
    }

    pub fn get_process_db_id(&self, process: ProcessID) -> Option<i64> {
        self.process_db_ids.get(&process).map(| x | *x)
    }

    pub fn get_node_db_id(&self, node: StateMachineNodeID) -> Option<i64> {
        self.node_db_ids.get(&node).map(| x | *x)
    }

    pub fn get_node_by_db_id(&self, node_db_id: i64) -> Option<StateMachineNodeID> {
        self.reverse_node_db_ids.get(&node_db_id).map(| x | *x)
    }

    pub fn materialize_into(&self, name: &str) -> Result<(), Sqlite3RelationsDbError> {
        // UGLY, UNSAFE and BAD, but nonetheless
        let mut sql_query = String::from("CREATE TABLE ");
        sql_query.push_str(name);
        sql_query.push_str(" AS SELECT ");
        for (idx, process) in self.processes.get_processes().enumerate() {
            let process_mnemonic = self.processes.get_process_mnemonic(process)
                .ok_or(HarnessError::new("Unable to find process mnemonic"))?;
            if idx > 0 {
                sql_query.push_str(format!(", state{}.NodeID AS {}", idx, process_mnemonic).as_str());
            } else {
                sql_query.push_str(format!("state0.SpaceStateID AS SpaceStateID, state0.NodeID AS {}", process_mnemonic).as_str());
            }
        }
        let mut where_clause = String::default();
        for (idx, process) in self.processes.get_processes().enumerate() {
            let process_db_id = *self.process_db_ids.get(&process)
                .ok_or(HarnessError::new("Unable to find process database identifier"))?;
            if idx > 0 {
                sql_query.push_str(format!(" INNER JOIN ProcessState AS state{} ON state0.SpaceStateID = state{}.SpaceStateID AND state{}.ProcessID = {}", idx, idx, idx, process_db_id).as_str());
            } else {
                sql_query.push_str(" FROM ProcessState AS state0");
                where_clause = format!(" WHERE state0.ProcessID = {}", process_db_id);
            }
        }
        sql_query.push_str(where_clause.as_str());
        self.connection.execute(&sql_query, ())?;

        for process in self.get_processes().iter() {
            let process_mnemonic = self.get_processes().get_process_mnemonic(process)
                .ok_or(HarnessError::new("Unable to find process mnemonic"))?;
            self.connection.execute(format!("CREATE INDEX {}_{} ON {}({})", name, process_mnemonic, name, process_mnemonic).as_str(), ())?;
        }

        Ok(())
    }
}