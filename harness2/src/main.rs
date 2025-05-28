use std::collections::BTreeMap;

use harness::{codegen::{codegen::ControlFlowCodegen, goblint::ControlFlowGoblintCodegen, output::WriteCodegenOutput}, control_flow::{builder::ControlFlowBuilder, mutex::ControlFlowMutexSet}, core::{error::HarnessError, mutex::mutex::ProcessSetMutualExclusion, process::ProcessSet, state_machine::StateMachineContext}, dsl2::{lua::LuaTemplateInterpreter, template::TemplateParser}, examples::base::HarnessExample};

pub mod harness;

fn generate(example: &impl HarnessExample) -> Result<(), HarnessError> {
    let mut context = StateMachineContext::new();
    let mut process_set = ProcessSet::new();
    let model = example.build_model(&mut context, &mut process_set)?;

    let state_space = process_set.get_state_space(&context)?;
    let mutual_exclusion = ProcessSetMutualExclusion::new(&context, &process_set, &state_space)?;
    let mutex_set = ControlFlowMutexSet::new(mutual_exclusion.iter());
    let control_flow_nodes = process_set.get_processes()
        .map(| process | {
            let root = process_set.get_process_entry_node(process).ok_or(HarnessError::new("Unable to find process entry node"))?;
            let node = ControlFlowBuilder::new(&context, root)?.build(&context, &process_set, process, &mutex_set)?;
            Ok((process, node.canonicalize()))
        }).collect::<Result<BTreeMap<_, _>, HarnessError>>()?;

    let mut stdout = std::io::stdout();
    let mut stderr = std::io::stderr();

    let mut codegen_output = WriteCodegenOutput::new(&mut stdout);
    let (template, codegen) = example.executable_codegen(&model)?;
    codegen.format(&mut codegen_output, &context, &process_set, &template, control_flow_nodes.iter().map(| (process, node) | (*process, node)), mutex_set.get_mutexes())?;

    let mut codegen_output = WriteCodegenOutput::new(&mut stderr);
    let (template, codegen) = example.goblint_codegen(&model)?;
    codegen.format(&mut codegen_output, &context, &process_set, &template, control_flow_nodes.iter().map(| (process, node) | (*process, node)), mutex_set.get_mutexes())?;
    Ok(())
}

fn main() {
    // match std::env::var("HARNESS_EXAMPLE").as_deref().unwrap_or("ttyprintk") {
    //     "ttyprintk" => generate(&TtyPrintkExample::new( 5)),
    //     "pcspkr" => generate(&PcspkrExample::new(5, 5)),
    //     example => panic!("Unknown HARNESS_EXAMPLE={}", example)

    // }.unwrap()
    let input = r#"
#include "linux/compiler_types.h"
#include "linux/kconfig.h"
#include "asm/orc_header.h"
#include "linux/build-salt.h"
#include "linux/console.h"
#include "linux/device.h"
#include "linux/elfnote-lto.h"
#include "linux/export-internal.h"
#include "linux/module.h"
#include "linux/serial.h"
#include "linux/tty.h"

extern struct tty_driver *registered_tty_driver;

@M('tty_driver_loaded_msg')
@M('tty_client_request_connection_msg')
@M('tty_driver_grant_connection_msg')
@M('tty_client_disconnect_msg')
@M('tty_driver_unloading_msg')

@A('tty_driver_load_action')
@A('tty_driver_loaded_action')
@A('tty_client_request_connection_action')
@A('tty_driver_grant_connection_action')
@A('tty_client_disconnect_action')
@A('tty_client_disconnected_action')
@A('tty_client_acquire_connection_action')
@A('tty_client_use_connection_action')
@A('tty_driver_unload_action')
@A('tty_driver_unloaded_action')

@S('tty_client_nodriver_state')
@S('tty_client_disconnected_state')
@S('tty_client_disconnecting_state')
@S('tty_client_wait_connection_state')
@S('tty_client_connected_state')

@S('tty_driver_unloaded_state')
@S('tty_driver_loading_state')
@S('tty_driver_unloading_state')
@S('tty_driver_client_inactive_substate')
@S('tty_driver_client_active_substate')

@P('tty_driver', tty_driver_unloaded_state)
@{
    tty_clients = {}
    for i = 1, 3 do
        tty_clients[i] = P('tty_client' .. i, tty_client_nodriver_state)
        tty_clients[i].client_id = i;
        tty_clients[i]:setup([[
            struct tty_struct tty;
            struct file file;
            const char content[] = "client%client_id%";
        ]])
    end
}@

@tty_driver_client_inactive_substate:product('tty_driver_loaded_state', tty_clients)

@tty_driver_loaded_action:multicast(tty_clients, tty_driver_loaded_msg)
@tty_client_request_connection_action:unicast(tty_driver, tty_client_request_connection_msg)
@tty_driver_grant_connection_action:respond(tty_driver_grant_connection_msg)
@tty_client_disconnected_action:unicast(tty_driver, tty_client_disconnect_msg)
@tty_driver_unload_action:multicast(tty_clients, tty_driver_unloading_msg)

@E(tty_client_nodriver_state, tty_client_nodriver_state, nil, nil)
@E(tty_client_nodriver_state, tty_client_disconnected_state, tty_driver_loaded_msg, nil)
@E(tty_client_disconnected_state, tty_client_disconnected_state, nil, nil)
@E(tty_client_disconnected_state, tty_client_nodriver_state, tty_driver_unloading_msg, nil)
@E(tty_client_disconnected_state, tty_client_wait_connection_state, nil, tty_client_request_connection_action)
@E(tty_client_wait_connection_state, tty_client_wait_connection_state, nil, nil)
@E(tty_client_wait_connection_state, tty_client_connected_state, tty_driver_grant_connection_msg, tty_client_acquire_connection_action)
@E(tty_client_wait_connection_state, tty_client_nodriver_state, tty_driver_unloading_msg, nil)
@E(tty_client_connected_state, tty_client_connected_state, nil, tty_client_use_connection_action)
@E(tty_client_connected_state, tty_client_disconnecting_state, nil, tty_client_disconnect_action)
@E(tty_client_disconnecting_state, tty_client_disconnecting_state, nil, nil)
@E(tty_client_disconnecting_state, tty_client_disconnected_state, nil, tty_client_disconnected_action)
@E(tty_client_disconnecting_state, tty_client_nodriver_state, tty_driver_unloading_msg, nil)

@E(tty_driver_unloaded_state, tty_driver_unloaded_state, nil, nil)
@E(tty_driver_unloaded_state, tty_driver_loading_state, nil, tty_driver_load_action)
@E(tty_driver_loading_state, tty_driver_loading_state, nil, nil)
@E(tty_driver_loading_state, tty_driver_loaded_state, nil, tty_driver_loaded_action)
@E(tty_driver_loaded_state, tty_driver_unloading_state, nil, tty_driver_unload_action)
@E(tty_driver_unloading_state, tty_driver_unloading_state, nil, nil)
@E(tty_driver_unloading_state, tty_driver_unloaded_state, nil, tty_driver_unloaded_action)
@E(tty_driver_client_inactive_substate, tty_driver_client_active_substate, tty_client_request_connection_msg, tty_driver_grant_connection_action)
@E(tty_driver_client_active_substate, tty_driver_client_inactive_substate, tty_client_disconnect_msg, nil)

@tty_driver_load_action:exec('init_module();')
@tty_driver_unloaded_action:exec('cleanup_module();')
@tty_client_acquire_connection_action:exec('registered_tty_driver->ops->open(&tty, &file);')
@tty_client_disconnect_action:exec('registered_tty_driver->ops->close(&tty, &file);')
@tty_client_use_connection_action:exec('registered_tty_driver->ops->write(&tty, content, sizeof(content));')
"#;

    let mut context = StateMachineContext::new();
    let mut process_set = ProcessSet::new();
    let template = TemplateParser::parse(&mut input.chars().map(| x | Ok(x))).unwrap();
    let mut lua_interp = LuaTemplateInterpreter::new();
    let codegen_template = lua_interp.interpret(&template).unwrap().build(&mut context, &mut process_set).unwrap();

    let state_space = process_set.get_state_space(&context).unwrap();
    let mutual_exclusion = ProcessSetMutualExclusion::new(&context, &process_set, &state_space).unwrap();
    let mutex_set = ControlFlowMutexSet::new(mutual_exclusion.iter());
    let control_flow_nodes = process_set.get_processes()
        .map(| process | {
            let root = process_set.get_process_entry_node(process).ok_or(HarnessError::new("Unable to find process entry node"))?;
            let node = ControlFlowBuilder::new(&context, root)?.build(&context, &process_set, process, &mutex_set)?;
            Ok((process, node.canonicalize()))
        }).collect::<Result<BTreeMap<_, _>, HarnessError>>().unwrap();

    let mut stdout = std::io::stdout();
    let mut codegen_output = WriteCodegenOutput::new(&mut stdout);
    let codegen = ControlFlowGoblintCodegen::new();
    codegen.format(&mut codegen_output, &context, &process_set, &codegen_template, control_flow_nodes.iter().map(| (process, node) | (*process, node)), mutex_set.get_mutexes()).unwrap();

}
