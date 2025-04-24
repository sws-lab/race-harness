
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

typedef long __harness_mutex;
typedef long __harness_thread;
void __harness_thread_create(__harness_thread *, void *, void *(*)(void *), void *);
void __harness_thread_join(__harness_thread *, void *);
void __harness_mutex_init(__harness_mutex *);
void __harness_mutex_lock(__harness_mutex *);
void __harness_mutex_unlock(__harness_mutex *);
extern volatile long __harness_random;

extern struct tty_driver *registered_tty_driver;
      

static __harness_mutex harness_kernel_module_invariant_0;
static __harness_mutex harness_kernel_module_invariant_1;
static __harness_mutex harness_kernel_module_invariant_2;
static __harness_mutex harness_kernel_module_invariant_3;
static __harness_mutex harness_kernel_module_invariant_4;
static __harness_mutex harness_kernel_module_invariant_5;
static __harness_mutex harness_kernel_module_invariant_6;

void *harness_kernel_module_process_tty_client1(void *harness_kernel_module_process_arg) {
  (void) harness_kernel_module_process_arg; // UNUSED
  struct tty_struct tty;
  struct file file;
  const char content[] = "tty_client1";
  unsigned long harness_kernel_module_process_state = 0;
  for (;;) {
    switch (harness_kernel_module_process_state) {
      case 0: /* tty_client_nodriver */
        __harness_mutex_lock(&harness_kernel_module_invariant_0);
        __harness_mutex_lock(&harness_kernel_module_invariant_6);
        switch (__harness_random % 2) {
          case 0:
            harness_kernel_module_process_state = 0; /* tty_client_nodriver */
            break;
          
          case 1:
            harness_kernel_module_process_state = 1; /* tty_client_disconnected */
            break;
          
        }
        __harness_mutex_unlock(&harness_kernel_module_invariant_6);
        __harness_mutex_unlock(&harness_kernel_module_invariant_0);
        break;
      
      case 1: /* tty_client_disconnected */
        __harness_mutex_lock(&harness_kernel_module_invariant_1);
        switch (__harness_random % 3) {
          case 0:
            harness_kernel_module_process_state = 1; /* tty_client_disconnected */
            break;
          
          case 1:
            harness_kernel_module_process_state = 2; /* tty_client_wait_connection */
            break;
          
          case 2:
            harness_kernel_module_process_state = 0; /* tty_client_nodriver */
            break;
          
        }
        __harness_mutex_unlock(&harness_kernel_module_invariant_1);
        break;
      
      case 2: /* tty_client_wait_connection */
        __harness_mutex_lock(&harness_kernel_module_invariant_2);
        __harness_mutex_lock(&harness_kernel_module_invariant_4);
        switch (__harness_random % 3) {
          case 0:
            harness_kernel_module_process_state = 2; /* tty_client_wait_connection */
            break;
          
          case 1:
            {
              registered_tty_driver->ops->open(&tty, &file);
            }
            harness_kernel_module_process_state = 3; /* tty_client_connected_state */
            break;
          
          case 2:
            harness_kernel_module_process_state = 0; /* tty_client_nodriver */
            break;
          
        }
        __harness_mutex_unlock(&harness_kernel_module_invariant_4);
        __harness_mutex_unlock(&harness_kernel_module_invariant_2);
        break;
      
      case 3: /* tty_client_connected_state */
        __harness_mutex_lock(&harness_kernel_module_invariant_3);
        __harness_mutex_lock(&harness_kernel_module_invariant_4);
        __harness_mutex_lock(&harness_kernel_module_invariant_5);
        switch (__harness_random % 2) {
          case 0:
            {
              registered_tty_driver->ops->write(&tty, content, sizeof(content));
            }
            harness_kernel_module_process_state = 3; /* tty_client_connected_state */
            break;
          
          case 1:
            {
              registered_tty_driver->ops->close(&tty, &file);
            }
            harness_kernel_module_process_state = 1; /* tty_client_disconnected */
            break;
          
        }
        __harness_mutex_unlock(&harness_kernel_module_invariant_5);
        __harness_mutex_unlock(&harness_kernel_module_invariant_4);
        __harness_mutex_unlock(&harness_kernel_module_invariant_3);
        break;
      
    }
  }
  return NULL;
}

void *harness_kernel_module_process_tty_driver(void *harness_kernel_module_process_arg) {
  (void) harness_kernel_module_process_arg; // UNUSED
  unsigned long harness_kernel_module_process_state = 0;
  for (;;) {
    switch (harness_kernel_module_process_state) {
      case 0: /* tty_driver_unloaded */
        __harness_mutex_lock(&harness_kernel_module_invariant_2);
        __harness_mutex_lock(&harness_kernel_module_invariant_3);
        __harness_mutex_lock(&harness_kernel_module_invariant_4);
        switch (__harness_random % 2) {
          case 0:
            harness_kernel_module_process_state = 0; /* tty_driver_unloaded */
            break;
          
          case 1:
            {
              init_module();
            }
            harness_kernel_module_process_state = 1; /* tty_driver_loaded (tty_driver_client_inactive) */
            break;
          
        }
        __harness_mutex_unlock(&harness_kernel_module_invariant_4);
        __harness_mutex_unlock(&harness_kernel_module_invariant_3);
        __harness_mutex_unlock(&harness_kernel_module_invariant_2);
        break;
      
      case 1: /* tty_driver_loaded (tty_driver_client_inactive) */
        __harness_mutex_lock(&harness_kernel_module_invariant_3);
        __harness_mutex_lock(&harness_kernel_module_invariant_5);
        switch (__harness_random % 2) {
          case 0:
            {
              cleanup_module();
            }
            harness_kernel_module_process_state = 0; /* tty_driver_unloaded */
            break;
          
          case 1:
            harness_kernel_module_process_state = 2; /* tty_driver_loaded (tty_driver_client_active) */
            break;
          
        }
        __harness_mutex_unlock(&harness_kernel_module_invariant_5);
        __harness_mutex_unlock(&harness_kernel_module_invariant_3);
        break;
      
      case 2: /* tty_driver_loaded (tty_driver_client_active) */
        __harness_mutex_lock(&harness_kernel_module_invariant_0);
        __harness_mutex_lock(&harness_kernel_module_invariant_6);
        switch (__harness_random % 1) {
          case 0:
            harness_kernel_module_process_state = 1; /* tty_driver_loaded (tty_driver_client_inactive) */
            break;
          
        }
        __harness_mutex_unlock(&harness_kernel_module_invariant_6);
        __harness_mutex_unlock(&harness_kernel_module_invariant_0);
        break;
      
    }
  }
  return NULL;
}

int main(void) {
  __harness_thread process0;
  __harness_thread process1;
  __harness_mutex_init(&harness_kernel_module_invariant_0);
  __harness_mutex_init(&harness_kernel_module_invariant_1);
  __harness_mutex_init(&harness_kernel_module_invariant_2);
  __harness_mutex_init(&harness_kernel_module_invariant_3);
  __harness_mutex_init(&harness_kernel_module_invariant_4);
  __harness_mutex_init(&harness_kernel_module_invariant_5);
  __harness_mutex_init(&harness_kernel_module_invariant_6);
  
  __harness_thread_create(&process0, NULL, harness_kernel_module_process_tty_client1, NULL);
  __harness_thread_create(&process1, NULL, harness_kernel_module_process_tty_driver, NULL);
  
  __harness_thread_join(&process0, NULL);
  __harness_thread_join(&process1, NULL);
  
  return 0;
}

