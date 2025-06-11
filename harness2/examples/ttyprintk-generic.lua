function new_ttyprintk_abstract_model(num_of_clients)
    old_model = swap_task_model(new_task_model())

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
        context = swap_task_model(old_model),
        tty_clients = tty_clients
    }
end

function new_ttyprintk_concrete_model(num_of_clients)
    old_model = swap_task_model(new_task_model())

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
    S('tty_driver_clients_inactive_state')
    S('tty_driver_clients_active_state')

    local tty_clients = {}
    for i = 1, num_of_clients do
        tty_clients[i] = P('tty_client' .. i, tty_client_nodriver_state)
        tty_clients[i].client_id = i;
    end
    P('tty_driver', tty_driver_unloaded_state)

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
    E(tty_driver_loading_state, tty_driver_clients_inactive_state, nil, tty_driver_loaded_action)
    E(tty_driver_clients_inactive_state, tty_driver_unloading_state, nil, tty_driver_unload_action)
    E(tty_driver_unloading_state, tty_driver_unloading_state, nil, nil)
    E(tty_driver_unloading_state, tty_driver_unloaded_state, nil, tty_driver_unloaded_action)
    E(tty_driver_clients_inactive_state, tty_driver_clients_active_state, tty_client_request_connection_msg, tty_driver_grant_connection_action)
    E(tty_driver_clients_active_state, tty_driver_clients_inactive_state, tty_client_disconnect_msg, nil)

    return {
        context = swap_task_model(old_model),
        tty_clients = tty_clients
    }
end

abstract_model = new_ttyprintk_abstract_model(2)
concrete_model = new_ttyprintk_concrete_model(6)
add_abstract_model('abstract', abstract_model.context)
swap_task_model(concrete_model.context)
tty_clients = concrete_model.tty_clients

add_mapping("DRV", 'abstract', 'concrete', {
    [abstract_model.context.tty_driver_unloaded_state] = concrete_model.context.tty_driver_unloaded_state,
    [abstract_model.context.tty_driver_loading_state] = concrete_model.context.tty_driver_loading_state,
    [abstract_model.context.tty_driver_unloading_state] = concrete_model.context.tty_driver_unloading_state,
    [abstract_model.context.tty_driver_loaded_state:subnode({abstract_model.context.tty_driver_client_inactive_substate, abstract_model.context.tty_driver_client_inactive_substate})] = concrete_model.context.tty_driver_clients_inactive_state,
    [abstract_model.context.tty_driver_loaded_state:subnode({abstract_model.context.tty_driver_client_active_substate, abstract_model.context.tty_driver_client_inactive_substate})] = concrete_model.context.tty_driver_clients_active_state,
    [abstract_model.context.tty_driver_loaded_state:subnode({abstract_model.context.tty_driver_client_inactive_substate, abstract_model.context.tty_driver_client_active_substate})] = concrete_model.context.tty_driver_clients_active_state,
    [abstract_model.context.tty_driver_loaded_state:subnode({abstract_model.context.tty_driver_client_active_substate, abstract_model.context.tty_driver_client_active_substate})] = concrete_model.context.tty_driver_clients_active_state
})

add_mapping("CLNT", 'abstract', 'concrete', {
    [abstract_model.context.tty_client_nodriver_state] = concrete_model.context.tty_client_nodriver_state,
    [abstract_model.context.tty_client_disconnected_state] = concrete_model.context.tty_client_disconnected_state,
    [abstract_model.context.tty_client_disconnecting_state] = concrete_model.context.tty_client_disconnecting_state,
    [abstract_model.context.tty_client_wait_connection_state] = concrete_model.context.tty_client_wait_connection_state,
    [abstract_model.context.tty_client_connected_state] = concrete_model.context.tty_client_connected_state
})

set_concretization([[
    SELECT DISTINCT
        DRV(a1.tty_driver) AS tty_driver,
        CLNT(a1.tty_client1) AS tty_client1,
        CLNT(a2.tty_client2) AS tty_client2,
        CLNT(a3.tty_client2) AS tty_client3,
        CLNT(a4.tty_client2) AS tty_client4,
        CLNT(a5.tty_client2) AS tty_client5,
        CLNT(a6.tty_client2) AS tty_client6
    FROM abstract AS a1
    INNER JOIN abstract AS a2 ON DRV(a1.tty_driver) = DRV(a2.tty_driver) AND a1.tty_client1 = a2.tty_client1
    INNER JOIN abstract AS a3 ON DRV(a1.tty_driver) = DRV(a3.tty_driver) AND a1.tty_client1 = a3.tty_client1
    INNER JOIN abstract AS a4 ON DRV(a1.tty_driver) = DRV(a4.tty_driver) AND a1.tty_client1 = a4.tty_client1
    INNER JOIN abstract AS a5 ON DRV(a1.tty_driver) = DRV(a5.tty_driver) AND a1.tty_client1 = a5.tty_client1
    INNER JOIN abstract AS a6 ON DRV(a1.tty_driver) = DRV(a6.tty_driver) AND a1.tty_client1 = a6.tty_client1
]])