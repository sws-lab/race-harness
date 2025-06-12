use std::{cell::RefCell, collections::HashMap, path::PathBuf, rc::Rc, sync::Arc};

use mlua::IntoLua;

use crate::harness::{core::{error::HarnessError, state_machine::StateMachineMessageEnvelopeBehavior}, dsl::{dsl::DSLInterpreter, lua::parser::LuaDSLParser}, harness::{symbolic_harness::{SymbolicHarness, SymbolicHarnessConcretization, SymbolicHarnessMapping}, template::HarnessSymbolicTemplate}, system::symbolic_model::{SymbolicSystemModel, SystemModelSymbol}};

use super::parser::LuaDSLFragment;

pub struct LuaDSLInterpreter {}

#[derive(Clone)]
struct HarnessModelDSLSymbol {
    harness: HarnessModelDSLValue,
    symbol: SystemModelSymbol
}

#[derive(Clone)]
struct HarnessModelDSLHandle {
    context: SymbolicSystemModel,
    symbols: HashMap<String, SystemModelSymbol>
}

#[derive(Clone)]
struct HarnessModelDSLValue {
    context: Rc<RefCell<HarnessModelDSLHandle>>,
    template: Rc<RefCell<HarnessSymbolicTemplate>>
}

enum EnvelopeBehavior {
    BlockAny = 1,
    BlockSame = 2,
    ReplaceAny = 3,
    ReplaceSame = 4
}

fn into_symbol(value: mlua::Value) -> mlua::Result<SystemModelSymbol> {
    Ok(value.as_userdata()
        .ok_or(mlua::Error::FromLuaConversionError { from: value.type_name(), to: "TemplateSymbolLuaValue".into(), message: None })?
        .borrow::<HarnessModelDSLSymbol>()?
        .symbol)
}

fn into_optional_symbol(value: mlua::Value) -> mlua::Result<Option<SystemModelSymbol>> {
    if value.is_nil() {
        Ok(None)
    } else {
        Ok(Some(value.as_userdata()
            .ok_or(mlua::Error::FromLuaConversionError { from: value.type_name(), to: "TemplateSymbolLuaValue".into(), message: None })?
            .borrow::<HarnessModelDSLSymbol>()?
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

impl HarnessModelDSLSymbol {
    pub fn new(harness: HarnessModelDSLValue, symbol: SystemModelSymbol) -> HarnessModelDSLSymbol {
        HarnessModelDSLSymbol { harness, symbol }
    }
}

impl HarnessModelDSLHandle {
    fn new(builder: SymbolicSystemModel) -> HarnessModelDSLHandle {
        HarnessModelDSLHandle {
            context: builder,
            symbols: HashMap::new()
        }
    }
}

impl mlua::UserData for HarnessModelDSLSymbol {
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
                .map(| pair | -> Result<SystemModelSymbol, mlua::Error> {
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
                .map(| pair | -> Result<SystemModelSymbol, mlua::Error> {
                    let destination = into_symbol(pair?.1)?;
                    Ok(destination)
                })
                .collect::<Result<Vec<_>, _>>()?;

            let symbol = this.harness.context.borrow_mut()
                .context
                .new_product_state(&mnemonic, this.symbol, other_processes.into_iter())
                ?;
            let symbol_value = HarnessModelDSLSymbol::new(this.harness.clone(), symbol);
            this.harness.context.borrow_mut().symbols.insert(mnemonic, symbol);
            Ok(symbol_value)
        });

        methods.add_method("subnode", | _, this, structure: mlua::Table | {
            let structure = structure
                .pairs::<mlua::Value, mlua::Value>()
                .map(| pair | -> Result<SystemModelSymbol, mlua::Error> {
                    let symbol = into_symbol(pair?.1)?;
                    Ok(symbol)
                })
                .collect::<Result<Vec<_>, _>>()?;

            let symbol = this.harness.context.borrow_mut()
                .context
                .new_product_element_state(this.symbol, structure)
                ?;
            let symbol_value = HarnessModelDSLSymbol::new(this.harness.clone(), symbol);
            Ok(symbol_value)
        });
    }
}

impl mlua::UserData for HarnessModelDSLValue {
    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("new_state", | _, this, mnemonic: String | {
            let symbol = this.context.borrow_mut().context.new_primitive_state(&mnemonic)
                ?;
            let symbol_value = HarnessModelDSLSymbol::new(this.clone(), symbol);
            this.context.borrow_mut().symbols.insert(mnemonic, symbol);
            Ok(symbol_value)
        });

        methods.add_method("new_message", | _, this, mnemonic: String | {
            let symbol = this.context.borrow_mut().context.new_message(&mnemonic)
                ?;
            let symbol_value = HarnessModelDSLSymbol::new(this.clone(), symbol);
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
            let symbol_value = HarnessModelDSLSymbol::new(this.clone(), symbol);
            this.context.borrow_mut().symbols.insert(mnemonic, symbol);
            Ok(symbol_value)
        });

        methods.add_method("new_process", | _, this, (mnemonic, entry): (String, mlua::Value) | {
            let symbol = this.context.borrow_mut().context.new_process(&mnemonic, into_symbol(entry)?)
                ?;
            let symbol_value = HarnessModelDSLSymbol::new(this.clone(), symbol);
            this.context.borrow_mut().symbols.insert(mnemonic, symbol);
            Ok(symbol_value)
        });

        methods.add_method("executable", | _, this, executable: bool | {
            this.template.borrow_mut().set_executable(executable);
            Ok(())
        });

        methods.add_method("clone", | _, this, () | {
            let new_context = this.context.borrow().clone();
            Ok(HarnessModelDSLValue {
                context: Rc::new(RefCell::new(new_context)),
                template: this.template.clone()
            })
        });

        methods.add_meta_method("__index", | lua, this, key: String | {
            this.context.borrow().symbols.get(&key)
                .map(| symbol | HarnessModelDSLSymbol::new(this.clone(), *symbol))
                .map(| symbol | symbol.into_lua(lua))
                .unwrap_or(Ok(mlua::Value::Nil))
        });
    }
}

impl DSLInterpreter for LuaDSLInterpreter {
    type DSLUnit = Vec<LuaDSLFragment>;

    fn parse<Input>(&self, input: &mut Input) -> Result<Self::DSLUnit, HarnessError>
            where Input: Iterator<Item = Result<char, HarnessError>> {
        LuaDSLParser::parse(input)
    }

    fn interpret(&self, fragments: &Self::DSLUnit, include_base_path: Option<PathBuf>) -> Result<SymbolicHarness, HarnessError> {
        let harness = HarnessModelDSLValue {
            context: Rc::new(RefCell::new(HarnessModelDSLHandle::new(SymbolicSystemModel::new()))),
            template: Rc::new(RefCell::new(HarnessSymbolicTemplate::new()))
        };
        let mut lua = mlua::Lua::new();
        Self::initialize(harness.template.clone(), &mut lua, include_base_path)?;
        lua.globals().set("__task_model", harness.clone())?;
        lua.globals().set("__abstract_models", lua.create_table()?)?;
        lua.globals().set("__mappings", lua.create_table()?)?;
        lua.globals().set("__queries", lua.create_table()?)?;
        Self::interpret_template(fragments.iter(), &harness.template, &mut lua)?;

        let abstract_models_table = lua.globals().get::<mlua::Table>("__abstract_models")?;
        let mut abstract_models = HashMap::new();
        for pair in abstract_models_table.pairs::<String, mlua::AnyUserData>() {
            let (model_name, model) = pair?;
            let model = model.borrow_mut::<HarnessModelDSLValue>()?;
            let model = model.context.replace(HarnessModelDSLHandle::new(SymbolicSystemModel::new())).context;
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
                let from_symbol = from_symbol.borrow::<HarnessModelDSLSymbol>()?;
                let to_symbol = to_symbol.borrow::<HarnessModelDSLSymbol>()?;
                mapping.insert(from_symbol.symbol, to_symbol.symbol);
            }

            mappings.insert(mapping_name, (mapping_source_model, mapping_target_model, mapping));
        }

        let quries_table = lua.globals().get::<mlua::Table>("__queries")?;
        let mut queries = Vec::new();
        for pair in quries_table.pairs::<mlua::Value, String>() {
            let (_, query) = pair?;
            queries.push(query);
        }

        let concretization = lua.globals().get::<Option<String>>("__concretization")?;

        let model: mlua::AnyUserData = lua.globals().get("__task_model")?;
        let model = model.borrow_mut::<HarnessModelDSLValue>()?;
        let template = model.template.replace(HarnessSymbolicTemplate::new());
        let model = model.context.replace(HarnessModelDSLHandle::new(SymbolicSystemModel::new())).context;

        let mut symbolic_harness = SymbolicHarness::new("concrete", model, template);
        if let Some(concretization_relation) = concretization {
            let mut model_index = HashMap::new();
            model_index.insert("concrete".into(), symbolic_harness.get_concrete_model_id());
            let mut symbolic_harness_concretization = SymbolicHarnessConcretization::new(concretization_relation);
            abstract_models.into_iter()
                .for_each(| (model_name, model) | {
                    let model_id = symbolic_harness_concretization.add_abstract_model(&mut symbolic_harness, &model_name, model);
                    model_index.insert(model_name, model_id);
                });
            symbolic_harness_concretization.add_queries(queries.into_iter());
            for (mapping_name, (source_model, target_model, mapping)) in mappings {
                let source_model_id = model_index.get(&source_model)
                    .ok_or(HarnessError::new("Unable to find source model for state mapping"))?;
                let target_model_id = model_index.get(&target_model)
                    .ok_or(HarnessError::new("Unable to find target model for state mapping"))?;
                symbolic_harness_concretization.add_mapping(mapping_name, SymbolicHarnessMapping::new(*source_model_id, *target_model_id, mapping));
            }
            
            symbolic_harness.set_concretization(symbolic_harness_concretization);
        }

        Ok(symbolic_harness)
    }
}

impl LuaDSLInterpreter {
    pub fn new() -> Self {
        Self {}
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
                let context = HarnessModelDSLValue {
                    context: Rc::new(RefCell::new(HarnessModelDSLHandle::new(SymbolicSystemModel::new()))),
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

    fn interpret_template<'a>(fragments: impl Iterator<Item = &'a LuaDSLFragment>, template: &RefCell<HarnessSymbolicTemplate>, lua: &mut mlua::Lua) -> Result<(), HarnessError> {
        for fragment in fragments {
            match fragment {
                LuaDSLFragment::Verbatim(content) =>
                    template.borrow_mut().append_global_prologue(content.into()),

                LuaDSLFragment::Interpreted(code) =>
                    lua.load(code).exec()?
            }
        }

        Ok(())
    }
}
