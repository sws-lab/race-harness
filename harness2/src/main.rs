use std::collections::HashMap;

use harness::{codegen::{base::{CodegenOutput, CodegenTemplate, ControlFlowCodegen}, executable::ControlFlowExecutableCodegen, goblint::ControlFlowGoblintCodegen, output::WriteCodegenOutput}, control_flow::{builder::ControlFlowBuilder, mutex::ControlFlowMutexSet, node::ControlFlowNode}, core::{error::HarnessError, mutex::mutex::ProcessSetMutualExclusion, process::{ProcessID, ProcessSet}, state_machine::{StateMachineActionID, StateMachineContext, StateMachineMessageDestination, StateMachineMessageID, StateMachineMessageParticipantID, StateMachineNodeID}}, entities::product_node::StateMachineProductNodeBuilder};

pub mod harness;

#[allow(dead_code)]
struct TtyPrintkModel {
    tty_driver_loaded_msg: StateMachineMessageID,
    tty_client_request_connection_msg: StateMachineMessageID,
    tty_driver_grant_connection_msg: StateMachineMessageID,
    tty_client_disconnect_msg: StateMachineMessageID,
    tty_driver_unloading_msg: StateMachineMessageID,

    tty_driver_load_action: StateMachineActionID,
    tty_driver_loaded_action: StateMachineActionID,
    tty_client_request_connection_action: StateMachineActionID,
    tty_driver_grant_connection_action: StateMachineActionID,
    tty_client_disconnect_action: StateMachineActionID,
    tty_client_disconnected_action: StateMachineActionID,
    tty_client_acquire_connection_action: StateMachineActionID,
    tty_client_use_connection_action: StateMachineActionID,
    tty_driver_unload_action: StateMachineActionID,
    tty_driver_unloaded_action: StateMachineActionID,

    tty_client_nodriver_state: StateMachineNodeID,
    tty_client_disconnected_state: StateMachineNodeID,
    tty_client_disconnecting_state: StateMachineNodeID,
    tty_client_wait_connection_state: StateMachineNodeID,
    tty_client_connected_state: StateMachineNodeID,

    tty_driver_unloaded_state: StateMachineNodeID,
    tty_driver_loading_state: StateMachineNodeID,
    tty_driver_unloading_state: StateMachineNodeID,
    tty_driver_client_inactive_substate: StateMachineNodeID,
    tty_driver_client_active_substate: StateMachineNodeID,

    tty_driver: ProcessID,
    tty_clients: Vec<ProcessID>
}

impl TtyPrintkModel {
    pub fn new(context: &mut StateMachineContext, process_set: &mut ProcessSet, num_of_clients: usize) -> Result<TtyPrintkModel, HarnessError> {
        let tty_driver_loaded_msg = context.new_message("tty_driver_loaded")?;
        let tty_client_request_connection_msg = context.new_message("tty_client_request_connection")?;
        let tty_driver_grant_connection_msg = context.new_message("tty_client_grant_connection")?;
        let tty_client_disconnect_msg = context.new_message("tty_client_disconnect")?;
        let tty_driver_unloading_msg = context.new_message("tty_driver_unloading")?;

        let tty_driver_load_action = context.new_action("tty_driver_load")?;
        let tty_driver_loaded_action = context.new_action("tty_driver_loaded")?;
        let tty_client_request_connection_action = context.new_action("tty_client_request_connection")?;
        let tty_driver_grant_connection_action = context.new_action("tty_driver_grant_connection")?;
        let tty_client_disconnect_action = context.new_action("tty_client_disconnect")?;
        let tty_client_disconnected_action = context.new_action("tty_client_disconnected")?;
        let tty_client_acquire_connection_action = context.new_action("tty_client_acquire_connection")?;
        let tty_client_use_connection_action = context.new_action("tty_client_use_connection")?;
        let tty_driver_unload_action = context.new_action("tty_driver_unload")?;
        let tty_driver_unloaded_action = context.new_action("tty_driver_unloaded")?;

        let tty_client_nodriver_state = context.new_node("tty_client_nodriver")?;
        let tty_client_disconnected_state = context.new_node("tty_client_disconnected")?;
        let tty_client_disconnecting_state = context.new_node("tty_client_disconnecting")?;
        let tty_client_wait_connection_state = context.new_node("tty_client_wait_connection")?;
        let tty_client_connected_state = context.new_node("tty_client_connected_state")?;

        let tty_driver_unloaded_state = context.new_node("tty_driver_unloaded")?;
        let tty_driver_loading_state = context.new_node("tty_driver_loading")?;
        let tty_driver_unloading_state = context.new_node("tty_driver_unloading")?;
        let tty_driver_client_inactive_substate = context.new_node("tty_driver_client_inactive")?;
        let tty_driver_client_active_substate = context.new_node("tty_driver_client_active")?;
        
        let tty_driver = process_set.new_process("tty_driver".into(), tty_driver_unloaded_state);
        let tty_clients = (0..num_of_clients).map(| client_id |
            process_set.new_process(format!("tty_client{}", client_id), tty_client_nodriver_state))
            .collect::<Vec<_>>();
        let tty_client_participants = tty_clients.iter()
            .map(| &process | process.into())
            .collect::<Vec<StateMachineMessageParticipantID>>();

        context.new_edge(tty_driver_client_inactive_substate, tty_driver_client_active_substate, Some(tty_client_request_connection_msg), Some(tty_driver_grant_connection_action))?;
        context.new_edge(tty_driver_client_active_substate, tty_driver_client_inactive_substate, Some(tty_client_disconnect_msg), None)?;
        let tty_driver_loaded_state = StateMachineProductNodeBuilder::new(tty_driver_client_inactive_substate, tty_clients.len()).build(context)?;

        process_set.new_inbound_message_mapping(tty_driver, tty_driver_loaded_state.get_inbound_message_mapping(tty_client_participants.clone().into_iter())?)?;
        process_set.new_outbound_message_mapping(tty_driver, tty_driver_loaded_state.get_outbound_message_mapping(tty_client_participants.clone().into_iter())?)?;

        context.add_envelope(tty_driver_loaded_action,
            StateMachineMessageDestination::Multicast(tty_client_participants.clone().into_iter().collect()),
            tty_driver_loaded_msg)?;
        context.add_envelope(tty_client_request_connection_action,
            StateMachineMessageDestination::Unicast( tty_driver.into()),
            tty_client_request_connection_msg)?;
        context.add_envelope(tty_driver_grant_connection_action,
            StateMachineMessageDestination::Response,
            tty_driver_grant_connection_msg)?;
        context.add_envelope(tty_client_disconnected_action,
            StateMachineMessageDestination::Unicast(tty_driver.into()),
            tty_client_disconnect_msg)?;
        context.add_envelope(tty_driver_unload_action,
            StateMachineMessageDestination::Multicast(tty_client_participants.into_iter().collect()),
            tty_driver_unloading_msg)?;

        context.new_edge(tty_client_nodriver_state, tty_client_nodriver_state, None, None)?;
        context.new_edge(tty_client_nodriver_state, tty_client_disconnected_state, Some(tty_driver_loaded_msg), None)?;
        context.new_edge(tty_client_disconnected_state, tty_client_disconnected_state, None, None)?;
        context.new_edge(tty_client_disconnected_state, tty_client_nodriver_state, Some(tty_driver_unloading_msg), None)?;
        context.new_edge(tty_client_disconnected_state, tty_client_wait_connection_state, None, Some(tty_client_request_connection_action))?;
        context.new_edge(tty_client_wait_connection_state, tty_client_wait_connection_state, None, None)?;
        context.new_edge(tty_client_wait_connection_state, tty_client_connected_state, Some(tty_driver_grant_connection_msg), Some(tty_client_acquire_connection_action))?;
        context.new_edge(tty_client_wait_connection_state, tty_client_nodriver_state, Some(tty_driver_unloading_msg), None)?;
        context.new_edge(tty_client_connected_state, tty_client_connected_state, None, Some(tty_client_use_connection_action))?;
        context.new_edge(tty_client_connected_state, tty_client_disconnecting_state, None, Some(tty_client_disconnect_action))?;
        context.new_edge(tty_client_disconnecting_state, tty_client_disconnecting_state, None, None)?;
        context.new_edge(tty_client_disconnecting_state, tty_client_disconnected_state, None, Some(tty_client_disconnected_action))?;
        context.new_edge(tty_client_disconnecting_state, tty_client_nodriver_state, Some(tty_driver_unloading_msg), None)?;

        context.new_edge(tty_driver_unloaded_state, tty_driver_unloaded_state, None, None)?;
        context.new_edge(tty_driver_unloaded_state, tty_driver_loading_state, None, Some(tty_driver_load_action))?;
        context.new_edge(tty_driver_loading_state, tty_driver_loading_state, None, None)?;
        context.new_edge(tty_driver_loading_state, tty_driver_loaded_state.get_root_node(), None, Some(tty_driver_loaded_action))?;
        context.new_edge(tty_driver_loaded_state.get_root_node(), tty_driver_unloading_state, None, Some(tty_driver_unload_action))?;
        context.new_edge(tty_driver_unloading_state, tty_driver_unloading_state, None, None)?;
        context.new_edge(tty_driver_unloading_state, tty_driver_unloaded_state, None, Some(tty_driver_unloaded_action))?;

        Ok(TtyPrintkModel {
            tty_driver_loaded_msg,
            tty_client_request_connection_msg,
            tty_driver_grant_connection_msg,
            tty_client_disconnect_msg,
            tty_driver_unloading_msg,
        
            tty_driver_load_action,
            tty_driver_loaded_action,
            tty_client_request_connection_action,
            tty_driver_grant_connection_action,
            tty_client_disconnect_action,
            tty_client_disconnected_action,
            tty_client_acquire_connection_action,
            tty_client_use_connection_action,
            tty_driver_unload_action,
            tty_driver_unloaded_action,
        
            tty_client_nodriver_state,
            tty_client_disconnected_state,
            tty_client_disconnecting_state,
            tty_client_wait_connection_state,
            tty_client_connected_state,
        
            tty_driver_unloaded_state,
            tty_driver_loading_state,
            tty_driver_unloading_state,
            tty_driver_client_inactive_substate,
            tty_driver_client_active_substate,
        
            tty_driver,
            tty_clients: Vec::from(tty_clients)
        })
    }
}

fn generate_goblint<'a>(codegen_output: &mut impl CodegenOutput, model: &TtyPrintkModel, context: &StateMachineContext, process_set: &ProcessSet, control_flow_nodes: impl Iterator<Item = (ProcessID, &'a ControlFlowNode)>, mutex_set: &'a ControlFlowMutexSet) -> Result<(), HarnessError> {
    let mut template = CodegenTemplate::new().set_global_prologue(Some(r#"
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
"#))
    .define_action(model.tty_driver_load_action, r#"
init_module();
"#).define_action(model.tty_driver_unloaded_action, r#"
cleanup_module();
"#).define_action(model.tty_client_acquire_connection_action, r#"
registered_tty_driver->ops->open(&tty, &file);
"#).define_action(model.tty_client_disconnect_action, r#"
registered_tty_driver->ops->close(&tty, &file);
"#).define_action(model.tty_client_use_connection_action, r#"
registered_tty_driver->ops->write(&tty, content, sizeof(content));
"#);

    for (index, client) in model.tty_clients.iter().enumerate() {
        template = template.set_process_parameter(*client, "client_id", format!("{}", index))
            .set_process_prologue(*client, r#"
struct tty_struct tty;
struct file file;
const char content[] = "client%client_id%";
"#);
    }

    let codegen = ControlFlowGoblintCodegen::new();
    codegen.format(codegen_output, &context, &process_set, &template, control_flow_nodes, mutex_set.get_mutexes())
}


fn generate_executable<'a>(codegen_output: &mut impl CodegenOutput, model: &TtyPrintkModel, context: &StateMachineContext, process_set: &ProcessSet, control_flow_nodes: impl Iterator<Item = (ProcessID, &'a ControlFlowNode)>, mutex_set: &'a ControlFlowMutexSet) -> Result<(), HarnessError> {
    let mut template = CodegenTemplate::new().set_global_prologue(Some(r#"
#include <stdlib.h>
#include <stdio.h>
#include <pthread.h>

struct S1 {
    _Atomic unsigned int connections;
    _Atomic unsigned int value;
};
                            
static struct S1 *s1_ptr;
"#)).set_process_prologue(model.tty_driver, "static struct S1 s1 = {0};")
    .define_action(model.tty_driver_load_action, r#"
s1_ptr = &s1;
s1_ptr->connections = 0;
s1_ptr->value = 0;
printf("Driver loaded\n");
"#).define_action(model.tty_driver_unloaded_action, r#"
printf("Driver unloaded\n");
s1_ptr = NULL;
"#).define_action(model.tty_client_acquire_connection_action, r#"
s1_ptr->connections++;
printf("Client %client_id% connected\n");
"#).define_action(model.tty_client_disconnect_action, r#"
s1_ptr->connections--;
printf("Client %client_id% disconnected\n");
"#).define_action(model.tty_client_use_connection_action, r#"
s1_ptr->value++;
printf("Client %client_id% active\n");
"#);

    for (index, client) in model.tty_clients.iter().enumerate() {
        template = template.set_process_parameter(*client, "client_id", format!("{}", index));
    }

    let codegen = ControlFlowExecutableCodegen::new();
    codegen.format(codegen_output, &context, &process_set, &template, control_flow_nodes, mutex_set.get_mutexes())
}

fn generate(executable_harness: bool) -> Result<(), HarnessError> {
    let mut context = StateMachineContext::new();
    let mut process_set = ProcessSet::new();
    let model = TtyPrintkModel::new(&mut context, &mut process_set, 3)?;

    let state_space = process_set.get_state_space(&context)?;
    let mutual_exclusion = ProcessSetMutualExclusion::new(&context, &process_set, &state_space)?;
    let mutex_set = ControlFlowMutexSet::new(mutual_exclusion.iter());
    let control_flow_nodes = process_set.get_processes()
        .map(| process | {
            let root = process_set.get_process_entry_node(process).ok_or(HarnessError::new("Unable to find process entry node"))?;
            let node = ControlFlowBuilder::new(&context, root)?.build(&context, &process_set, process, &mutex_set)?;
            Ok((process, node.canonicalize()))
        }).collect::<Result<HashMap<_, _>, HarnessError>>()?;

    let mut stdout = std::io::stdout();
    let mut codegen_output = WriteCodegenOutput::new(&mut stdout);

    if executable_harness {
        generate_executable(&mut codegen_output, &model, &context, &process_set, control_flow_nodes.iter().map(| (process, node) | (*process, node)), &mutex_set)?;
    } else {
        generate_goblint(&mut codegen_output, &model, &context, &process_set, control_flow_nodes.iter().map(| (process, node) | (*process, node)), &mutex_set)?;
    }
    Ok(())
}

fn main() {
    generate(std::env::var("EXECUTABLE_HARNESS").is_ok()).unwrap();
}
