use std::{cell::RefCell, collections::HashMap, path::PathBuf, rc::Rc, sync::Arc};

use mlua::{AnyUserData, IntoLua};

use crate::harness::{core::{error::HarnessError, state_machine::{StateMachineMessageEnvelopeBehavior, StateMachineNodeID}}, frontend::{symbolic_model::{HarnessModelSymbol, HarnessSymbolicModel, HarnessSymbolicModelBuild}, template::HarnessSymbolicTemplate}};

use super::parser::TemplateFragment;

pub struct HarnessSymbolicModelMapping {
    source_model: String,
    target_model: String,
    mapping: HashMap<HarnessModelSymbol, HarnessModelSymbol>
}

pub struct InterpretedLuaModelTemplate {
    concrete_model: HarnessSymbolicModel,
    abstract_models: HashMap<String, HarnessSymbolicModel>,
    queries: Vec<String>,
    concretization: Option<String>,
    mappings: HashMap<String, HarnessSymbolicModelMapping>,
    template: HarnessSymbolicTemplate
}

#[derive(Clone)]
struct TemplateSymbolLuaValue {
    harness: HarnessContextValue,
    symbol: HarnessModelSymbol
}

#[derive(Clone)]
struct HarnessModelHandle {
    context: HarnessSymbolicModel,
    symbols: HashMap<String, HarnessModelSymbol>
}

#[derive(Clone)]
struct HarnessContextValue {
    context: Rc<RefCell<HarnessModelHandle>>,
    template: Rc<RefCell<HarnessSymbolicTemplate>>
}

enum EnvelopeBehavior {
    BlockAny = 1,
    BlockSame = 2,
    ReplaceAny = 3,
    ReplaceSame = 4
}

fn into_symbol(value: mlua::Value) -> mlua::Result<HarnessModelSymbol> {
    Ok(value.as_userdata()
        .ok_or(mlua::Error::FromLuaConversionError { from: value.type_name(), to: "TemplateSymbolLuaValue".into(), message: None })?
        .borrow::<TemplateSymbolLuaValue>()?
        .symbol)
}

fn into_optional_symbol(value: mlua::Value) -> mlua::Result<Option<HarnessModelSymbol>> {
    if value.is_nil() {
        Ok(None)
    } else {
        Ok(Some(value.as_userdata()
            .ok_or(mlua::Error::FromLuaConversionError { from: value.type_name(), to: "TemplateSymbolLuaValue".into(), message: None })?
            .borrow::<TemplateSymbolLuaValue>()?
            .symbol))
    }
}

impl From<mlua::Error> for HarnessError {
    fn from(value: mlua::Error) -> Self {
        HarnessError::new(value.to_string())
    }
}

impl From<HarnessError> for mlua::Error {
    fn from(value: HarnessError) -> Self {
        mlua::Error::ExternalError(Arc::new(value))
    }
}

impl mlua::IntoLua for EnvelopeBehavior {
    fn into_lua(self, _: &mlua::Lua) -> mlua::Result<mlua::Value> {
        Ok(mlua::Value::Integer(self as i64))
    }
}

impl mlua::FromLua for EnvelopeBehavior {
    fn from_lua(value: mlua::Value, _: &mlua::Lua) -> mlua::Result<Self> {
        let ivalue = value.as_i64().ok_or(mlua::Error::FromLuaConversionError { from: value.type_name(), to: "EnvelopeBehavior".into(), message: None })?;
        match ivalue {
            x if x == EnvelopeBehavior::BlockAny as i64 => Ok(EnvelopeBehavior::BlockAny),
            x if x == EnvelopeBehavior::BlockSame as i64 => Ok(EnvelopeBehavior::BlockSame),
            x if x == EnvelopeBehavior::ReplaceAny as i64 => Ok(EnvelopeBehavior::ReplaceAny),
            x if x == EnvelopeBehavior::ReplaceSame as i64 => Ok(EnvelopeBehavior::ReplaceSame),
            _ => Err(mlua::Error::FromLuaConversionError { from: value.type_name(), to: "EnvelopeBehavior".into(), message: None })
        }
    }
}

impl From<EnvelopeBehavior> for StateMachineMessageEnvelopeBehavior {
    fn from(value: EnvelopeBehavior) -> Self {
        match value {
            EnvelopeBehavior::BlockAny => StateMachineMessageEnvelopeBehavior::BlockAnyMessage,
            EnvelopeBehavior::BlockSame => StateMachineMessageEnvelopeBehavior::BlockSameMessage,
            EnvelopeBehavior::ReplaceAny => StateMachineMessageEnvelopeBehavior::ReplaceAnyMessage,
            EnvelopeBehavior::ReplaceSame => StateMachineMessageEnvelopeBehavior::ReplaceSameMessage
        }
    }
}

impl TemplateSymbolLuaValue {
    pub fn new(harness: HarnessContextValue, symbol: HarnessModelSymbol) -> TemplateSymbolLuaValue {
        TemplateSymbolLuaValue { harness, symbol }
    }
}

impl HarnessModelHandle {
    fn new(builder: HarnessSymbolicModel) -> HarnessModelHandle {
        HarnessModelHandle {
            context: builder,
            symbols: HashMap::new()
        }
    }
}

impl mlua::UserData for TemplateSymbolLuaValue {
    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("unicast", | _, this, (destination, behavior, message): (mlua::Value, EnvelopeBehavior, mlua::Value) | {
            this.harness.context.borrow_mut()
                .context
                .new_unicast_envelope(this.symbol, into_symbol(destination)?, behavior.into(), into_symbol(message)?)
                ?;
            Ok(this.clone())
        });

        methods.add_method("multicast", | _, this, (destinations, behavior, message): (mlua::Value, EnvelopeBehavior, mlua::Value) | {
            let destinations = destinations.as_table()
                .ok_or(mlua::Error::FromLuaConversionError { from: destinations.type_name(), to: "[TemplateSymbolLuaValue]".into(), message: None })?
                .pairs::<mlua::Value, mlua::Value>()
                .map(| pair | -> Result<HarnessModelSymbol, mlua::Error> {
                    let destination = into_symbol(pair?.1)?;
                    Ok(destination)
                })
                .collect::<Result<Vec<_>, _>>()?;
            this.harness.context.borrow_mut()
                .context
                .new_multicast_envelope(this.symbol, destinations.into_iter(), behavior.into(), into_symbol(message)?)
                ?;
            Ok(this.clone())
        });

        methods.add_method("respond", | _, this, (behavior, message): (EnvelopeBehavior, mlua::Value) | {
            this.harness.context.borrow_mut()
                .context
                .new_response_envelope(this.symbol, behavior.into(), into_symbol(message)?)
                ?;
            Ok(this.clone())
        });

        methods.add_method("exec", | _, this, action: String | {
            this.harness.template.borrow_mut()
                .set_action_content(this.symbol, action)
                ?;
            Ok(this.clone())
        });

        methods.add_method("setup", | _, this, prologue: String | {
            this.harness.template.borrow_mut()
                .append_process_prologue(this.symbol, prologue)
                ?;
            Ok(this.clone())
        });

        methods.add_meta_method("__newindex", | _, this, (key, value): (String, mlua::Value) | {
            this.harness.template.borrow_mut()
                .set_process_parameter(this.symbol, key, value.to_string()?)
                ?;
            Ok(this.clone())
        });

        methods.add_method("product", | _, this, (mnemonic, other_processes): (String, mlua::Value) | {
            let other_processes = other_processes.as_table()
                .ok_or(mlua::Error::FromLuaConversionError { from: other_processes.type_name(), to: "[TemplateSymbolLuaValue]".into(), message: None })?
                .pairs::<mlua::Value, mlua::Value>()
                .map(| pair | -> Result<HarnessModelSymbol, mlua::Error> {
                    let destination = into_symbol(pair?.1)?;
                    Ok(destination)
                })
                .collect::<Result<Vec<_>, _>>()?;

            let symbol = this.harness.context.borrow_mut()
                .context
                .new_product_state(&mnemonic, this.symbol, other_processes.into_iter())
                ?;
            let symbol_value = TemplateSymbolLuaValue::new(this.harness.clone(), symbol);
            this.harness.context.borrow_mut().symbols.insert(mnemonic, symbol);
            Ok(symbol_value)
        });

        methods.add_method("subnode", | _, this, structure: mlua::Table | {
            let structure = structure
                .pairs::<mlua::Value, mlua::Value>()
                .map(| pair | -> Result<HarnessModelSymbol, mlua::Error> {
                    let symbol = into_symbol(pair?.1)?;
                    Ok(symbol)
                })
                .collect::<Result<Vec<_>, _>>()?;

            let symbol = this.harness.context.borrow_mut()
                .context
                .new_product_element_state(this.symbol, structure)
                ?;
            let symbol_value = TemplateSymbolLuaValue::new(this.harness.clone(), symbol);
            Ok(symbol_value)
        });
    }
}

impl mlua::UserData for HarnessContextValue {
    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("new_state", | _, this, mnemonic: String | {
            let symbol = this.context.borrow_mut().context.new_primitive_state(&mnemonic)
                ?;
            let symbol_value = TemplateSymbolLuaValue::new(this.clone(), symbol);
            this.context.borrow_mut().symbols.insert(mnemonic, symbol);
            Ok(symbol_value)
        });

        methods.add_method("new_message", | _, this, mnemonic: String | {
            let symbol = this.context.borrow_mut().context.new_message(&mnemonic)
                ?;
            let symbol_value = TemplateSymbolLuaValue::new(this.clone(), symbol);
            this.context.borrow_mut().symbols.insert(mnemonic, symbol);
            Ok(symbol_value)
        });

        methods.add_method("new_edge", | _, this, (source, target, trigger, action): (mlua::Value, mlua::Value, mlua::Value, mlua::Value)| {
            this.context.borrow_mut().context.new_edge(into_symbol(source)?, into_symbol(target)?, into_optional_symbol(trigger)?, into_optional_symbol(action)?)
                ?;
            Ok(())
        });

        methods.add_method("new_action", | _, this, mnemonic: String | {
            let symbol = this.context.borrow_mut().context.new_action(&mnemonic)
                ?;
            let symbol_value = TemplateSymbolLuaValue::new(this.clone(), symbol);
            this.context.borrow_mut().symbols.insert(mnemonic, symbol);
            Ok(symbol_value)
        });

        methods.add_method("new_process", | _, this, (mnemonic, entry): (String, mlua::Value) | {
            let symbol = this.context.borrow_mut().context.new_process(&mnemonic, into_symbol(entry)?)
                ?;
            let symbol_value = TemplateSymbolLuaValue::new(this.clone(), symbol);
            this.context.borrow_mut().symbols.insert(mnemonic, symbol);
            Ok(symbol_value)
        });

        methods.add_method("executable", | _, this, executable: bool | {
            this.template.borrow_mut().set_executable(executable);
            Ok(())
        });

        methods.add_method("clone", | _, this, () | {
            let new_context = this.context.borrow().clone();
            Ok(HarnessContextValue {
                context: Rc::new(RefCell::new(new_context)),
                template: this.template.clone()
            })
        });

        methods.add_meta_method("__index", | lua, this, key: String | {
            this.context.borrow().symbols.get(&key)
                .map(| symbol | TemplateSymbolLuaValue::new(this.clone(), *symbol))
                .map(| symbol | symbol.into_lua(lua))
                .unwrap_or(Ok(mlua::Value::Nil))
        });
    }
}

impl HarnessSymbolicModelMapping {
    pub fn get_source_model_name(&self) -> &str {
        &self.source_model
    }

    pub fn get_target_model_name(&self) -> &str {
        &self.target_model
    }

    pub fn get_mapping(&self) -> &HashMap<HarnessModelSymbol, HarnessModelSymbol> {
        &self.mapping
    }

    pub fn build(&self, source_build: &HarnessSymbolicModelBuild, target_build: &HarnessSymbolicModelBuild) -> Result<HashMap<StateMachineNodeID, StateMachineNodeID>, HarnessError> {
        self.mapping.iter()
            .map(| (source_symbol, target_symbol) | {
                let source = source_build.get_state(*source_symbol)
                    .ok_or(HarnessError::new("Unable to find mapped process node"))?;
                let target = target_build.get_state(*target_symbol)
                    .ok_or(HarnessError::new("Unable to find mapped process node"))?;
                Ok((source, target))
            })
            .collect()
    }
}

impl InterpretedLuaModelTemplate {
    pub fn new(fragments: impl Iterator<Item= TemplateFragment>, include_base_path: Option<PathBuf>) -> Result<InterpretedLuaModelTemplate, HarnessError> {
        let harness = HarnessContextValue {
            context: Rc::new(RefCell::new(HarnessModelHandle::new(HarnessSymbolicModel::new()))),
            template: Rc::new(RefCell::new(HarnessSymbolicTemplate::new()))
        };
        let mut lua = mlua::Lua::new();
        Self::initialize(harness.template.clone(), &mut lua, include_base_path)?;
        lua.globals().set("__task_model", harness.clone())?;
        lua.globals().set("__abstract_models", lua.create_table()?)?;
        lua.globals().set("__mappings", lua.create_table()?)?;
        lua.globals().set("__queries", lua.create_table()?)?;
        Self::interpret_template(fragments, &harness.template, &mut lua)?;

        let abstract_models_table = lua.globals().get::<mlua::Table>("__abstract_models")?;
        let mut abstract_models = HashMap::default();
        for pair in abstract_models_table.pairs::<String, mlua::AnyUserData>() {
            let (model_name, model) = pair?;
            let model = model.borrow_mut::<HarnessContextValue>()?;
            let model = model.context.replace(HarnessModelHandle::new(HarnessSymbolicModel::new())).context;
            abstract_models.insert(model_name, model);
        }

        let mappings_table = lua.globals().get::<mlua::Table>("__mappings")?;
        let mut mappings = HashMap::new();
        for pair in mappings_table.pairs::<String, mlua::Table>() {
            let (mapping_name, mapping_tuple) = pair?;
            let mapping_source_model = mapping_tuple.get::<String>(1)?;
            let mapping_target_model = mapping_tuple.get::<String>(2)?;
            let mapping_table = mapping_tuple.get::<mlua::Table>(3)?;
            let mut mapping = HashMap::new();
            for mapping_pair in mapping_table.pairs() {
                let (from_symbol, to_symbol): (mlua::AnyUserData, mlua::AnyUserData) = mapping_pair?;
                let from_symbol = from_symbol.borrow::<TemplateSymbolLuaValue>()?;
                let to_symbol = to_symbol.borrow::<TemplateSymbolLuaValue>()?;
                mapping.insert(from_symbol.symbol, to_symbol.symbol);
            }

            mappings.insert(mapping_name, HarnessSymbolicModelMapping {
                source_model: mapping_source_model,
                target_model: mapping_target_model,
                mapping
            });
        }

        let quries_table = lua.globals().get::<mlua::Table>("__queries")?;
        let mut queries = Vec::new();
        for pair in quries_table.pairs::<mlua::Value, String>() {
            let (_, query) = pair?;
            queries.push(query);
        }

        let concretization = lua.globals().get::<Option<String>>("__concretization")?;

        let model: AnyUserData = lua.globals().get("__task_model")?;
        let model = model.borrow_mut::<HarnessContextValue>()?;
        let template = model.template.replace(HarnessSymbolicTemplate::new());
        let model = model.context.replace(HarnessModelHandle::new(HarnessSymbolicModel::new())).context;

        Ok(InterpretedLuaModelTemplate {
            concrete_model: model,
            abstract_models,
            queries,
            concretization,
            mappings,
            template
        })
    }

    pub fn get_concrete_model(&self) -> &HarnessSymbolicModel {
        &self.concrete_model
    }

    pub fn get_concretization(&self) -> Option<&str> {
        self.concretization.as_deref()
    }

    pub fn get_abstract_models(&self) -> impl Iterator<Item = (&str, &HarnessSymbolicModel)> {
        self.abstract_models.iter()
            .map(| (name, model ) | (name.as_str(), model))
    }

    pub fn get_mappings(&self) -> &HashMap<String, HarnessSymbolicModelMapping> {
        &self.mappings
    }

    pub fn get_template(&self) -> &HarnessSymbolicTemplate {
        &self.template
    }

    pub fn get_queries(&self) -> impl Iterator<Item = &str> {
        self.queries.iter().map(| x | x.as_str())
    }

    fn initialize(template: Rc<RefCell<HarnessSymbolicTemplate>>, lua: &mut mlua::Lua, include_base_path: Option<PathBuf>) -> Result<(), HarnessError> {
        {
            let include_fn = lua.create_function(move |lua, filepath: String | {
                let path = std::path::Path::new(&filepath);
                let pathbuf = if path.is_relative() && include_base_path.is_some() {
                    let mut basepath = include_base_path.clone().unwrap();
                    basepath.push(path);
                    basepath
                } else {
                    path.to_path_buf()
                };
                lua.load(pathbuf).exec()
            })?;
            lua.globals().set("include", include_fn)?;
        }

        {
            let new_task_model_fn = lua.create_function(move | _, () | {
                let context = HarnessContextValue {
                    context: Rc::new(RefCell::new(HarnessModelHandle::new(HarnessSymbolicModel::new()))),
                    template: template.clone()
                };
                Ok(context)
            })?;
            lua.globals().set("new_task_model", new_task_model_fn)?;
        }

        lua.globals().set("BLOCK_ANY", EnvelopeBehavior::BlockAny)?;
        lua.globals().set("BLOCK_SAME", EnvelopeBehavior::BlockSame)?;
        lua.globals().set("REPLACE_ANY", EnvelopeBehavior::ReplaceAny)?;
        lua.globals().set("REPLACE_SAME", EnvelopeBehavior::ReplaceSame)?;

        lua.load(include_str!("prelude.lua")).exec()?;

        Ok(())
    }

    fn interpret_template(fragments: impl Iterator<Item = TemplateFragment>, template: &RefCell<HarnessSymbolicTemplate>, lua: &mut mlua::Lua) -> Result<(), HarnessError> {
        for fragment in fragments {
            match fragment {
                TemplateFragment::Verbatim(content) =>
                    template.borrow_mut().append_global_prologue(content),

                TemplateFragment::Interpreted(code) =>
                    lua.load(code).exec()?
            }
        }

        Ok(())
    }
}
