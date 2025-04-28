#include <stdlib.h>
#include <stdio.h>
#include <pthread.h>

#define __harness_mutex pthread_mutex_t
#define __harness_thread pthread_t
#define __harness_thread_create pthread_create
#define __harness_thread_join pthread_join
#define __harness_mutex_init pthread_mutex_init
#define __harness_mutex_lock pthread_mutex_lock
#define __harness_mutex_unlock pthread_mutex_unlock
#define __harness_random rand()

struct S1 {
  _Atomic int connections;
  _Atomic int value;
};
static struct S1 *s1_ptr;
      

static __harness_mutex harness_state_mutex;

static unsigned long process_tty_client1_state;
static unsigned long process_tty_client2_state;
static unsigned long process_tty_driver_state;

void *harness_kernel_module_process_tty_client1(void *harness_kernel_module_process_arg) {
  (void) harness_kernel_module_process_arg; // UNUSED
  
  for (;;) {
    __harness_mutex_lock(&harness_state_mutex);
    const unsigned long harness_process_state = process_tty_client1_state;
    int state_transition_permitted;
    __harness_mutex_unlock(&harness_state_mutex);
    switch (harness_process_state) {
      case 0: /* tty_client_nodriver */
        switch (__harness_random % 2) {
          case 0: /* tty_client_nodriver */
            __harness_mutex_lock(&harness_state_mutex);
            if ((process_tty_driver_state == 0 || process_tty_driver_state == 1 || process_tty_driver_state == 2 || process_tty_driver_state == 6 || process_tty_driver_state == 0 || process_tty_driver_state == 1 || process_tty_driver_state == 2 || process_tty_driver_state == 6) && (process_tty_client2_state == 0 || process_tty_client2_state == 1 || process_tty_client2_state == 0 || process_tty_client2_state == 1)) {
              state_transition_permitted = 1;
              process_tty_client1_state = 0;
            } else {
              state_transition_permitted = 0;
            }
            __harness_mutex_unlock(&harness_state_mutex);
            break;
          
          case 1: /* tty_client_disconnected */
            __harness_mutex_lock(&harness_state_mutex);
            if ((process_tty_client2_state == 0 || process_tty_client2_state == 1 || process_tty_client2_state == 0 || process_tty_client2_state == 2 || process_tty_client2_state == 3 || process_tty_client2_state == 4 || process_tty_client2_state == 1 || process_tty_client2_state == 2 || process_tty_client2_state == 3 || process_tty_client2_state == 4) && (process_tty_driver_state == 2 || process_tty_driver_state == 6 || process_tty_driver_state == 3 || process_tty_driver_state == 5 || process_tty_driver_state == 4 || process_tty_driver_state == 2 || process_tty_driver_state == 3 || process_tty_driver_state == 4 || process_tty_driver_state == 5 || process_tty_driver_state == 6)) {
              state_transition_permitted = 1;
              process_tty_client1_state = 1;
            } else {
              state_transition_permitted = 0;
            }
            __harness_mutex_unlock(&harness_state_mutex);
            break;
          
        }
        break;
      
      case 1: /* tty_client_disconnected */
        switch (__harness_random % 3) {
          case 0: /* tty_client_disconnected */
            __harness_mutex_lock(&harness_state_mutex);
            if ((process_tty_client2_state == 0 || process_tty_client2_state == 1 || process_tty_client2_state == 0 || process_tty_client2_state == 2 || process_tty_client2_state == 3 || process_tty_client2_state == 4 || process_tty_client2_state == 1 || process_tty_client2_state == 2 || process_tty_client2_state == 3 || process_tty_client2_state == 4) && (process_tty_driver_state == 2 || process_tty_driver_state == 6 || process_tty_driver_state == 3 || process_tty_driver_state == 5 || process_tty_driver_state == 4 || process_tty_driver_state == 2 || process_tty_driver_state == 3 || process_tty_driver_state == 4 || process_tty_driver_state == 5 || process_tty_driver_state == 6)) {
              state_transition_permitted = 1;
              process_tty_client1_state = 1;
            } else {
              state_transition_permitted = 0;
            }
            __harness_mutex_unlock(&harness_state_mutex);
            break;
          
          case 1: /* tty_client_wait_connection */
            __harness_mutex_lock(&harness_state_mutex);
            if ((process_tty_client2_state == 1 || process_tty_client2_state == 4 || process_tty_client2_state == 3 || process_tty_client2_state == 1 || process_tty_client2_state == 3 || process_tty_client2_state == 4) && (process_tty_driver_state == 3 || process_tty_driver_state == 4 || process_tty_driver_state == 2 || process_tty_driver_state == 5 || process_tty_driver_state == 2 || process_tty_driver_state == 3 || process_tty_driver_state == 4 || process_tty_driver_state == 5)) {
              state_transition_permitted = 1;
              process_tty_client1_state = 2;
            } else {
              state_transition_permitted = 0;
            }
            __harness_mutex_unlock(&harness_state_mutex);
            break;
          
          case 2: /* tty_client_nodriver */
            __harness_mutex_lock(&harness_state_mutex);
            if ((process_tty_driver_state == 0 || process_tty_driver_state == 1 || process_tty_driver_state == 2 || process_tty_driver_state == 6 || process_tty_driver_state == 0 || process_tty_driver_state == 1 || process_tty_driver_state == 2 || process_tty_driver_state == 6) && (process_tty_client2_state == 0 || process_tty_client2_state == 1 || process_tty_client2_state == 0 || process_tty_client2_state == 1)) {
              state_transition_permitted = 1;
              process_tty_client1_state = 0;
            } else {
              state_transition_permitted = 0;
            }
            __harness_mutex_unlock(&harness_state_mutex);
            break;
          
        }
        break;
      
      case 2: /* tty_client_wait_connection */
        switch (__harness_random % 3) {
          case 0: /* tty_client_wait_connection */
            __harness_mutex_lock(&harness_state_mutex);
            if ((process_tty_client2_state == 1 || process_tty_client2_state == 4 || process_tty_client2_state == 3 || process_tty_client2_state == 1 || process_tty_client2_state == 3 || process_tty_client2_state == 4) && (process_tty_driver_state == 3 || process_tty_driver_state == 4 || process_tty_driver_state == 2 || process_tty_driver_state == 5 || process_tty_driver_state == 2 || process_tty_driver_state == 3 || process_tty_driver_state == 4 || process_tty_driver_state == 5)) {
              state_transition_permitted = 1;
              process_tty_client1_state = 2;
            } else {
              state_transition_permitted = 0;
            }
            __harness_mutex_unlock(&harness_state_mutex);
            break;
          
          case 1: /* tty_client_connected_state */
            __harness_mutex_lock(&harness_state_mutex);
            if ((process_tty_client2_state == 1 || process_tty_client2_state == 2 || process_tty_client2_state == 4 || process_tty_client2_state == 1 || process_tty_client2_state == 2 || process_tty_client2_state == 3 || process_tty_client2_state == 3 || process_tty_client2_state == 4) && (process_tty_driver_state == 4 || process_tty_driver_state == 5 || process_tty_driver_state == 4 || process_tty_driver_state == 5)) {
              state_transition_permitted = 1;
              process_tty_client1_state = 3;
            } else {
              state_transition_permitted = 0;
            }
            __harness_mutex_unlock(&harness_state_mutex);
            
            if (state_transition_permitted) {
              s1_ptr->connections++;
            }
            break;
          
          case 2: /* tty_client_nodriver */
            __harness_mutex_lock(&harness_state_mutex);
            if ((process_tty_driver_state == 0 || process_tty_driver_state == 1 || process_tty_driver_state == 2 || process_tty_driver_state == 6 || process_tty_driver_state == 0 || process_tty_driver_state == 1 || process_tty_driver_state == 2 || process_tty_driver_state == 6) && (process_tty_client2_state == 0 || process_tty_client2_state == 1 || process_tty_client2_state == 0 || process_tty_client2_state == 1)) {
              state_transition_permitted = 1;
              process_tty_client1_state = 0;
            } else {
              state_transition_permitted = 0;
            }
            __harness_mutex_unlock(&harness_state_mutex);
            break;
          
        }
        break;
      
      case 3: /* tty_client_connected_state */
        switch (__harness_random % 2) {
          case 0: /* tty_client_connected_state */
            __harness_mutex_lock(&harness_state_mutex);
            if ((process_tty_client2_state == 1 || process_tty_client2_state == 2 || process_tty_client2_state == 4 || process_tty_client2_state == 1 || process_tty_client2_state == 2 || process_tty_client2_state == 3 || process_tty_client2_state == 3 || process_tty_client2_state == 4) && (process_tty_driver_state == 4 || process_tty_driver_state == 5 || process_tty_driver_state == 4 || process_tty_driver_state == 5)) {
              state_transition_permitted = 1;
              process_tty_client1_state = 3;
            } else {
              state_transition_permitted = 0;
            }
            __harness_mutex_unlock(&harness_state_mutex);
            
            if (state_transition_permitted) {
              s1_ptr->value++;
            }
            break;
          
          case 1: /* tty_client_disconnecting */
            __harness_mutex_lock(&harness_state_mutex);
            if ((process_tty_client2_state == 1 || process_tty_client2_state == 2 || process_tty_client2_state == 3 || process_tty_client2_state == 3 || process_tty_client2_state == 4 || process_tty_client2_state == 1 || process_tty_client2_state == 2 || process_tty_client2_state == 4) && (process_tty_driver_state == 4 || process_tty_driver_state == 5 || process_tty_driver_state == 4 || process_tty_driver_state == 5)) {
              state_transition_permitted = 1;
              process_tty_client1_state = 4;
            } else {
              state_transition_permitted = 0;
            }
            __harness_mutex_unlock(&harness_state_mutex);
            
            if (state_transition_permitted) {
              s1_ptr->connections--;
            }
            break;
          
        }
        break;
      
      case 4: /* tty_client_disconnecting */
        switch (__harness_random % 2) {
          case 0: /* tty_client_disconnecting */
            __harness_mutex_lock(&harness_state_mutex);
            if ((process_tty_client2_state == 1 || process_tty_client2_state == 2 || process_tty_client2_state == 3 || process_tty_client2_state == 3 || process_tty_client2_state == 4 || process_tty_client2_state == 1 || process_tty_client2_state == 2 || process_tty_client2_state == 4) && (process_tty_driver_state == 4 || process_tty_driver_state == 5 || process_tty_driver_state == 4 || process_tty_driver_state == 5)) {
              state_transition_permitted = 1;
              process_tty_client1_state = 4;
            } else {
              state_transition_permitted = 0;
            }
            __harness_mutex_unlock(&harness_state_mutex);
            break;
          
          case 1: /* tty_client_disconnected */
            __harness_mutex_lock(&harness_state_mutex);
            if ((process_tty_client2_state == 0 || process_tty_client2_state == 1 || process_tty_client2_state == 0 || process_tty_client2_state == 2 || process_tty_client2_state == 3 || process_tty_client2_state == 4 || process_tty_client2_state == 1 || process_tty_client2_state == 2 || process_tty_client2_state == 3 || process_tty_client2_state == 4) && (process_tty_driver_state == 2 || process_tty_driver_state == 6 || process_tty_driver_state == 3 || process_tty_driver_state == 5 || process_tty_driver_state == 4 || process_tty_driver_state == 2 || process_tty_driver_state == 3 || process_tty_driver_state == 4 || process_tty_driver_state == 5 || process_tty_driver_state == 6)) {
              state_transition_permitted = 1;
              process_tty_client1_state = 1;
            } else {
              state_transition_permitted = 0;
            }
            __harness_mutex_unlock(&harness_state_mutex);
            break;
          
        }
        break;
      
    }
  }
  return NULL;
}

void *harness_kernel_module_process_tty_client2(void *harness_kernel_module_process_arg) {
  (void) harness_kernel_module_process_arg; // UNUSED
  
  for (;;) {
    __harness_mutex_lock(&harness_state_mutex);
    const unsigned long harness_process_state = process_tty_client2_state;
    int state_transition_permitted;
    __harness_mutex_unlock(&harness_state_mutex);
    switch (harness_process_state) {
      case 0: /* tty_client_nodriver */
        switch (__harness_random % 2) {
          case 0: /* tty_client_nodriver */
            __harness_mutex_lock(&harness_state_mutex);
            if ((process_tty_client1_state == 0 || process_tty_client1_state == 0 || process_tty_client1_state == 1 || process_tty_client1_state == 1) && (process_tty_driver_state == 0 || process_tty_driver_state == 1 || process_tty_driver_state == 2 || process_tty_driver_state == 6 || process_tty_driver_state == 0 || process_tty_driver_state == 1 || process_tty_driver_state == 2 || process_tty_driver_state == 6)) {
              state_transition_permitted = 1;
              process_tty_client2_state = 0;
            } else {
              state_transition_permitted = 0;
            }
            __harness_mutex_unlock(&harness_state_mutex);
            break;
          
          case 1: /* tty_client_disconnected */
            __harness_mutex_lock(&harness_state_mutex);
            if ((process_tty_client1_state == 0 || process_tty_client1_state == 1 || process_tty_client1_state == 0 || process_tty_client1_state == 1 || process_tty_client1_state == 3 || process_tty_client1_state == 4 || process_tty_client1_state == 2 || process_tty_client1_state == 2 || process_tty_client1_state == 3 || process_tty_client1_state == 4) && (process_tty_driver_state == 2 || process_tty_driver_state == 6 || process_tty_driver_state == 3 || process_tty_driver_state == 4 || process_tty_driver_state == 5 || process_tty_driver_state == 2 || process_tty_driver_state == 3 || process_tty_driver_state == 4 || process_tty_driver_state == 5 || process_tty_driver_state == 6)) {
              state_transition_permitted = 1;
              process_tty_client2_state = 1;
            } else {
              state_transition_permitted = 0;
            }
            __harness_mutex_unlock(&harness_state_mutex);
            break;
          
        }
        break;
      
      case 1: /* tty_client_disconnected */
        switch (__harness_random % 3) {
          case 0: /* tty_client_disconnected */
            __harness_mutex_lock(&harness_state_mutex);
            if ((process_tty_client1_state == 0 || process_tty_client1_state == 1 || process_tty_client1_state == 0 || process_tty_client1_state == 1 || process_tty_client1_state == 3 || process_tty_client1_state == 4 || process_tty_client1_state == 2 || process_tty_client1_state == 2 || process_tty_client1_state == 3 || process_tty_client1_state == 4) && (process_tty_driver_state == 2 || process_tty_driver_state == 6 || process_tty_driver_state == 3 || process_tty_driver_state == 4 || process_tty_driver_state == 5 || process_tty_driver_state == 2 || process_tty_driver_state == 3 || process_tty_driver_state == 4 || process_tty_driver_state == 5 || process_tty_driver_state == 6)) {
              state_transition_permitted = 1;
              process_tty_client2_state = 1;
            } else {
              state_transition_permitted = 0;
            }
            __harness_mutex_unlock(&harness_state_mutex);
            break;
          
          case 1: /* tty_client_wait_connection */
            __harness_mutex_lock(&harness_state_mutex);
            if ((process_tty_client1_state == 1 || process_tty_client1_state == 1 || process_tty_client1_state == 3 || process_tty_client1_state == 4 || process_tty_client1_state == 3 || process_tty_client1_state == 4) && (process_tty_driver_state == 2 || process_tty_driver_state == 3 || process_tty_driver_state == 5 || process_tty_driver_state == 4 || process_tty_driver_state == 2 || process_tty_driver_state == 3 || process_tty_driver_state == 4 || process_tty_driver_state == 5)) {
              state_transition_permitted = 1;
              process_tty_client2_state = 2;
            } else {
              state_transition_permitted = 0;
            }
            __harness_mutex_unlock(&harness_state_mutex);
            break;
          
          case 2: /* tty_client_nodriver */
            __harness_mutex_lock(&harness_state_mutex);
            if ((process_tty_client1_state == 0 || process_tty_client1_state == 0 || process_tty_client1_state == 1 || process_tty_client1_state == 1) && (process_tty_driver_state == 0 || process_tty_driver_state == 1 || process_tty_driver_state == 2 || process_tty_driver_state == 6 || process_tty_driver_state == 0 || process_tty_driver_state == 1 || process_tty_driver_state == 2 || process_tty_driver_state == 6)) {
              state_transition_permitted = 1;
              process_tty_client2_state = 0;
            } else {
              state_transition_permitted = 0;
            }
            __harness_mutex_unlock(&harness_state_mutex);
            break;
          
        }
        break;
      
      case 2: /* tty_client_wait_connection */
        switch (__harness_random % 3) {
          case 0: /* tty_client_wait_connection */
            __harness_mutex_lock(&harness_state_mutex);
            if ((process_tty_client1_state == 1 || process_tty_client1_state == 1 || process_tty_client1_state == 3 || process_tty_client1_state == 4 || process_tty_client1_state == 3 || process_tty_client1_state == 4) && (process_tty_driver_state == 2 || process_tty_driver_state == 3 || process_tty_driver_state == 5 || process_tty_driver_state == 4 || process_tty_driver_state == 2 || process_tty_driver_state == 3 || process_tty_driver_state == 4 || process_tty_driver_state == 5)) {
              state_transition_permitted = 1;
              process_tty_client2_state = 2;
            } else {
              state_transition_permitted = 0;
            }
            __harness_mutex_unlock(&harness_state_mutex);
            break;
          
          case 1: /* tty_client_connected_state */
            __harness_mutex_lock(&harness_state_mutex);
            if ((process_tty_client1_state == 1 || process_tty_client1_state == 2 || process_tty_client1_state == 3 || process_tty_client1_state == 1 || process_tty_client1_state == 3 || process_tty_client1_state == 4 || process_tty_client1_state == 2 || process_tty_client1_state == 4) && (process_tty_driver_state == 3 || process_tty_driver_state == 4 || process_tty_driver_state == 3 || process_tty_driver_state == 4)) {
              state_transition_permitted = 1;
              process_tty_client2_state = 3;
            } else {
              state_transition_permitted = 0;
            }
            __harness_mutex_unlock(&harness_state_mutex);
            
            if (state_transition_permitted) {
              s1_ptr->connections++;
            }
            break;
          
          case 2: /* tty_client_nodriver */
            __harness_mutex_lock(&harness_state_mutex);
            if ((process_tty_client1_state == 0 || process_tty_client1_state == 0 || process_tty_client1_state == 1 || process_tty_client1_state == 1) && (process_tty_driver_state == 0 || process_tty_driver_state == 1 || process_tty_driver_state == 2 || process_tty_driver_state == 6 || process_tty_driver_state == 0 || process_tty_driver_state == 1 || process_tty_driver_state == 2 || process_tty_driver_state == 6)) {
              state_transition_permitted = 1;
              process_tty_client2_state = 0;
            } else {
              state_transition_permitted = 0;
            }
            __harness_mutex_unlock(&harness_state_mutex);
            break;
          
        }
        break;
      
      case 3: /* tty_client_connected_state */
        switch (__harness_random % 2) {
          case 0: /* tty_client_connected_state */
            __harness_mutex_lock(&harness_state_mutex);
            if ((process_tty_client1_state == 1 || process_tty_client1_state == 2 || process_tty_client1_state == 3 || process_tty_client1_state == 1 || process_tty_client1_state == 3 || process_tty_client1_state == 4 || process_tty_client1_state == 2 || process_tty_client1_state == 4) && (process_tty_driver_state == 3 || process_tty_driver_state == 4 || process_tty_driver_state == 3 || process_tty_driver_state == 4)) {
              state_transition_permitted = 1;
              process_tty_client2_state = 3;
            } else {
              state_transition_permitted = 0;
            }
            __harness_mutex_unlock(&harness_state_mutex);
            
            if (state_transition_permitted) {
              s1_ptr->value++;
            }
            break;
          
          case 1: /* tty_client_disconnecting */
            __harness_mutex_lock(&harness_state_mutex);
            if ((process_tty_client1_state == 1 || process_tty_client1_state == 2 || process_tty_client1_state == 3 || process_tty_client1_state == 4 || process_tty_client1_state == 1 || process_tty_client1_state == 2 || process_tty_client1_state == 3 || process_tty_client1_state == 4) && (process_tty_driver_state == 3 || process_tty_driver_state == 4 || process_tty_driver_state == 3 || process_tty_driver_state == 4)) {
              state_transition_permitted = 1;
              process_tty_client2_state = 4;
            } else {
              state_transition_permitted = 0;
            }
            __harness_mutex_unlock(&harness_state_mutex);
            
            if (state_transition_permitted) {
              s1_ptr->connections--;
            }
            break;
          
        }
        break;
      
      case 4: /* tty_client_disconnecting */
        switch (__harness_random % 2) {
          case 0: /* tty_client_disconnecting */
            __harness_mutex_lock(&harness_state_mutex);
            if ((process_tty_client1_state == 1 || process_tty_client1_state == 2 || process_tty_client1_state == 3 || process_tty_client1_state == 4 || process_tty_client1_state == 1 || process_tty_client1_state == 2 || process_tty_client1_state == 3 || process_tty_client1_state == 4) && (process_tty_driver_state == 3 || process_tty_driver_state == 4 || process_tty_driver_state == 3 || process_tty_driver_state == 4)) {
              state_transition_permitted = 1;
              process_tty_client2_state = 4;
            } else {
              state_transition_permitted = 0;
            }
            __harness_mutex_unlock(&harness_state_mutex);
            break;
          
          case 1: /* tty_client_disconnected */
            __harness_mutex_lock(&harness_state_mutex);
            if ((process_tty_client1_state == 0 || process_tty_client1_state == 1 || process_tty_client1_state == 0 || process_tty_client1_state == 1 || process_tty_client1_state == 3 || process_tty_client1_state == 4 || process_tty_client1_state == 2 || process_tty_client1_state == 2 || process_tty_client1_state == 3 || process_tty_client1_state == 4) && (process_tty_driver_state == 2 || process_tty_driver_state == 6 || process_tty_driver_state == 3 || process_tty_driver_state == 4 || process_tty_driver_state == 5 || process_tty_driver_state == 2 || process_tty_driver_state == 3 || process_tty_driver_state == 4 || process_tty_driver_state == 5 || process_tty_driver_state == 6)) {
              state_transition_permitted = 1;
              process_tty_client2_state = 1;
            } else {
              state_transition_permitted = 0;
            }
            __harness_mutex_unlock(&harness_state_mutex);
            break;
          
        }
        break;
      
    }
  }
  return NULL;
}

void *harness_kernel_module_process_tty_driver(void *harness_kernel_module_process_arg) {
  (void) harness_kernel_module_process_arg; // UNUSED
  
  for (;;) {
    __harness_mutex_lock(&harness_state_mutex);
    const unsigned long harness_process_state = process_tty_driver_state;
    int state_transition_permitted;
    __harness_mutex_unlock(&harness_state_mutex);
    switch (harness_process_state) {
      case 0: /* tty_driver_unloaded */
        switch (__harness_random % 2) {
          case 0: /* tty_driver_unloaded */
            __harness_mutex_lock(&harness_state_mutex);
            if ((process_tty_client1_state == 0 || process_tty_client1_state == 0) && (process_tty_client2_state == 0 || process_tty_client2_state == 0)) {
              state_transition_permitted = 1;
              process_tty_driver_state = 0;
            } else {
              state_transition_permitted = 0;
            }
            __harness_mutex_unlock(&harness_state_mutex);
            break;
          
          case 1: /* tty_driver_loading */
            __harness_mutex_lock(&harness_state_mutex);
            if ((process_tty_client1_state == 0 || process_tty_client1_state == 0) && (process_tty_client2_state == 0 || process_tty_client2_state == 0)) {
              state_transition_permitted = 1;
              process_tty_driver_state = 1;
            } else {
              state_transition_permitted = 0;
            }
            __harness_mutex_unlock(&harness_state_mutex);
            
            if (state_transition_permitted) {
              static struct S1 s1;
              s1.connections = 0;
              s1.value = 0;
              s1_ptr = &s1;
            }
            break;
          
        }
        break;
      
      case 1: /* tty_driver_loading */
        switch (__harness_random % 2) {
          case 0: /* tty_driver_loading */
            __harness_mutex_lock(&harness_state_mutex);
            if ((process_tty_client1_state == 0 || process_tty_client1_state == 0) && (process_tty_client2_state == 0 || process_tty_client2_state == 0)) {
              state_transition_permitted = 1;
              process_tty_driver_state = 1;
            } else {
              state_transition_permitted = 0;
            }
            __harness_mutex_unlock(&harness_state_mutex);
            break;
          
          case 1: /* tty_driver_loaded (tty_driver_client_inactive, tty_driver_client_inactive) */
            __harness_mutex_lock(&harness_state_mutex);
            if ((process_tty_client1_state == 0 || process_tty_client1_state == 1 || process_tty_client1_state == 2 || process_tty_client1_state == 0 || process_tty_client1_state == 1 || process_tty_client1_state == 2) && (process_tty_client2_state == 0 || process_tty_client2_state == 1 || process_tty_client2_state == 2 || process_tty_client2_state == 0 || process_tty_client2_state == 1 || process_tty_client2_state == 2)) {
              state_transition_permitted = 1;
              process_tty_driver_state = 2;
            } else {
              state_transition_permitted = 0;
            }
            __harness_mutex_unlock(&harness_state_mutex);
            break;
          
        }
        break;
      
      case 2: /* tty_driver_loaded (tty_driver_client_inactive, tty_driver_client_inactive) */
        switch (__harness_random % 3) {
          case 0: /* tty_driver_unloading */
            __harness_mutex_lock(&harness_state_mutex);
            if ((process_tty_client1_state == 0 || process_tty_client1_state == 1 || process_tty_client1_state == 1 || process_tty_client1_state == 0) && (process_tty_client2_state == 0 || process_tty_client2_state == 1 || process_tty_client2_state == 1 || process_tty_client2_state == 0)) {
              state_transition_permitted = 1;
              process_tty_driver_state = 6;
            } else {
              state_transition_permitted = 0;
            }
            __harness_mutex_unlock(&harness_state_mutex);
            
            if (state_transition_permitted) {
              s1_ptr = NULL;
            }
            break;
          
          case 1: /* tty_driver_loaded (tty_driver_client_active, tty_driver_client_inactive) */
            __harness_mutex_lock(&harness_state_mutex);
            if ((process_tty_client1_state == 1 || process_tty_client1_state == 2 || process_tty_client1_state == 3 || process_tty_client1_state == 4 || process_tty_client1_state == 3 || process_tty_client1_state == 4 || process_tty_client1_state == 1 || process_tty_client1_state == 2) && (process_tty_client2_state == 1 || process_tty_client2_state == 2 || process_tty_client2_state == 1 || process_tty_client2_state == 2)) {
              state_transition_permitted = 1;
              process_tty_driver_state = 5;
            } else {
              state_transition_permitted = 0;
            }
            __harness_mutex_unlock(&harness_state_mutex);
            break;
          
          case 2: /* tty_driver_loaded (tty_driver_client_inactive, tty_driver_client_active) */
            __harness_mutex_lock(&harness_state_mutex);
            if ((process_tty_client1_state == 1 || process_tty_client1_state == 2 || process_tty_client1_state == 1 || process_tty_client1_state == 2) && (process_tty_client2_state == 1 || process_tty_client2_state == 2 || process_tty_client2_state == 3 || process_tty_client2_state == 4 || process_tty_client2_state == 2 || process_tty_client2_state == 3 || process_tty_client2_state == 4 || process_tty_client2_state == 1)) {
              state_transition_permitted = 1;
              process_tty_driver_state = 3;
            } else {
              state_transition_permitted = 0;
            }
            __harness_mutex_unlock(&harness_state_mutex);
            break;
          
        }
        break;
      
      case 3: /* tty_driver_loaded (tty_driver_client_inactive, tty_driver_client_active) */
        switch (__harness_random % 2) {
          case 0: /* tty_driver_loaded (tty_driver_client_active, tty_driver_client_active) */
            __harness_mutex_lock(&harness_state_mutex);
            if ((process_tty_client1_state == 1 || process_tty_client1_state == 2 || process_tty_client1_state == 3 || process_tty_client1_state == 4 || process_tty_client1_state == 2 || process_tty_client1_state == 3 || process_tty_client1_state == 4 || process_tty_client1_state == 1) && (process_tty_client2_state == 1 || process_tty_client2_state == 2 || process_tty_client2_state == 3 || process_tty_client2_state == 4 || process_tty_client2_state == 4 || process_tty_client2_state == 1 || process_tty_client2_state == 2 || process_tty_client2_state == 3)) {
              state_transition_permitted = 1;
              process_tty_driver_state = 4;
            } else {
              state_transition_permitted = 0;
            }
            __harness_mutex_unlock(&harness_state_mutex);
            break;
          
          case 1: /* tty_driver_loaded (tty_driver_client_inactive, tty_driver_client_inactive) */
            __harness_mutex_lock(&harness_state_mutex);
            if ((process_tty_client1_state == 0 || process_tty_client1_state == 1 || process_tty_client1_state == 2 || process_tty_client1_state == 0 || process_tty_client1_state == 1 || process_tty_client1_state == 2) && (process_tty_client2_state == 0 || process_tty_client2_state == 1 || process_tty_client2_state == 2 || process_tty_client2_state == 0 || process_tty_client2_state == 1 || process_tty_client2_state == 2)) {
              state_transition_permitted = 1;
              process_tty_driver_state = 2;
            } else {
              state_transition_permitted = 0;
            }
            __harness_mutex_unlock(&harness_state_mutex);
            break;
          
        }
        break;
      
      case 4: /* tty_driver_loaded (tty_driver_client_active, tty_driver_client_active) */
        switch (__harness_random % 2) {
          case 0: /* tty_driver_loaded (tty_driver_client_inactive, tty_driver_client_active) */
            __harness_mutex_lock(&harness_state_mutex);
            if ((process_tty_client1_state == 1 || process_tty_client1_state == 2 || process_tty_client1_state == 1 || process_tty_client1_state == 2) && (process_tty_client2_state == 1 || process_tty_client2_state == 2 || process_tty_client2_state == 3 || process_tty_client2_state == 4 || process_tty_client2_state == 2 || process_tty_client2_state == 3 || process_tty_client2_state == 4 || process_tty_client2_state == 1)) {
              state_transition_permitted = 1;
              process_tty_driver_state = 3;
            } else {
              state_transition_permitted = 0;
            }
            __harness_mutex_unlock(&harness_state_mutex);
            break;
          
          case 1: /* tty_driver_loaded (tty_driver_client_active, tty_driver_client_inactive) */
            __harness_mutex_lock(&harness_state_mutex);
            if ((process_tty_client1_state == 1 || process_tty_client1_state == 2 || process_tty_client1_state == 3 || process_tty_client1_state == 4 || process_tty_client1_state == 3 || process_tty_client1_state == 4 || process_tty_client1_state == 1 || process_tty_client1_state == 2) && (process_tty_client2_state == 1 || process_tty_client2_state == 2 || process_tty_client2_state == 1 || process_tty_client2_state == 2)) {
              state_transition_permitted = 1;
              process_tty_driver_state = 5;
            } else {
              state_transition_permitted = 0;
            }
            __harness_mutex_unlock(&harness_state_mutex);
            break;
          
        }
        break;
      
      case 5: /* tty_driver_loaded (tty_driver_client_active, tty_driver_client_inactive) */
        switch (__harness_random % 2) {
          case 0: /* tty_driver_loaded (tty_driver_client_inactive, tty_driver_client_inactive) */
            __harness_mutex_lock(&harness_state_mutex);
            if ((process_tty_client1_state == 0 || process_tty_client1_state == 1 || process_tty_client1_state == 2 || process_tty_client1_state == 0 || process_tty_client1_state == 1 || process_tty_client1_state == 2) && (process_tty_client2_state == 0 || process_tty_client2_state == 1 || process_tty_client2_state == 2 || process_tty_client2_state == 0 || process_tty_client2_state == 1 || process_tty_client2_state == 2)) {
              state_transition_permitted = 1;
              process_tty_driver_state = 2;
            } else {
              state_transition_permitted = 0;
            }
            __harness_mutex_unlock(&harness_state_mutex);
            break;
          
          case 1: /* tty_driver_loaded (tty_driver_client_active, tty_driver_client_active) */
            __harness_mutex_lock(&harness_state_mutex);
            if ((process_tty_client1_state == 1 || process_tty_client1_state == 2 || process_tty_client1_state == 3 || process_tty_client1_state == 4 || process_tty_client1_state == 2 || process_tty_client1_state == 3 || process_tty_client1_state == 4 || process_tty_client1_state == 1) && (process_tty_client2_state == 1 || process_tty_client2_state == 2 || process_tty_client2_state == 3 || process_tty_client2_state == 4 || process_tty_client2_state == 4 || process_tty_client2_state == 1 || process_tty_client2_state == 2 || process_tty_client2_state == 3)) {
              state_transition_permitted = 1;
              process_tty_driver_state = 4;
            } else {
              state_transition_permitted = 0;
            }
            __harness_mutex_unlock(&harness_state_mutex);
            break;
          
        }
        break;
      
      case 6: /* tty_driver_unloading */
        switch (__harness_random % 2) {
          case 0: /* tty_driver_unloading */
            __harness_mutex_lock(&harness_state_mutex);
            if ((process_tty_client1_state == 0 || process_tty_client1_state == 1 || process_tty_client1_state == 1 || process_tty_client1_state == 0) && (process_tty_client2_state == 0 || process_tty_client2_state == 1 || process_tty_client2_state == 1 || process_tty_client2_state == 0)) {
              state_transition_permitted = 1;
              process_tty_driver_state = 6;
            } else {
              state_transition_permitted = 0;
            }
            __harness_mutex_unlock(&harness_state_mutex);
            break;
          
          case 1: /* tty_driver_unloaded */
            __harness_mutex_lock(&harness_state_mutex);
            if ((process_tty_client1_state == 0 || process_tty_client1_state == 0) && (process_tty_client2_state == 0 || process_tty_client2_state == 0)) {
              state_transition_permitted = 1;
              process_tty_driver_state = 0;
            } else {
              state_transition_permitted = 0;
            }
            __harness_mutex_unlock(&harness_state_mutex);
            break;
          
        }
        break;
      
    }
  }
  return NULL;
}

int main(void) {
  __harness_mutex_init(&harness_state_mutex, NULL);
  process_tty_client1_state = 0;
  process_tty_client2_state = 0;
  process_tty_driver_state = 0;
  
  __harness_thread process_tty_client1;
  __harness_thread process_tty_client2;
  __harness_thread process_tty_driver;
  
  __harness_thread_create(&process_tty_client1, NULL, harness_kernel_module_process_tty_client1, NULL);
  __harness_thread_create(&process_tty_client2, NULL, harness_kernel_module_process_tty_client2, NULL);
  __harness_thread_create(&process_tty_driver, NULL, harness_kernel_module_process_tty_driver, NULL);
  
  __harness_thread_join(process_tty_client1, NULL);
  __harness_thread_join(process_tty_client2, NULL);
  __harness_thread_join(process_tty_driver, NULL);
  return 0;
}

