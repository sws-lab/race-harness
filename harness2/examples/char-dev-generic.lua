function new_char_dev_abstract_model(num_of_clients)
    old_model = swap_task_model(new_task_model())

    M('char_dev_driver_loaded_msg')
    M('char_dev_client_request_connection_msg')
    M('char_dev_driver_grant_connection_msg')
    M('char_dev_client_disconnect_msg')
    M('char_dev_driver_unloading_msg')

    A('char_dev_driver_load_action')
    A('char_dev_driver_loaded_action')
    A('char_dev_client_request_connection_action')
    A('char_dev_driver_grant_connection_action')
    A('char_dev_client_disconnect_action')
    A('char_dev_client_disconnected_action')
    A('char_dev_client_acquire_connection_action')
    A('char_dev_client_use_connection_action')
    A('char_dev_client_use2_connection_action')
    A('char_dev_driver_unload_action')
    A('char_dev_driver_unloaded_action')

    S('char_dev_client_nodriver_state')
    S('char_dev_client_disconnected_state')
    S('char_dev_client_disconnecting_state')
    S('char_dev_client_wait_connection_state')
    S('char_dev_client_connected_state')

    S('char_dev_driver_unloaded_state')
    S('char_dev_driver_loading_state')
    S('char_dev_driver_unloading_state')
    S('char_dev_driver_client_inactive_substate')
    S('char_dev_driver_client_active_substate')

    local char_dev_clients = {}
    for i = 1, num_of_clients do
        char_dev_clients[i] = P('char_dev_client' .. i, char_dev_client_nodriver_state)
        char_dev_clients[i].client_id = i;
    end
    P('char_dev_driver', char_dev_driver_unloaded_state)

    char_dev_driver_client_inactive_substate:product('char_dev_driver_loaded_state', char_dev_clients)

    char_dev_driver_loaded_action:multicast(char_dev_clients, BLOCK_ANY, char_dev_driver_loaded_msg)
    char_dev_client_request_connection_action:unicast(char_dev_driver, BLOCK_ANY, char_dev_client_request_connection_msg)
    char_dev_driver_grant_connection_action:respond(BLOCK_ANY, char_dev_driver_grant_connection_msg)
    char_dev_client_disconnected_action:unicast(char_dev_driver, BLOCK_ANY, char_dev_client_disconnect_msg)
    char_dev_driver_unload_action:multicast(char_dev_clients, BLOCK_ANY, char_dev_driver_unloading_msg)

    E(char_dev_client_nodriver_state, char_dev_client_nodriver_state, nil, nil)
    E(char_dev_client_nodriver_state, char_dev_client_disconnected_state, char_dev_driver_loaded_msg, nil)
    E(char_dev_client_disconnected_state, char_dev_client_disconnected_state, nil, nil)
    E(char_dev_client_disconnected_state, char_dev_client_nodriver_state, char_dev_driver_unloading_msg, nil)
    E(char_dev_client_disconnected_state, char_dev_client_wait_connection_state, nil, char_dev_client_request_connection_action)
    E(char_dev_client_wait_connection_state, char_dev_client_wait_connection_state, nil, nil)
    E(char_dev_client_wait_connection_state, char_dev_client_connected_state, char_dev_driver_grant_connection_msg, char_dev_client_acquire_connection_action)
    E(char_dev_client_wait_connection_state, char_dev_client_nodriver_state, char_dev_driver_unloading_msg, nil)
    E(char_dev_client_connected_state, char_dev_client_connected_state, nil, char_dev_client_use_connection_action)
    E(char_dev_client_connected_state, char_dev_client_connected_state, nil, char_dev_client_use2_connection_action)
    E(char_dev_client_connected_state, char_dev_client_disconnecting_state, nil, char_dev_client_disconnect_action)
    E(char_dev_client_disconnecting_state, char_dev_client_disconnecting_state, nil, nil)
    E(char_dev_client_disconnecting_state, char_dev_client_disconnected_state, nil, char_dev_client_disconnected_action)
    E(char_dev_client_disconnecting_state, char_dev_client_nodriver_state, char_dev_driver_unloading_msg, nil)

    E(char_dev_driver_unloaded_state, char_dev_driver_unloaded_state, nil, nil)
    E(char_dev_driver_unloaded_state, char_dev_driver_loading_state, nil, char_dev_driver_load_action)
    E(char_dev_driver_loading_state, char_dev_driver_loading_state, nil, nil)
    E(char_dev_driver_loading_state, char_dev_driver_loaded_state, nil, char_dev_driver_loaded_action)
    E(char_dev_driver_loaded_state, char_dev_driver_unloading_state, nil, char_dev_driver_unload_action)
    E(char_dev_driver_unloading_state, char_dev_driver_unloading_state, nil, nil)
    E(char_dev_driver_unloading_state, char_dev_driver_unloaded_state, nil, char_dev_driver_unloaded_action)
    E(char_dev_driver_client_inactive_substate, char_dev_driver_client_active_substate, char_dev_client_request_connection_msg, char_dev_driver_grant_connection_action)
    E(char_dev_driver_client_active_substate, char_dev_driver_client_inactive_substate, char_dev_client_disconnect_msg, nil)

    return {
        context = swap_task_model(old_model),
        char_dev_clients = char_dev_clients
    }
end

function new_char_dev_concrete_model(num_of_clients)
    old_model = swap_task_model(new_task_model())

    M('char_dev_driver_loaded_msg')
    M('char_dev_client_request_connection_msg')
    M('char_dev_driver_grant_connection_msg')
    M('char_dev_client_disconnect_msg')
    M('char_dev_driver_unloading_msg')

    A('char_dev_driver_load_action')
    A('char_dev_driver_loaded_action')
    A('char_dev_client_request_connection_action')
    A('char_dev_driver_grant_connection_action')
    A('char_dev_client_disconnect_action')
    A('char_dev_client_disconnected_action')
    A('char_dev_client_acquire_connection_action')
    A('char_dev_client_use_connection_action')
    A('char_dev_client_use2_connection_action')
    A('char_dev_driver_unload_action')
    A('char_dev_driver_unloaded_action')

    S('char_dev_client_nodriver_state')
    S('char_dev_client_disconnected_state')
    S('char_dev_client_disconnecting_state')
    S('char_dev_client_wait_connection_state')
    S('char_dev_client_connected_state')

    S('char_dev_driver_unloaded_state')
    S('char_dev_driver_loading_state')
    S('char_dev_driver_unloading_state')
    S('char_dev_driver_clients_inactive_state')
    S('char_dev_driver_clients_active_state')

    local char_dev_clients = {}
    for i = 1, num_of_clients do
        char_dev_clients[i] = P('char_dev_client' .. i, char_dev_client_nodriver_state)
        char_dev_clients[i].client_id = i;
    end
    P('char_dev_driver', char_dev_driver_unloaded_state)

    char_dev_driver_loaded_action:multicast(char_dev_clients, BLOCK_ANY, char_dev_driver_loaded_msg)
    char_dev_client_request_connection_action:unicast(char_dev_driver, BLOCK_ANY, char_dev_client_request_connection_msg)
    char_dev_driver_grant_connection_action:respond(BLOCK_ANY, char_dev_driver_grant_connection_msg)
    char_dev_client_disconnected_action:unicast(char_dev_driver, BLOCK_ANY, char_dev_client_disconnect_msg)
    char_dev_driver_unload_action:multicast(char_dev_clients, BLOCK_ANY, char_dev_driver_unloading_msg)

    E(char_dev_client_nodriver_state, char_dev_client_nodriver_state, nil, nil)
    E(char_dev_client_nodriver_state, char_dev_client_disconnected_state, char_dev_driver_loaded_msg, nil)
    E(char_dev_client_disconnected_state, char_dev_client_disconnected_state, nil, nil)
    E(char_dev_client_disconnected_state, char_dev_client_nodriver_state, char_dev_driver_unloading_msg, nil)
    E(char_dev_client_disconnected_state, char_dev_client_wait_connection_state, nil, char_dev_client_request_connection_action)
    E(char_dev_client_wait_connection_state, char_dev_client_wait_connection_state, nil, nil)
    E(char_dev_client_wait_connection_state, char_dev_client_connected_state, char_dev_driver_grant_connection_msg, char_dev_client_acquire_connection_action)
    E(char_dev_client_wait_connection_state, char_dev_client_nodriver_state, char_dev_driver_unloading_msg, nil)
    E(char_dev_client_connected_state, char_dev_client_connected_state, nil, char_dev_client_use_connection_action)
    E(char_dev_client_connected_state, char_dev_client_connected_state, nil, char_dev_client_use2_connection_action)
    E(char_dev_client_connected_state, char_dev_client_disconnecting_state, nil, char_dev_client_disconnect_action)
    E(char_dev_client_disconnecting_state, char_dev_client_disconnecting_state, nil, nil)
    E(char_dev_client_disconnecting_state, char_dev_client_disconnected_state, nil, char_dev_client_disconnected_action)
    E(char_dev_client_disconnecting_state, char_dev_client_nodriver_state, char_dev_driver_unloading_msg, nil)

    E(char_dev_driver_unloaded_state, char_dev_driver_unloaded_state, nil, nil)
    E(char_dev_driver_unloaded_state, char_dev_driver_loading_state, nil, char_dev_driver_load_action)
    E(char_dev_driver_loading_state, char_dev_driver_loading_state, nil, nil)
    E(char_dev_driver_loading_state, char_dev_driver_clients_inactive_state, nil, char_dev_driver_loaded_action)
    E(char_dev_driver_clients_inactive_state, char_dev_driver_unloading_state, nil, char_dev_driver_unload_action)
    E(char_dev_driver_unloading_state, char_dev_driver_unloading_state, nil, nil)
    E(char_dev_driver_unloading_state, char_dev_driver_unloaded_state, nil, char_dev_driver_unloaded_action)
    E(char_dev_driver_clients_inactive_state, char_dev_driver_clients_active_state, char_dev_client_request_connection_msg, char_dev_driver_grant_connection_action)
    E(char_dev_driver_clients_active_state, char_dev_driver_clients_inactive_state, char_dev_client_disconnect_msg, nil)

    return {
        context = swap_task_model(old_model),
        char_dev_clients = char_dev_clients
    }
end

abstract_model = new_char_dev_abstract_model(2)
concrete_model = new_char_dev_concrete_model(4)
add_abstract_model('abstract', abstract_model.context)
swap_task_model(concrete_model.context)
char_dev_clients = concrete_model.char_dev_clients

add_mapping("DRV", 'abstract', 'concrete', {
    [abstract_model.context.char_dev_driver_unloaded_state] = concrete_model.context.char_dev_driver_unloaded_state,
    [abstract_model.context.char_dev_driver_loading_state] = concrete_model.context.char_dev_driver_loading_state,
    [abstract_model.context.char_dev_driver_unloading_state] = concrete_model.context.char_dev_driver_unloading_state,
    [abstract_model.context.char_dev_driver_loaded_state:subnode({abstract_model.context.char_dev_driver_client_inactive_substate, abstract_model.context.char_dev_driver_client_inactive_substate})] = concrete_model.context.char_dev_driver_clients_inactive_state,
    [abstract_model.context.char_dev_driver_loaded_state:subnode({abstract_model.context.char_dev_driver_client_active_substate, abstract_model.context.char_dev_driver_client_inactive_substate})] = concrete_model.context.char_dev_driver_clients_active_state,
    [abstract_model.context.char_dev_driver_loaded_state:subnode({abstract_model.context.char_dev_driver_client_inactive_substate, abstract_model.context.char_dev_driver_client_active_substate})] = concrete_model.context.char_dev_driver_clients_active_state,
    [abstract_model.context.char_dev_driver_loaded_state:subnode({abstract_model.context.char_dev_driver_client_active_substate, abstract_model.context.char_dev_driver_client_active_substate})] = concrete_model.context.char_dev_driver_clients_active_state
})

add_mapping("CLNT", 'abstract', 'concrete', {
    [abstract_model.context.char_dev_client_nodriver_state] = concrete_model.context.char_dev_client_nodriver_state,
    [abstract_model.context.char_dev_client_disconnected_state] = concrete_model.context.char_dev_client_disconnected_state,
    [abstract_model.context.char_dev_client_disconnecting_state] = concrete_model.context.char_dev_client_disconnecting_state,
    [abstract_model.context.char_dev_client_wait_connection_state] = concrete_model.context.char_dev_client_wait_connection_state,
    [abstract_model.context.char_dev_client_connected_state] = concrete_model.context.char_dev_client_connected_state
})

add_query([[
    CREATE TABLE mapped_abstract AS
        SELECT DISTINCT
            DRV(char_dev_driver) AS char_dev_driver,
            CLNT(char_dev_client1) AS char_dev_client1,
            CLNT(char_dev_client2) AS char_dev_client2
        FROM abstract;

    CREATE INDEX mapped_abstract_char_dev_driver ON mapped_abstract(char_dev_driver);
    CREATE INDEX mapped_abstract_char_dev_client1 ON mapped_abstract(char_dev_client1);
    CREATE INDEX mapped_abstract_char_dev_client2 ON mapped_abstract(char_dev_client2);

    CREATE VIEW concrete AS
        SELECT
            a1.char_dev_driver AS char_dev_driver,
            a1.char_dev_client1 AS char_dev_client1,
            a2.char_dev_client2 AS char_dev_client2,
            a3.char_dev_client2 AS char_dev_client3,
            a4.char_dev_client2 AS char_dev_client4
        FROM mapped_abstract AS a1
        INNER JOIN mapped_abstract AS a2 ON a1.char_dev_driver = a2.char_dev_driver AND a1.char_dev_client1 = a2.char_dev_client1
        INNER JOIN mapped_abstract AS a3 ON a1.char_dev_driver = a3.char_dev_driver AND a1.char_dev_client1 = a3.char_dev_client1
        INNER JOIN mapped_abstract AS a4 ON a1.char_dev_driver = a4.char_dev_driver AND a1.char_dev_client1 = a4.char_dev_client1
]])

set_concretization('concrete')