A('pcspkr_driver_probe_action')
A('pcspkr_driver_send_event_action')
A('pcspkr_driver_suspend_action')
A('pcspkr_driver_shutdown_action')
A('i8253_client_use_action')

S('i8253_client_active_state')
S('pcspkr_driver_unloaded_state')
S('pcspkr_driver_active_state')
S('pcspkr_driver_suspended_state')

i8253_clients = {}
for i = 1, 4 do
    i8253_clients[i] = P('i8253_client' .. i, i8253_client_active_state)
end

pcspkr_drivers = {}
for i = 1, 1 do
    pcspkr_drivers[i] = P('pcspkr_driver' .. i, pcspkr_driver_unloaded_state)
end

E(pcspkr_driver_unloaded_state, pcspkr_driver_unloaded_state, nil, nil)
E(pcspkr_driver_unloaded_state, pcspkr_driver_active_state, nil, pcspkr_driver_probe_action)
E(pcspkr_driver_active_state, pcspkr_driver_active_state, nil, pcspkr_driver_send_event_action)
E(pcspkr_driver_active_state, pcspkr_driver_unloaded_state, nil, pcspkr_driver_shutdown_action)
E(pcspkr_driver_active_state, pcspkr_driver_suspended_state, nil, pcspkr_driver_suspend_action)
E(pcspkr_driver_suspended_state, pcspkr_driver_suspended_state, nil, nil)
E(pcspkr_driver_suspended_state, pcspkr_driver_active_state, nil, nil)
E(pcspkr_driver_suspended_state, pcspkr_driver_unloaded_state, nil, pcspkr_driver_shutdown_action)
E(i8253_client_active_state, i8253_client_active_state, nil, i8253_client_use_action)
