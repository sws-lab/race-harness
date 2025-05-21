use std::collections::HashMap;

use harness::{control_flow::{builder::ControlFlowBuilder, mutex::ControlFlowMutexSet}, core::{format::ProcessSetStateSpaceFormatter, mutex::{format::MutualExclusionSegmentFormatter, mutex::ProcessSetMutualExclusion}, process::ProcessSet, state_machine::{StateMachineContext, StateMachineMessageDestination, StateMachineMessageParticipantID}}, entities::product_node::StateMachineProductNodeBuilder};

pub mod harness;

fn main() {
    let mut context = StateMachineContext::new();
    const NUM_OF_CLIENTS: usize = 2;

    let tty_driver_loaded_msg = context.new_message("tty_driver_loaded").unwrap();
    let tty_client_request_connection_msg = context.new_message("tty_client_request_connection").unwrap();
    let tty_driver_grant_connection_msg = context.new_message("tty_client_grant_connection").unwrap();
    let tty_client_disconnect_msg = context.new_message("tty_client_disconnect").unwrap();
    let tty_driver_unloading_msg = context.new_message("tty_driver_unloading").unwrap();

    let tty_driver_load_action = context.new_action("tty_driver_load").unwrap();
    let tty_driver_loaded_action = context.new_action("tty_driver_loaded").unwrap();
    let tty_client_request_connection_action = context.new_action("tty_client_request_connection").unwrap();
    let tty_driver_grant_connection_action = context.new_action("tty_driver_grant_connection").unwrap();
    let tty_client_disconnect_action = context.new_action("tty_client_disconnect").unwrap();
    let tty_client_disconnected_action = context.new_action("tty_client_disconnected").unwrap();
    let tty_client_acquire_connection_action = context.new_action("tty_client_acquire_connection").unwrap();
    let tty_client_use_connection_action = context.new_action("tty_client_use_connection").unwrap();
    let tty_driver_unload_action = context.new_action("tty_driver_unload").unwrap();
    let tty_driver_unloaded_action = context.new_action("tty_driver_unloaded").unwrap();

    let tty_client_nodriver_state = context.new_node("tty_client_nodriver").unwrap();
    let tty_client_disconnected_state = context.new_node("tty_client_disconnected").unwrap();
    let tty_client_disconnecting_state = context.new_node("tty_client_disconnecting").unwrap();
    let tty_client_wait_connection_state = context.new_node("tty_client_wait_connection").unwrap();
    let tty_client_connected_state = context.new_node("tty_client_connected_state").unwrap();

    let tty_driver_unloaded_state = context.new_node("tty_driver_unloaded").unwrap();
    let tty_driver_loading_state = context.new_node("tty_driver_loading").unwrap();
    let tty_driver_unloading_state = context.new_node("tty_driver_unloading").unwrap();
    let tty_driver_client_inactive_substate = context.new_node("tty_driver_client_inactive").unwrap();
    let tty_driver_client_active_substate = context.new_node("tty_driver_client_active").unwrap();
    
    let mut process_set = ProcessSet::new();
    let tty_driver = process_set.new_process("tty_driver".into(), tty_driver_unloaded_state);
    let tty_clients = (0..NUM_OF_CLIENTS).map(| client_id |
        process_set.new_process(format!("tty_client{}", client_id), tty_client_nodriver_state))
        .collect::<Vec<_>>();
    let tty_client_participants = tty_clients.iter()
        .map(| &process | process.into())
        .collect::<Vec<StateMachineMessageParticipantID>>();

    context.new_edge(tty_driver_client_inactive_substate, tty_driver_client_active_substate, Some(tty_client_request_connection_msg), Some(tty_driver_grant_connection_action)).unwrap();
    context.new_edge(tty_driver_client_active_substate, tty_driver_client_inactive_substate, Some(tty_client_disconnect_msg), None).unwrap();
    let tty_driver_loaded_state = StateMachineProductNodeBuilder::new(tty_driver_client_inactive_substate, tty_clients.len()).build(&mut context).unwrap();

    process_set.new_inbound_message_mapping(tty_driver, tty_driver_loaded_state.get_inbound_message_mapping(tty_client_participants.clone().into_iter()).unwrap()).unwrap();
    process_set.new_outbound_message_mapping(tty_driver, tty_driver_loaded_state.get_outbound_message_mapping(tty_client_participants.clone().into_iter()).unwrap()).unwrap();

    context.add_envelope(tty_driver_loaded_action,
         StateMachineMessageDestination::Multicast(tty_client_participants.clone().into_iter().collect()),
        tty_driver_loaded_msg).unwrap();
    context.add_envelope(tty_client_request_connection_action,
        StateMachineMessageDestination::Unicast( tty_driver.into()),
        tty_client_request_connection_msg).unwrap();
    context.add_envelope(tty_driver_grant_connection_action,
        StateMachineMessageDestination::Response,
        tty_driver_grant_connection_msg).unwrap();
    context.add_envelope(tty_client_disconnected_action,
        StateMachineMessageDestination::Unicast(tty_driver.into()),
        tty_client_disconnect_msg).unwrap();
    context.add_envelope(tty_driver_unload_action,
        StateMachineMessageDestination::Multicast(tty_client_participants.into_iter().collect()),
        tty_driver_unloading_msg).unwrap();

    context.new_edge(tty_client_nodriver_state, tty_client_nodriver_state, None, None).unwrap();
    context.new_edge(tty_client_nodriver_state, tty_client_disconnected_state, Some(tty_driver_loaded_msg), None).unwrap();
    context.new_edge(tty_client_disconnected_state, tty_client_disconnected_state, None, None).unwrap();
    context.new_edge(tty_client_disconnected_state, tty_client_nodriver_state, Some(tty_driver_unloading_msg), None).unwrap();
    context.new_edge(tty_client_disconnected_state, tty_client_wait_connection_state, None, Some(tty_client_request_connection_action)).unwrap();
    context.new_edge(tty_client_wait_connection_state, tty_client_wait_connection_state, None, None).unwrap();
    context.new_edge(tty_client_wait_connection_state, tty_client_connected_state, Some(tty_driver_grant_connection_msg), Some(tty_client_acquire_connection_action)).unwrap();
    context.new_edge(tty_client_wait_connection_state, tty_client_nodriver_state, Some(tty_driver_unloading_msg), None).unwrap();
    context.new_edge(tty_client_connected_state, tty_client_connected_state, None, Some(tty_client_use_connection_action)).unwrap();
    context.new_edge(tty_client_connected_state, tty_client_disconnecting_state, None, Some(tty_client_disconnect_action)).unwrap();
    context.new_edge(tty_client_disconnecting_state, tty_client_disconnecting_state, None, None).unwrap();
    context.new_edge(tty_client_disconnecting_state, tty_client_disconnected_state, None, Some(tty_client_disconnected_action)).unwrap();
    context.new_edge(tty_client_disconnecting_state, tty_client_nodriver_state, Some(tty_driver_unloading_msg), None).unwrap();

    context.new_edge(tty_driver_unloaded_state, tty_driver_unloaded_state, None, None).unwrap();
    context.new_edge(tty_driver_unloaded_state, tty_driver_loading_state, None, Some(tty_driver_load_action)).unwrap();
    context.new_edge(tty_driver_loading_state, tty_driver_loading_state, None, None).unwrap();
    context.new_edge(tty_driver_loading_state, tty_driver_loaded_state.get_root_node(), None, Some(tty_driver_loaded_action)).unwrap();
    context.new_edge(tty_driver_loaded_state.get_root_node(), tty_driver_unloading_state, None, Some(tty_driver_unload_action)).unwrap();
    context.new_edge(tty_driver_unloading_state, tty_driver_unloading_state, None, None).unwrap();
    context.new_edge(tty_driver_unloading_state, tty_driver_unloaded_state, None, Some(tty_driver_unloaded_action)).unwrap();
    
    let state_space = process_set.get_state_space(&context).unwrap();
    println!("{}", state_space.len());
    println!("{}", ProcessSetStateSpaceFormatter::new(&context, &process_set, &state_space).unwrap());

    println!("--------------------------------------");
    let mutual_exclusion = ProcessSetMutualExclusion::new(&context, &process_set, &state_space).unwrap();
    for segment in mutual_exclusion.iter() {
        println!("{}", MutualExclusionSegmentFormatter::new(&context, &process_set, segment).unwrap());
    }

    let mutex_set = ControlFlowMutexSet::new(mutual_exclusion.iter());
    let control_flow_nodes = process_set.get_processes()
        .map(| process | {
            let root = process_set.get_process_entry_node(process).unwrap();
            let node = ControlFlowBuilder::new(&context, root).unwrap().build(&context, &process_set, process, &mutex_set).unwrap();
            println!("{:?}", node);
            (process, node)
        }).collect::<HashMap<_, _>>();
}
