use std::collections::HashMap;

use crate::harness::{codegen::template::CodegenTemplate, core::{error::HarnessError, state_machine::StateMachineNodeID}, harness::symbolic_harness::SymbolicHarness, system::model::SystemModel};

pub struct HarnessStateMapping(String, String, HashMap<StateMachineNodeID, StateMachineNodeID>);

pub struct HarnessConcretization {
    abstract_models: HashMap<String, SystemModel>,
    concretization_relation: String,
    queries: Vec<String>,
    mappings: HashMap<String, HarnessStateMapping>
}

pub struct Harness {
    concrete_model: (String, SystemModel),
    concretization: Option<HarnessConcretization>,
    template: CodegenTemplate
}

impl HarnessStateMapping {
    pub fn get_source_model_name(&self) -> &str {
        &self.0
    }

    pub fn get_target_model_name(&self) -> &str {
        &self.1
    }

    pub fn get_mapping(&self) -> &HashMap<StateMachineNodeID, StateMachineNodeID> {
        &self.2
    }
}

impl HarnessConcretization {
    pub fn get_abstract_models(&self) -> &HashMap<String, SystemModel> {
        &self.abstract_models
    }

    pub fn get_concretization_relation(&self) -> &str {
        &self.concretization_relation
    }

    pub fn get_queries(&self) -> &Vec<String> {
        &self.queries
    }

    pub fn get_mappings(&self) -> &HashMap<String, HarnessStateMapping> {
        &self.mappings
    }
}

impl Harness {
    pub fn new(symbolic_harness: &SymbolicHarness) -> Result<Harness, HarnessError> {
        let (concrete_model_name, concrete_symbolic_model) = symbolic_harness.get_model(symbolic_harness.get_concrete_model_id())
            .ok_or(HarnessError::new("Unable to find concrete model in symbolic harness"))?;
        let concrete_model = SystemModel::new(concrete_symbolic_model)?;
        let codegen_template = symbolic_harness.get_template().build(concrete_model.get_symbols())?;

        let concretization = match symbolic_harness.get_concretization() {
            Some(concretization) => {
                let mut abstract_models = HashMap::new();
                for abstract_model_id in concretization.get_abstract_model_ids() {
                    let (abstract_model_name, abstract_model) = symbolic_harness.get_model(*abstract_model_id)
                        .ok_or(HarnessError::new("Unable to find abstract model in symbolic harness"))?;
                    let model = SystemModel::new(abstract_model)?;
                    abstract_models.insert(abstract_model_name.into(), model);
                }
    
                let mut mappings = HashMap::new();
                for (name, mapping) in concretization.get_state_mappings() {
                    let (source_model_name, _) = symbolic_harness.get_model(mapping.get_source_model())
                        .ok_or(HarnessError::new("Unable to find model in symbolic harness"))?;
                    let (target_model_name, _) = symbolic_harness.get_model(mapping.get_target_model())
                        .ok_or(HarnessError::new("Unable to find model in symbolic harness"))?;
                    let source_model = abstract_models.get(source_model_name)
                        .ok_or(HarnessError::new("Unable to find abstract model in harness"))?;
                    let target_model = if target_model_name == concrete_model_name {
                        &concrete_model
                    } else {
                        abstract_models.get(target_model_name)
                            .ok_or(HarnessError::new("Unable to find abstract model in harness"))?
                    };
                    let mapping_build = mapping.build(source_model.get_symbols(), target_model.get_symbols())?;
                    mappings.insert(name.into(), HarnessStateMapping(source_model_name.into(), target_model_name.into(), mapping_build));
                }

                Some(HarnessConcretization {
                    abstract_models,
                    concretization_relation: concretization.get_concretization_relation().into(),
                    queries: concretization.get_queries().clone(),
                    mappings
                })
            },

            None => None
        };

        let harness = Harness {
            concrete_model: (concrete_model_name.into(), concrete_model),
            concretization,
            template: codegen_template
        };
        Ok(harness)
    }

    pub fn get_concrete_model_name(&self) -> &str {
        self.concrete_model.0.as_str()
    }

    pub fn get_concrete_model(&self) -> &SystemModel {
        &self.concrete_model.1
    }

    pub fn get_concretization(&self) -> Option<&HarnessConcretization> {
        self.concretization.as_ref()
    }

    pub fn get_template(&self) -> &CodegenTemplate {
        &self.template
    }
}
