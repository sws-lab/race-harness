use std::collections::HashMap;

use crate::harness::{core::{error::HarnessError, reachability::{ProcessReachabilityPair, ProcessStateReachability}, state_machine::StateMachineNodeID}, frontend::model::HarnessModel, relations::{db::Sqlite3RelationsDb, error::Sqlite3RelationsDbError}};

pub struct HarnessModelConcretization<'a> {
    abstract_models: HashMap<String, &'a HarnessModel>,
    concrete_model: &'a HarnessModel,
    mappings: HashMap<String, (String, String, HashMap<StateMachineNodeID, StateMachineNodeID>)>,
    queries: Vec<String>
}

impl<'a> HarnessModelConcretization<'a> {
    pub fn new(concrete_model: &'a HarnessModel) -> HarnessModelConcretization<'a> {
        HarnessModelConcretization {
            abstract_models: HashMap::default(),
            concrete_model,
            mappings: HashMap::default(),
            queries: Vec::new()
        }
    }

    pub fn add_abstract_model<T: Into<String>>(&mut self, name: T, model: &'a HarnessModel) {
        self.abstract_models.insert(name.into(), model);
    }

    pub fn add_mapping(&mut self, name: impl Into<String>, source: impl Into<String>, target: impl Into<String>, mapping: HashMap<StateMachineNodeID, StateMachineNodeID>) {
        self.mappings.insert(name.into(), (source.into(), target.into(), mapping));
    }

    pub fn add_query(&mut self, query: impl Into<String>) {
        self.queries.push(query.into());
    }

    pub fn construct_reachability(&self, db: &rusqlite::Connection, concretization: &str) -> Result<ProcessStateReachability, Sqlite3RelationsDbError> {
        let relation_db = Sqlite3RelationsDb::new(&db)?;
        let mut db_models = HashMap::new();
        for (name, abstract_model) in &self.abstract_models {
            let state_space = abstract_model.get_processes().get_state_space(abstract_model.get_context())?;
            let db_model = relation_db.new_model(abstract_model.get_processes(), abstract_model.get_context(), name)?;
            let _ = db_model.new_state_space(&state_space, name)?;
            db_model.materialize(&name)?;
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

        for query in &self.queries {
            let _ = db.execute_batch(query)?;
        }

        let mut reachability = ProcessStateReachability::new();
        let processes = self.concrete_model.get_processes().iter().collect::<Vec<_>>();
        for (process_idx, &process) in processes.iter().enumerate() {
            let process_mnemonic = self.concrete_model.get_processes().get_process_mnemonic(process)
                .ok_or(HarnessError::new("Unable to retrieve process mnemonic"))?;
            for other_process in self.concrete_model.get_processes().iter().skip(process_idx).filter(| proc | *proc != process) {
                let other_process_mnemonic = self.concrete_model.get_processes().get_process_mnemonic(other_process)
                    .ok_or(HarnessError::new("Unable to retrieve process mnemonic"))?;

                let mut concretization_pair_stmt = db.prepare(format!("SELECT DISTINCT {}, {} FROM {}", process_mnemonic, other_process_mnemonic, concretization).as_str())?;
                let mut concrete_pair_cursor = concretization_pair_stmt.query(())?;
                loop {
                    let row = match concrete_pair_cursor.next().unwrap() {
                        Some(row) => row,
                        None => break
                    };

                    let process_state = row.get::<_, i64>(0)?;
                    let other_process_state = row.get::<_, i64>(1)?;
                    let node = db_models.get("concrete").unwrap().get_node_by_db_id(process_state)
                        .ok_or(HarnessError::new("Unable to find process node for provided db identifier"))?;
                    let other_node = db_models.get("concrete").unwrap().get_node_by_db_id(other_process_state)
                        .ok_or(HarnessError::new("Unable to find process node for provided db identifier"))?;
            
                    reachability.mark_cooccuring(&ProcessReachabilityPair::new(process, node, other_process, other_node));
                }
            }
        }
        Ok(reachability)
    }
}