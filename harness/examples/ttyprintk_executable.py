from harness.core import ProcessSet, ProcessSetMutualExclusion
from harness.entities import StateGraphSimpleNode, StateGraphSimpleAction, StateGraphSimpleMessage, StateGraphProductNode, StateGraphDerivedNode, StateGraphProductResponseMessageDestination, StateGraphProductMessage, StateGraphGroupMessageDestination
from harness.control_flow import ControlFlowBuilder, ControlFlowFormatter, ControlFlowMutexSet
from harness.codegen.control_flow import HarnessControlFlowExecutableCodegen
import ttyprintk_generic


codegen = HarnessControlFlowExecutableCodegen()
codegen.set_global_prologue('''
#include <stdlib.h>
#include <stdio.h>
#include <pthread.h>

struct S1 {
    _Atomic unsigned int connections;
    _Atomic unsigned int value;
};
                            
static struct S1 *s1_ptr;
''')
codegen.set_process_prologue(ttyprintk_generic.tty_driver, '''
static struct S1 s1 = {0};
''')

codegen.set_action(ttyprintk_generic.tty_driver_load_action, '''
s1_ptr = &s1;
s1_ptr->connections = 0;
s1_ptr->value = 0;
printf("Driver loaded\\n");
''')
codegen.set_action(ttyprintk_generic.tty_driver_unloaded_action, '''
printf("Driver unloaded\\n");
s1_ptr = NULL;
''')
codegen.set_action(ttyprintk_generic.tty_client_acquire_connection_action, '''
s1_ptr->connections++;
printf("Client %client_id% connected\\n");
''')
codegen.set_action(ttyprintk_generic.tty_client_disconnect_action, '''
s1_ptr->connections--;
printf("Client %client_id% disconnected\\n");
''')
codegen.set_action(ttyprintk_generic.tty_client_use_connection_action, '''
s1_ptr->value++;
printf("Client %client_id% active\\n");
''')

for index, client in enumerate(ttyprintk_generic.tty_clients):
    codegen.set_process_parameters(client, {
        'client_id': index
    })

print(codegen.format(ttyprintk_generic.control_flow_nodes, ttyprintk_generic.mutex_set))
