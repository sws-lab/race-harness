from harness.codegen.control_flow import HarnessControlFlowGoblintCodegen
import ttyprintk_generic


codegen = HarnessControlFlowGoblintCodegen()
codegen.set_global_prologue('''
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
''')
for client in ttyprintk_generic.tty_clients:
    codegen.set_process_prologue(client, '''
    struct tty_struct tty;
    struct file file;
    const char content[] = "client%client_id%";
    ''')

codegen.set_action(ttyprintk_generic.tty_driver_load_action, '''
init_module();
''')
codegen.set_action(ttyprintk_generic.tty_driver_unloaded_action, '''
cleanup_module();
''')
codegen.set_action(ttyprintk_generic.tty_client_acquire_connection_action, '''
registered_tty_driver->ops->open(&tty, &file);
''')
codegen.set_action(ttyprintk_generic.tty_client_disconnect_action, '''
registered_tty_driver->ops->close(&tty, &file);
''')
codegen.set_action(ttyprintk_generic.tty_client_use_connection_action, '''
registered_tty_driver->ops->write(&tty, content, sizeof(content));
''')

for index, client in enumerate(ttyprintk_generic.tty_clients):
    codegen.set_process_parameters(client, {
        'client_id': index
    })

print(codegen.format(ttyprintk_generic.control_flow_nodes, ttyprintk_generic.mutex_set))
