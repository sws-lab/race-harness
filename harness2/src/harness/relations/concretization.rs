use std::collections::HashMap;

use crate::harness::{core::{error::HarnessError, process_state::{ProcessSetState, ProcessSetStateSpace}, state_machine::StateMachineNodeID}, frontend::model::HarnessModel, relations::{db::Sqlite3RelationsDb, error::Sqlite3RelationsDbError}};

pub struct HarnessModelConcretization<'a> {
    abstract_models: HashMap<String, &'a HarnessModel>,
    concrete_model: &'a HarnessModel,
    mappings: HashMap<String, (String, String, HashMap<StateMachineNodeID, StateMachineNodeID>)>
}

impl<'a> HarnessModelConcretization<'a> {
    pub fn new(concrete_model: &'a HarnessModel) -> HarnessModelConcretization<'a> {
        HarnessModelConcretization {
            abstract_models: HashMap::default(),
            concrete_model,
            mappings: HashMap::default()
        }
    }

    pub fn add_abstract_model<T: Into<String>>(&mut self, name: T, model: &'a HarnessModel) {
        self.abstract_models.insert(name.into(), model);
    }

    pub fn add_mapping(&mut self, name: impl Into<String>, source: impl Into<String>, target: impl Into<String>, mapping: HashMap<StateMachineNodeID, StateMachineNodeID>) {
        self.mappings.insert(name.into(), (source.into(), target.into(), mapping));
    }

    pub fn concretize(&self, concretization: &str) -> Result<ProcessSetStateSpace, Sqlite3RelationsDbError> {
        let db = rusqlite::Connection::open_in_memory()?;
        let relation_db = Sqlite3RelationsDb::new(&db)?;
        let mut db_models = HashMap::new();
        for (name, abstract_model) in &self.abstract_models {
            let state_space = abstract_model.get_processes().get_state_space(abstract_model.get_context())?;
            let db_model = relation_db.new_model(abstract_model.get_processes(), abstract_model.get_context(), name)?;
            let _ = db_model.new_state_space(&state_space, name)?;
            db_models.insert(name.clone(), db_model);
        }

        let concrete_db_model = relation_db.new_model(self.concrete_model.get_processes(), self.concrete_model.get_context(), "concrete")?;
        db_models.insert("concrete".into(), concrete_db_model);
        for (name, (source_model_name, target_model_name, mapping)) in &self.mappings {
            let source_model = db_models.get(source_model_name)
                .ok_or(HarnessError::new("Unable to find mapped model"))?;
            let target_model = db_models.get(target_model_name)
                .ok_or(HarnessError::new("Unable to find mapped model"))?;

            let mapping = mapping.iter()
                .map(| (source_node, target_node) | {
                    let source_node_db_id = source_model.get_node_db_id(*source_node)
                        .ok_or(HarnessError::new("Unable to find mapped source node db identifier"))?;
                    let target_node_db_id = target_model.get_node_db_id(*target_node)
                        .ok_or(HarnessError::new("Unable to find mapped target node db identifier"))?;
                    Ok((source_node_db_id, target_node_db_id))
                })
                .collect::<Result<HashMap<i64, i64>, HarnessError>>()?;
            db.create_scalar_function(name.as_str(), 1, rusqlite::functions::FunctionFlags::SQLITE_DETERMINISTIC, move | ctx | {
                let source_node_db_id = ctx.get::<i64>(0)?;
                let target_node_db_id = *mapping.get(&source_node_db_id)
                    .ok_or(rusqlite::Error::UserFunctionError(HarnessError::new("Unable to find mapping for a node").into()))?;
                Ok(target_node_db_id)
            })?;
        }

        let mut concretization_stmt = db.prepare(concretization)?;
        let mut concrete_space_cursor = concretization_stmt.query(())?;
        let mut space = Vec::new();
        loop {
            let row = match concrete_space_cursor.next().unwrap() {
                Some(row) => row,
                None => break
            };
    
            let mut psstate = Vec::new();
            for process in self.concrete_model.get_processes().iter() {
                let process_mnemonic = self.concrete_model.get_processes().get_process_mnemonic(process)
                    .ok_or(HarnessError::new("Unable to retrieve process mnemonic"))?;
                let process_state = row.get::<_, i64>(process_mnemonic)?;
                let node = db_models.get("concrete").unwrap().get_node_by_db_id(process_state)
                    .ok_or(HarnessError::new("Unable to find process node for provided db identifier"))?;
                psstate.push((process, node));
            }
            let psstate = ProcessSetState::new_from(psstate.into_iter());
            space.push(psstate);
        }
        Ok(ProcessSetStateSpace::new(space.into_iter()))
    }
}