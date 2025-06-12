use crate::harness::{core::{process::ProcessSet, state_machine::StateMachineContext}, relations::{error::Sqlite3RelationsDbError, model::Sqlite3ModelDatabase}};

pub struct Sqlite3ModelRelationsDb<'a> {
    connection: &'a rusqlite::Connection
}

impl<'a> Sqlite3ModelRelationsDb<'a> {
    pub fn new(connection: &'a rusqlite::Connection) -> Result<Sqlite3ModelRelationsDb<'a>, Sqlite3RelationsDbError> {
        let db = Sqlite3ModelRelationsDb { connection };
        db.initialize_schema()?;
        Ok(db)
    }

    pub fn new_model<'b: 'a, T: Into<String>>(&self, processes: &'b ProcessSet, context: &'b StateMachineContext, name: T) -> Result<Sqlite3ModelDatabase<'a>, Sqlite3RelationsDbError> {
        Sqlite3ModelDatabase::new(self.connection, processes, context, name.into())
    }

    fn initialize_schema(&self) -> Result<(), rusqlite::Error> {
        self.connection.execute(r#"
            CREATE TABLE IF NOT EXISTS Model (
                ID INTEGER PRIMARY KEY AUTOINCREMENT,
                Name VARCHAR
            )
        "#, ())?;

        self.connection.execute(r#"
            CREATE TABLE IF NOT EXISTS Process (
                ID INTEGER PRIMARY KEY AUTOINCREMENT,
                Name VARCHAR,
                ModelID INTEGER,
                FOREIGN KEY (ModelID) REFERENCES Model(ID)
            )
        "#, ())?;

        self.connection.execute(r#"
            CREATE TABLE IF NOT EXISTS Node (
                ID INTEGER PRIMARY KEY,
                Name VARCHAR,
                ModelID INTEGER,
                FOREIGN KEY (ModelID) REFERENCES Model(ID)
            )
        "#, ())?;

        self.connection.execute(r#"
            CREATE TABLE IF NOT EXISTS Space (
                ID INTEGER PRIMARY KEY,
                Name VARCHAR,
                ModelID INTEGER,
                FOREIGN KEY (ModelID) REFERENCES Model(ID)
            )
        "#, ())?;

        self.connection.execute(r#"
            CREATE TABLE IF NOT EXISTS SpaceState (
                ID INTEGER PRIMARY KEY,
                SpaceID INTEGER,
                FOREIGN KEY (SpaceID) REFERENCES Space(ID)
            )
        "#, ())?;

        self.connection.execute(r#"
            CREATE TABLE IF NOT EXISTS ProcessState (
                SpaceStateID INTEGER,
                ProcessID INTEGER,
                NodeID INTEGER,
                PRIMARY KEY (SpaceStateID, ProcessID, NodeID),
                FOREIGN KEY (SpaceStateID) REFERENCES SpaceState(ID),
                FOREIGN KEY (ProcessID) REFERENCES Process(ID),
                FOREIGN KEY (NodeID) REFERENCES Node(ID)
            )
        "#, ())?;

        Ok(())
    }
}