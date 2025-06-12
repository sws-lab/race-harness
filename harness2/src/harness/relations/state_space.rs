use crate::harness::{core::{error::HarnessError, process_state::ProcessSetStateSpace}, relations::{error::Sqlite3RelationsDbError, model::Sqlite3ModelDatabase}};

pub struct Sqlite3StateSpaceDatabase {}

impl<'a> Sqlite3StateSpaceDatabase {
    pub fn new(connection: &'a rusqlite::Connection, model: &'a Sqlite3ModelDatabase<'a>, state_space: &ProcessSetStateSpace, name: &str) -> Result<Sqlite3StateSpaceDatabase, Sqlite3RelationsDbError> {
        let mut space_insert_query = connection.prepare(r#"
            INSERT INTO Space (Name, ModelID) VALUES (?, ?) RETURNING ID
        "#)?;
        let mut psstate_insert_query = connection.prepare(r#"
            INSERT INTO SpaceState (SpaceID) VALUES (?) RETURNING ID
        "#)?;
        let mut state_insert_query = connection.prepare(r#"
            INSERT INTO ProcessState (SpaceStateID, ProcessID, NodeID) VALUES (?, ?, ?)
        "#)?;

        let space_db_id: i64 = space_insert_query.query((name, model.get_harness_db_id()))?
            .next()?
            .expect("Expected state space identifier to exist")
            .get(0)?;

        for psstate in state_space.iter() {
            let psstate_db_id: i64 = psstate_insert_query.query([space_db_id])?
                .next()?
                .expect("Expected process set state identifier to exist")
                .get(0)?;
            for process in model.get_processes().get_processes() {
                let node = psstate.get_process_node(process)
                    .ok_or(HarnessError::new("Unable to retrieve process node"))?;

                let process_db_id = model.get_process_db_id(process)
                    .ok_or(HarnessError::new("Unable to find process db identifier"))?;
                let node_db_id = model.get_node_db_id(node)
                    .ok_or(HarnessError::new("Unable to find node db identifier"))?;

                state_insert_query.execute((psstate_db_id, process_db_id, node_db_id))?;
            }
        }

        let db_space = Sqlite3StateSpaceDatabase {};
    
        Ok(db_space)
    }
}
