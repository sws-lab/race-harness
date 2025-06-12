use std::collections::{HashMap, HashSet};

use crate::harness::{core::{error::HarnessError, state_machine::StateMachineNodeID}, harness::template::HarnessSymbolicTemplate, system::symbolic_model::{SymbolicSystemModel, SystemModelSymbol, SystemModelSymbols}};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub struct SymbolicHarnessModelID(u64);

pub struct SymbolicHarnessMapping {
    source_model: SymbolicHarnessModelID,
    target_model: SymbolicHarnessModelID,
    mapping: HashMap<SystemModelSymbol, SystemModelSymbol>
}

pub struct SymbolicHarnessConcretization {
    abstract_models: HashSet<SymbolicHarnessModelID>,
    queries: Vec<String>,
    concretization_relation: String,
    mappings: HashMap<String, SymbolicHarnessMapping>,
} 

pub struct SymbolicHarness {
    models: HashMap<SymbolicHarnessModelID, (String, SymbolicSystemModel)>,
    concrete_model: SymbolicHarnessModelID,
    concretization: Option<SymbolicHarnessConcretization>,
    template: HarnessSymbolicTemplate
}

impl SymbolicHarnessMapping {
    pub fn new(source: SymbolicHarnessModelID, target: SymbolicHarnessModelID, mapping: HashMap<SystemModelSymbol, SystemModelSymbol>) -> SymbolicHarnessMapping {
        SymbolicHarnessMapping {
            source_model: source,
            target_model: target,
            mapping
        }
    }

    pub fn add_mapping(&mut self, source_symbol: SystemModelSymbol, target_symbol: SystemModelSymbol) {
        self.mapping.insert(source_symbol, target_symbol);
    }

    pub fn get_source_model(&self) -> SymbolicHarnessModelID {
        self.source_model
    }

    pub fn get_target_model(&self) -> SymbolicHarnessModelID {
        self.target_model
    }

    pub fn get_state_mapping(&self) -> &HashMap<SystemModelSymbol, SystemModelSymbol> {
        &self.mapping
    }

    pub fn build(&self, source_symbols: &SystemModelSymbols, target_symbols: &SystemModelSymbols) -> Result<HashMap<StateMachineNodeID, StateMachineNodeID>, HarnessError> {
        self.mapping.iter()
            .map(| (source_symbol, target_symbol) | {
                let source = source_symbols.get_state(*source_symbol)
                    .ok_or(HarnessError::new("Unable to find mapped process node"))?;
                let target = target_symbols.get_state(*target_symbol)
                    .ok_or(HarnessError::new("Unable to find mapped process node"))?;
                Ok((source, target))
            })
            .collect()
    }
}

impl SymbolicHarnessConcretization {
    pub fn new(concretization_relation: impl Into<String>) -> SymbolicHarnessConcretization {
        SymbolicHarnessConcretization {
            abstract_models: HashSet::new(),
            queries: Vec::new(),
            concretization_relation: concretization_relation.into(),
            mappings: HashMap::new()
        }
    }

    pub fn add_abstract_model(&mut self, harness: &mut SymbolicHarness, model_name: impl Into<String>, model: SymbolicSystemModel) -> SymbolicHarnessModelID {
        let model_id = SymbolicHarnessModelID(harness.models.len() as u64);
        harness.models.insert(model_id, (model_name.into(), model));
        self.abstract_models.insert(model_id);
        model_id
    }

    pub fn add_query(&mut self, query: impl Into<String>) {
        self.queries.push(query.into());
    }

    pub fn add_queries(&mut self, queries: impl Iterator<Item = String>) {
        self.queries.extend(queries);
    }

    pub fn add_mapping(&mut self, mapping_name: impl Into<String>, mapping: SymbolicHarnessMapping) {
        self.mappings.insert(mapping_name.into(), mapping);
    }

    pub fn get_abstract_model_ids(&self) -> &HashSet<SymbolicHarnessModelID> {
        &self.abstract_models
    }

    pub fn get_queries(&self) -> &Vec<String> {
        &self.queries
    }

    pub fn get_concretization_relation(&self) -> &str {
        &self.concretization_relation
    }

    pub fn get_state_mappings(&self) -> &HashMap<String, SymbolicHarnessMapping> {
        &self.mappings
    }
}

impl SymbolicHarness {
    pub fn new(concrete_model_name: impl Into<String>, concrete_model: SymbolicSystemModel, template: HarnessSymbolicTemplate) -> SymbolicHarness {
        let mut models = HashMap::new();
        let concrete_model_id = SymbolicHarnessModelID(models.len() as u64);
        models.insert(concrete_model_id, (concrete_model_name.into(), concrete_model));
        SymbolicHarness {
            models,
            concrete_model: concrete_model_id,
            concretization: None,
            template
        }
    }

    pub fn set_concretization(&mut self, concretization: SymbolicHarnessConcretization) {
        self.concretization = Some(concretization);
    }

    pub fn get_model(&self, model_id: SymbolicHarnessModelID) -> Option<(&str, &SymbolicSystemModel)> {
        self.models.get(&model_id)
            .map(| (name, model) | (name.as_str(), model))
    }

    pub fn get_concrete_model_id(&self) -> SymbolicHarnessModelID {
        self.concrete_model
    }

    pub fn get_concretization(&self) -> Option<&SymbolicHarnessConcretization> {
        self.concretization.as_ref()
    }

    pub fn get_template(&self) -> &HarnessSymbolicTemplate {
        &self.template
    }
}
