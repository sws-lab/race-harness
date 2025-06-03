use std::{cell::RefCell, collections::HashMap, path::PathBuf, rc::Rc, sync::Arc};

use mlua::{AnyUserData, IntoLua};

use crate::harness::{core::{error::HarnessError, state_machine::StateMachineMessageEnvelopeBehavior}, frontend::{model::{HarnessModelSymbol, HarnessModel}, template::HarnessTemplate}};

use super::parser::TemplateFragment;

pub struct LuaTemplateInterpreter {}

#[derive(Clone)]
struct TemplateSymbolLuaValue {
    harness: HarnessContextValue,
    symbol: HarnessModelSymbol
}

#[derive(Clone)]
struct HarnessModelHandle {
    context: HarnessModel,
    symbols: HashMap<String, HarnessModelSymbol>
}

#[derive(Clone)]
struct HarnessContextValue {
    context: Rc<RefCell<HarnessModelHandle>>,
    template: Rc<RefCell<HarnessTemplate>>
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
    
fn map_lua_error(err: mlua::Error) -> HarnessError {
    HarnessError::new(err.to_string())
}

fn map_harness_error(err: HarnessError) -> mlua::Error {
    mlua::Error::ExternalError(Arc::new(err))
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
    fn new(builder: HarnessModel) -> HarnessModelHandle {
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
                .map_err(map_harness_error)?;
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
                .map_err(map_harness_error)?;
            Ok(this.clone())
        });

        methods.add_method("respond", | _, this, (behavior, message): (EnvelopeBehavior, mlua::Value) | {
            this.harness.context.borrow_mut()
                .context
                .new_response_envelope(this.symbol, behavior.into(), into_symbol(message)?)
                .map_err(map_harness_error)?;
            Ok(this.clone())
        });

        methods.add_method("exec", | _, this, action: String | {
            this.harness.template.borrow_mut()
                .set_action_content(this.symbol, action)
                .map_err(map_harness_error)?;
            Ok(this.clone())
        });

        methods.add_method("setup", | _, this, prologue: String | {
            this.harness.template.borrow_mut()
                .append_process_prologue(this.symbol, prologue)
                .map_err(map_harness_error)?;
            Ok(this.clone())
        });

        methods.add_meta_method("__newindex", | _, this, (key, value): (String, mlua::Value) | {
            this.harness.template.borrow_mut()
                .set_process_parameter(this.symbol, key, value.to_string()?)
                .map_err(map_harness_error)?;
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
                .map_err(map_harness_error)?;
            let symbol_value = TemplateSymbolLuaValue::new(this.harness.clone(), symbol);
            this.harness.context.borrow_mut().symbols.insert(mnemonic, symbol);
            Ok(symbol_value)
        });
    }
}

impl mlua::UserData for HarnessContextValue {
    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("new_state", | _, this, mnemonic: String | {
            let symbol = this.context.borrow_mut().context.new_primitive_state(&mnemonic)
                .map_err(map_harness_error)?;
            let symbol_value = TemplateSymbolLuaValue::new(this.clone(), symbol);
            this.context.borrow_mut().symbols.insert(mnemonic, symbol);
            Ok(symbol_value)
        });

        methods.add_method("new_message", | _, this, mnemonic: String | {
            let symbol = this.context.borrow_mut().context.new_message(&mnemonic)
                .map_err(map_harness_error)?;
            let symbol_value = TemplateSymbolLuaValue::new(this.clone(), symbol);
            this.context.borrow_mut().symbols.insert(mnemonic, symbol);
            Ok(symbol_value)
        });

        methods.add_method("new_edge", | _, this, (source, target, trigger, action): (mlua::Value, mlua::Value, mlua::Value, mlua::Value)| {
            this.context.borrow_mut().context.new_edge(into_symbol(source)?, into_symbol(target)?, into_optional_symbol(trigger)?, into_optional_symbol(action)?)
                .map_err(map_harness_error)?;
            Ok(())
        });

        methods.add_method("new_action", | _, this, mnemonic: String | {
            let symbol = this.context.borrow_mut().context.new_action(&mnemonic)
                .map_err(map_harness_error)?;
            let symbol_value = TemplateSymbolLuaValue::new(this.clone(), symbol);
            this.context.borrow_mut().symbols.insert(mnemonic, symbol);
            Ok(symbol_value)
        });

        methods.add_method("new_process", | _, this, (mnemonic, entry): (String, mlua::Value) | {
            let symbol = this.context.borrow_mut().context.new_process(&mnemonic, into_symbol(entry)?)
                .map_err(map_harness_error)?;
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

impl LuaTemplateInterpreter {
    pub fn new() -> LuaTemplateInterpreter {
        LuaTemplateInterpreter {}
    }

    fn initialize<'a, 'b: 'a>(&self, harness: HarnessContextValue, lua: &'b mut mlua::Lua, include_base_path: Option<PathBuf>) -> Result<(), HarnessError> {
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
            }).map_err(map_lua_error)?;
            lua.globals().set("include", include_fn).map_err(map_lua_error)?;
        }

        {
            let new_task_model_fn = lua.create_function(move | _, () | {
                let context = HarnessContextValue {
                    context: Rc::new(RefCell::new(HarnessModelHandle::new(HarnessModel::new()))),
                    template: harness.template.clone()
                };
                Ok(context)
            }).map_err(map_lua_error)?;
            lua.globals().set("new_task_model", new_task_model_fn).map_err(map_lua_error)?;
        }

        lua.globals().set("BLOCK_ANY", EnvelopeBehavior::BlockAny).map_err(map_lua_error)?;
        lua.globals().set("BLOCK_SAME", EnvelopeBehavior::BlockSame).map_err(map_lua_error)?;
        lua.globals().set("REPLACE_ANY", EnvelopeBehavior::ReplaceAny).map_err(map_lua_error)?;
        lua.globals().set("REPLACE_SAME", EnvelopeBehavior::ReplaceSame).map_err(map_lua_error)?;

        lua.load(include_str!("prelude.lua")).exec().map_err(map_lua_error)?;

        Ok(())
    }

    fn interpret_template<'a>(&self, fragments: impl Iterator<Item = TemplateFragment>, harness: HarnessContextValue, lua: &mut mlua::Lua) -> Result<(), HarnessError> {
        lua.globals().set("__task_model", harness.clone()).map_err(map_lua_error)?;
        for fragment in fragments {
            match fragment {
                TemplateFragment::Verbatim(content) =>
                    harness.template.borrow_mut().append_global_prologue(content),

                TemplateFragment::Interpreted(code) =>
                    lua.load(code).exec().map_err(map_lua_error)?
            }
        }

        Ok(())
    }

    pub fn interpret(&mut self, fragments: impl Iterator<Item = TemplateFragment>, include_base_path: Option<PathBuf>) -> Result<(HarnessModel, HarnessTemplate), HarnessError> {
        let harness = HarnessContextValue {
            context: Rc::new(RefCell::new(HarnessModelHandle::new(HarnessModel::new()))),
            template: Rc::new(RefCell::new(HarnessTemplate::new()))
        };
        let mut lua = mlua::Lua::new();
        self.initialize(harness.clone(), &mut lua, include_base_path)?;
        self.interpret_template(fragments, harness, &mut lua)?;
        let context: AnyUserData = lua.globals().get("__task_model").map_err(map_lua_error)?;
        let context = context.borrow_mut::<HarnessContextValue>().map_err(map_lua_error)?;
        Ok((
            context.context.replace(HarnessModelHandle::new(HarnessModel::new())).context,
            context.template.replace(HarnessTemplate::new())
        ))
    }
}