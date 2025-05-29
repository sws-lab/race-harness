use std::{cell::RefCell, rc::Rc, sync::Arc};

use crate::harness::{builder::builder::{HarnessBuilder, HarnessBuilderSymbol}, core::error::HarnessError};

use super::parser::TemplateFragment;

pub struct LuaTemplateInterpreter {}

#[derive(Clone)]
struct TemplateSymbolLuaValue {
    harness: Rc<RefCell<HarnessBuilder>>,
    symbol: HarnessBuilderSymbol
}

fn into_symbol(value: mlua::Value) -> mlua::Result<HarnessBuilderSymbol> {
    Ok(value.as_userdata()
        .ok_or(mlua::Error::FromLuaConversionError { from: value.type_name(), to: "TemplateSymbolLuaValue".into(), message: None })?
        .borrow::<TemplateSymbolLuaValue>()?
        .symbol)
}

fn into_optional_symbol(value: mlua::Value) -> mlua::Result<Option<HarnessBuilderSymbol>> {
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

impl TemplateSymbolLuaValue {
    pub fn new(harness: Rc<RefCell<HarnessBuilder>>, symbol: HarnessBuilderSymbol) -> TemplateSymbolLuaValue {
        TemplateSymbolLuaValue { harness, symbol }
    }
}

impl mlua::UserData for TemplateSymbolLuaValue {
    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("unicast", | _, this, (destination, message): (mlua::Value, mlua::Value) | {
            this.harness.borrow_mut()
                .new_unicast_envelope(this.symbol, into_symbol(destination)?, into_symbol(message)?)
                .map_err(map_harness_error)?;
            Ok(this.clone())
        });

        methods.add_method("multicast", | _, this, (destinations, message): (mlua::Value, mlua::Value) | {
            let destinations = destinations.as_table()
                .ok_or(mlua::Error::FromLuaConversionError { from: destinations.type_name(), to: "[TemplateSymbolLuaValue]".into(), message: None })?
                .pairs::<mlua::Value, mlua::Value>()
                .map(| pair | -> Result<HarnessBuilderSymbol, mlua::Error> {
                    let destination = into_symbol(pair?.1)?;
                    Ok(destination)
                })
                .collect::<Result<Vec<_>, _>>()?;
            this.harness.borrow_mut()
                .new_multicast_envelope(this.symbol, destinations.into_iter(), into_symbol(message)?)
                .map_err(map_harness_error)?;
            Ok(this.clone())
        });

        methods.add_method("respond", | _, this, message: mlua::Value | {
            this.harness.borrow_mut()
                .new_response_envelope(this.symbol, into_symbol(message)?)
                .map_err(map_harness_error)?;
            Ok(this.clone())
        });

        methods.add_method("exec", | _, this, action: String | {
            this.harness.borrow_mut()
                .set_action_content(this.symbol, action)
                .map_err(map_harness_error)?;
            Ok(this.clone())
        });

        methods.add_method("setup", | _, this, prologue: String | {
            this.harness.borrow_mut()
                .append_process_prologue(this.symbol, prologue)
                .map_err(map_harness_error)?;
            Ok(this.clone())
        });

        methods.add_meta_method("__newindex", | _, this, (key, value): (String, mlua::Value) | {
            this.harness.borrow_mut()
                .set_process_parameter(this.symbol, key, value.to_string()?)
                .map_err(map_harness_error)?;
            Ok(this.clone())
        });

        methods.add_method("product", | lua, this, (mnemonic, other_processes): (String, mlua::Value) | {
            let other_processes = other_processes.as_table()
                .ok_or(mlua::Error::FromLuaConversionError { from: other_processes.type_name(), to: "[TemplateSymbolLuaValue]".into(), message: None })?
                .pairs::<mlua::Value, mlua::Value>()
                .map(| pair | -> Result<HarnessBuilderSymbol, mlua::Error> {
                    let destination = into_symbol(pair?.1)?;
                    Ok(destination)
                })
                .collect::<Result<Vec<_>, _>>()?;

            let symbol = this.harness.borrow_mut()
                .new_product_state(&mnemonic, this.symbol, other_processes.into_iter())
                .map_err(map_harness_error)?;
            let symbol_value = TemplateSymbolLuaValue::new(this.harness.clone(), symbol);
            lua.globals().set(mnemonic, symbol_value.clone())?;
            Ok(symbol_value)
        });
    }
}

impl LuaTemplateInterpreter {
    pub fn new() -> LuaTemplateInterpreter {
        LuaTemplateInterpreter {}
    }

    fn initialize<'a, 'b: 'a>(&self, harness: Rc<RefCell<HarnessBuilder>>, lua: &'b mut mlua::Lua) -> Result<(), HarnessError> {
        {
            let harness = harness.clone();
            let new_state_fn = lua.create_function(move |lua, mnemonic: String| {
                let symbol = harness.borrow_mut().new_primitive_state(&mnemonic)
                    .map_err(map_harness_error)?;
                let symbol_value = TemplateSymbolLuaValue::new(harness.clone(), symbol);
                lua.globals().set(mnemonic, symbol_value.clone())?;
                Ok(symbol_value)
            }).map_err(map_lua_error)?;
            lua.globals().set("S", new_state_fn).map_err(map_lua_error)?;
        }
        {
            let harness = harness.clone();
            let new_message_fn = lua.create_function(move |lua, mnemonic: String| {
                let symbol = harness.borrow_mut().new_message(&mnemonic)
                    .map_err(map_harness_error)?;
                let symbol_value = TemplateSymbolLuaValue::new(harness.clone(), symbol);
                lua.globals().set(mnemonic, symbol_value.clone())?;
                Ok(symbol_value)
            }).map_err(map_lua_error)?;
            lua.globals().set("M", new_message_fn).map_err(map_lua_error)?;
        }
        {
            let harness = harness.clone();
            let new_edge_fn = lua.create_function(move |_, (source, target, trigger, action): (mlua::Value, mlua::Value, mlua::Value, mlua::Value)| {
                harness.borrow_mut().new_edge(into_symbol(source)?, into_symbol(target)?, into_optional_symbol(trigger)?, into_optional_symbol(action)?)
                    .map_err(map_harness_error)?;
                Ok(())
            }).map_err(map_lua_error)?;
            lua.globals().set("E", new_edge_fn).map_err(map_lua_error)?;
        }
        {
            let harness = harness.clone();
            let new_action_fn = lua.create_function(move |lua, mnemonic: String| {
                let symbol = harness.borrow_mut().new_action(&mnemonic)
                    .map_err(map_harness_error)?;
                let symbol_value = TemplateSymbolLuaValue::new(harness.clone(), symbol);
                lua.globals().set(mnemonic, symbol_value.clone())?;
                Ok(symbol_value)
            }).map_err(map_lua_error)?;
            lua.globals().set("A", new_action_fn).map_err(map_lua_error)?;
        }
        {
            let harness = harness.clone();
            let new_process_fn = lua.create_function(move |lua, (mnemonic, entry): (String, mlua::Value) | {
                let symbol = harness.borrow_mut().new_process(&mnemonic, into_symbol(entry)?)
                    .map_err(map_harness_error)?;
                let symbol_value = TemplateSymbolLuaValue::new(harness.clone(), symbol);
                lua.globals().set(mnemonic, symbol_value.clone())?;
                Ok(symbol_value)
            }).map_err(map_lua_error)?;
            lua.globals().set("P", new_process_fn).map_err(map_lua_error)?;
        }
        Ok(())
    }

    fn interpret_template<'a>(&self, fragments: impl Iterator<Item = TemplateFragment>, harness: Rc<RefCell<HarnessBuilder>>, lua: &mut mlua::Lua) -> Result<(), HarnessError> {
        for fragment in fragments {
            match fragment {
                TemplateFragment::Verbatim(content) =>
                    harness.borrow_mut().append_global_prologue(content),

                TemplateFragment::Interpreted(code) =>
                    lua.load(code).exec().map_err(map_lua_error)?
            }
        }

        Ok(())
    }

    pub fn interpret(&mut self, fragments: impl Iterator<Item = TemplateFragment>) -> Result<HarnessBuilder, HarnessError> {
        let harness = Rc::new(RefCell::new(HarnessBuilder::new()));
        let mut lua = mlua::Lua::new();
        self.initialize(harness.clone(), &mut lua)?;
        self.interpret_template(fragments, harness.clone(), &mut lua)?;
        Ok(harness.replace(HarnessBuilder::new()))
    }
}