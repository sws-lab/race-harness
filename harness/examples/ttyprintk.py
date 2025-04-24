from harness.core import ProcessSet
from harness.entities import StateGraphSimpleNode, StateGraphSimpleAction, StateGraphSimpleMessage, StateGraphProductNode, StateGraphDerivedNode, StateGraphResponseMessageDestination, StateGraphProductMessage, StateGraphResponseGroupDestination
from harness.codegen.kernel_module import KernelModuleHarnessGenerator, KernelModuleHarnessProcessTemplate

NUM_OF_CLIENTS = 2

# Messages are quite simple. Driver might communicate to the clients that it has been loaded
# (in reality there is no such communication, but corresponding invariant is simply upheld by the kernel).
# Driver also communicates to the clients when it has been unloaded, and responds to client connection requests.
# Client might request connection and notify driver of disconnection (i.e. dropping kernel handle in reality).
empty_msg = StateGraphSimpleMessage(mnemonic='_')
tty_driver_loaded_msg = StateGraphSimpleMessage(mnemonic='tty_driver_loaded')
tty_client_request_connection_msg = StateGraphSimpleMessage(mnemonic='tty_client_request_connection')
tty_driver_grant_connection_msg = StateGraphSimpleMessage(mnemonic='tty_driver_grant_connection')
tty_client_disconnect_msg = StateGraphSimpleMessage(mnemonic='tty_client_disconnect')
tty_driver_unloaded_msg = StateGraphSimpleMessage(mnemonic='tty_driver_unloaded')

# Client states are simple (here by "connection" I mean acquiring API handle for driver within the kernel, or something similar)
tty_client_nodriver_state = StateGraphSimpleNode(mnemonic='tty_client_nodriver') # No driver loaded => client cannot be connected
tty_client_disconnected_state = StateGraphSimpleNode(mnemonic='tty_client_disconnected') # There is a driver loaded, but client is not connected
tty_client_wait_connection_state = StateGraphSimpleNode(mnemonic='tty_client_wait_connection') # Client has requested connection to the driver (i.e. client attemps to acquire a handle in the kernel)
tty_client_connected_state = StateGraphSimpleNode(mnemonic='tty_client_connected_state') # Client is connected to the driver and can interact with TTY

# Driver states are a bit more complex
tty_driver_unloaded_state = StateGraphSimpleNode(mnemonic='tty_driver_unloaded') # Driver is not loaded
# Now, the only remaining state of the driver is "being loaded", but this state is not scalar,
# because in the driver we need to track each individual client state to avoid unloading if there are active users
# (in reality this is guaranteed by the kernel). Thus, loaded state of the driver contains a substate for each individual
# client.
tty_driver_client_inactive_substate = StateGraphSimpleNode(mnemonic='tty_driver_client_inactive')
tty_driver_client_active_substate = StateGraphSimpleNode(mnemonic='tty_driver_client_active')
tty_driver_all_clients_inactive_substate = StateGraphProductNode((), empty_msg) # Product of all client states
for _ in range(NUM_OF_CLIENTS):
    tty_driver_all_clients_inactive_substate.add_subnode(tty_driver_client_inactive_substate)
tty_driver_loaded_state = StateGraphDerivedNode(mnemonic_prefix='tty_driver_loaded', base=tty_driver_all_clients_inactive_substate) # The actual "loaded" state for the driver

# Processes -- this is simple, we instantiate a number of client processes + one driver process
processes = ProcessSet()
tty_clients = [processes.add_process(mnemonic=f'tty_client{i + 1}', entry_node=tty_client_nodriver_state) for i in range(NUM_OF_CLIENTS)] # Initial state for a client -- no driver
tty_driver = processes.add_process(mnemonic='tty_driver', entry_node=tty_driver_unloaded_state)

# Message maps -- this is only needed to map individual client connection/disconnection request to a single compound "loaded" state of the driver.
tty_driver.add_message_mapping(StateGraphProductMessage.product_message_mapping_from(tty_clients, empty_msg))

# Actions -- actions are used to represent harness C code "payload" attached to state machine transitions + virtual messages to be sent upon transition
noop_action = StateGraphSimpleAction(mnemonic='noop')
tty_driver_load_action = StateGraphSimpleAction(mnemonic='tty_driver_load') \
    .add_envelope(destination=StateGraphResponseGroupDestination(tty_clients), message=tty_driver_loaded_msg)
tty_client_request_connection_action = StateGraphSimpleAction(mnemonic='tty_client_request_connection') \
    .add_envelope(destination=tty_driver, message=tty_client_request_connection_msg)
tty_driver_grant_connection_action = StateGraphSimpleAction(mnemonic='tty_driver_grant_connection') \
    .add_envelope(destination=StateGraphResponseMessageDestination(), message=tty_driver_grant_connection_msg)
tty_client_disconnect_action = StateGraphSimpleAction(mnemonic='tty_client_disconnect') \
    .add_envelope(destination=tty_driver, message=tty_client_disconnect_msg)
tty_client_acquire_connection_action = StateGraphSimpleAction(mnemonic='tty_client_acquire_connection')
tty_client_use_connection_action = StateGraphSimpleAction(mnemonic='tty_client_use_connection')
tty_driver_unload_action = StateGraphSimpleAction(mnemonic='tty_driver_unload') \
    .add_envelope(destination=StateGraphResponseGroupDestination(tty_clients), message=tty_driver_unloaded_msg)

# Now, the actual state machine for the client
tty_client_nodriver_state.add_edge(trigger=None, target=tty_client_nodriver_state, action=noop_action)
tty_client_nodriver_state.add_edge(trigger=tty_driver_loaded_msg, target=tty_client_disconnected_state, action=noop_action)
tty_client_disconnected_state.add_edge(trigger=None, target=tty_client_disconnected_state, action=noop_action)
tty_client_disconnected_state.add_edge(trigger=None, target=tty_client_wait_connection_state, action=tty_client_request_connection_action)
tty_client_disconnected_state.add_edge(trigger=tty_driver_unloaded_msg, target=tty_client_nodriver_state, action=noop_action)
tty_client_wait_connection_state.add_edge(trigger=None, target=tty_client_wait_connection_state, action=noop_action)
tty_client_wait_connection_state.add_edge(trigger=tty_driver_grant_connection_msg, target=tty_client_connected_state, action=tty_client_acquire_connection_action)
tty_client_wait_connection_state.add_edge(trigger=tty_driver_unloaded_msg, target=tty_client_nodriver_state, action=noop_action)
tty_client_connected_state.add_edge(trigger=None, target=tty_client_connected_state, action=tty_client_use_connection_action)
tty_client_connected_state.add_edge(trigger=None, target=tty_client_disconnected_state, action=tty_client_disconnect_action)

# And for the driver
tty_driver_unloaded_state.add_edge(trigger=None, target=tty_driver_unloaded_state, action=noop_action)
tty_driver_unloaded_state.add_edge(trigger=None, target=tty_driver_loaded_state, action=tty_driver_load_action)
tty_driver_client_inactive_substate.add_edge(trigger=tty_client_request_connection_msg, target=tty_driver_client_active_substate, action=tty_driver_grant_connection_action)
tty_driver_client_active_substate.add_edge(trigger=tty_client_disconnect_msg, target=tty_driver_client_inactive_substate, action=noop_action)
# Note that for the compound "loaded" state we only permit unloading when all clients are inactive
tty_driver_loaded_state.add_edge(match_base=tty_driver_all_clients_inactive_substate, trigger=None, target=tty_driver_unloaded_state, action=tty_driver_unload_action)

# Obtain complete state space for the system and derive some invariants
state_space = processes.state_space
tty_client_invariants = [
    *(
        state_space.derive_invariant(process=client, state=tty_client_connected_state, invariant_process=tty_driver)
        for client in tty_clients
    ),
    *(
        state_space.derive_invariant(process=client, state=tty_client_wait_connection_state, invariant_process=tty_driver)
        for client in tty_clients
    ),
    *(
        state_space.derive_invariant(process=client, state=tty_client_disconnected_state, invariant_process=tty_driver)
        for client in tty_clients
    )
]

# Now let's prepare templates for processes
tty_driver_template = KernelModuleHarnessProcessTemplate(tty_driver.entry_node)
tty_driver_template.define_action(tty_driver_load_action, 'init_module();')
tty_driver_template.define_action(tty_driver_unload_action, 'cleanup_module();')

tty_client_template = KernelModuleHarnessProcessTemplate(tty_client_nodriver_state)
tty_client_template.set_initializer('''
struct tty_struct tty;
struct file file;
const char content[] = "%client_buffer%";
''')
tty_client_template.define_action(tty_client_acquire_connection_action, 'registered_tty_driver->ops->open(&tty, &file);')
tty_client_template.define_action(tty_client_disconnect_action, 'registered_tty_driver->ops->close(&tty, &file);')
tty_client_template.define_action(tty_client_use_connection_action, 'registered_tty_driver->ops->write(&tty, content, sizeof(content));')

# And generate the code
codegen = KernelModuleHarnessGenerator(processes)
codegen.set_process_template(tty_driver, tty_driver_template)
for client in tty_clients:
    codegen.set_process_template(client, tty_client_template, {
        'client_buffer': client.mnemonic
    })
for invariant in tty_client_invariants:
    codegen.add_invariant(invariant)
print(codegen.generate())