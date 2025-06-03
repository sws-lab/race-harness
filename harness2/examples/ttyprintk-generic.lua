function new_ttyprintk_model(num_of_clients)
    old_ctx = swap_task_context(__task_context:clone())

    M('tty_driver_loaded_msg')
    M('tty_client_request_connection_msg')
    M('tty_driver_grant_connection_msg')
    M('tty_client_disconnect_msg')
    M('tty_driver_unloading_msg')

    A('tty_driver_load_action')
    A('tty_driver_loaded_action')
    A('tty_client_request_connection_action')
    A('tty_driver_grant_connection_action')
    A('tty_client_disconnect_action')
    A('tty_client_disconnected_action')
    A('tty_client_acquire_connection_action')
    A('tty_client_use_connection_action')
    A('tty_driver_unload_action')
    A('tty_driver_unloaded_action')

    S('tty_client_nodriver_state')
    S('tty_client_disconnected_state')
    S('tty_client_disconnecting_state')
    S('tty_client_wait_connection_state')
    S('tty_client_connected_state')

    S('tty_driver_unloaded_state')
    S('tty_driver_loading_state')
    S('tty_driver_unloading_state')
    S('tty_driver_client_inactive_substate')
    S('tty_driver_client_active_substate')

    local tty_clients = {}
    for i = 1, num_of_clients do
        tty_clients[i] = P('tty_client' .. i, tty_client_nodriver_state)
        tty_clients[i].client_id = i;
    end
    P('tty_driver', tty_driver_unloaded_state)

    tty_driver_client_inactive_substate:product('tty_driver_loaded_state', tty_clients)

    tty_driver_loaded_action:multicast(tty_clients, BLOCK_ANY, tty_driver_loaded_msg)
    tty_client_request_connection_action:unicast(tty_driver, BLOCK_ANY, tty_client_request_connection_msg)
    tty_driver_grant_connection_action:respond(BLOCK_ANY, tty_driver_grant_connection_msg)
    tty_client_disconnected_action:unicast(tty_driver, BLOCK_ANY, tty_client_disconnect_msg)
    tty_driver_unload_action:multicast(tty_clients, BLOCK_ANY, tty_driver_unloading_msg)

    E(tty_client_nodriver_state, tty_client_nodriver_state, nil, nil)
    E(tty_client_nodriver_state, tty_client_disconnected_state, tty_driver_loaded_msg, nil)
    E(tty_client_disconnected_state, tty_client_disconnected_state, nil, nil)
    E(tty_client_disconnected_state, tty_client_nodriver_state, tty_driver_unloading_msg, nil)
    E(tty_client_disconnected_state, tty_client_wait_connection_state, nil, tty_client_request_connection_action)
    E(tty_client_wait_connection_state, tty_client_wait_connection_state, nil, nil)
    E(tty_client_wait_connection_state, tty_client_connected_state, tty_driver_grant_connection_msg, tty_client_acquire_connection_action)
    E(tty_client_wait_connection_state, tty_client_nodriver_state, tty_driver_unloading_msg, nil)
    E(tty_client_connected_state, tty_client_connected_state, nil, tty_client_use_connection_action)
    E(tty_client_connected_state, tty_client_disconnecting_state, nil, tty_client_disconnect_action)
    E(tty_client_disconnecting_state, tty_client_disconnecting_state, nil, nil)
    E(tty_client_disconnecting_state, tty_client_disconnected_state, nil, tty_client_disconnected_action)
    E(tty_client_disconnecting_state, tty_client_nodriver_state, tty_driver_unloading_msg, nil)

    E(tty_driver_unloaded_state, tty_driver_unloaded_state, nil, nil)
    E(tty_driver_unloaded_state, tty_driver_loading_state, nil, tty_driver_load_action)
    E(tty_driver_loading_state, tty_driver_loading_state, nil, nil)
    E(tty_driver_loading_state, tty_driver_loaded_state, nil, tty_driver_loaded_action)
    E(tty_driver_loaded_state, tty_driver_unloading_state, nil, tty_driver_unload_action)
    E(tty_driver_unloading_state, tty_driver_unloading_state, nil, nil)
    E(tty_driver_unloading_state, tty_driver_unloaded_state, nil, tty_driver_unloaded_action)
    E(tty_driver_client_inactive_substate, tty_driver_client_active_substate, tty_client_request_connection_msg, tty_driver_grant_connection_action)
    E(tty_driver_client_active_substate, tty_driver_client_inactive_substate, tty_client_disconnect_msg, nil)

    return {
        context = swap_task_context(old_ctx),
        tty_clients = tty_clients
    }
end

model1 = new_ttyprintk_model(4)
-- model2 = new_ttyprintk_model(1)
swap_task_context(model1.context)
tty_clients = model1.tty_clients