
extern _Atomic long _harness_random;

typedef unsigned int __harness_thread_t;
typedef unsigned int __harness_mutex_t;
extern void __harness_thread_create(__harness_thread_t *, void *, void *(*)(void *), void *);
extern void __harness_thread_join(__harness_thread_t, void **);
extern void __harness_mutex_init(__harness_mutex_t *, void *);
extern void __harness_mutex_lock(__harness_mutex_t *);
extern void __harness_mutex_unlock(__harness_mutex_t *);



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


static _Atomic unsigned int process_tty_client1_init_barrier;
static _Atomic unsigned int process_tty_client2_init_barrier;
static _Atomic unsigned int process_tty_client3_init_barrier;
static _Atomic unsigned int process_tty_driver_init_barrier;

/* tty_driver: tty_driver_loaded (tty_driver_client_active, tty_driver_client_active, tty_driver_client_active), tty_client1: tty_client_nodriver, tty_driver: tty_driver_loaded (tty_driver_client_active, tty_driver_client_active, tty_driver_client_inactive), tty_driver: tty_driver_loaded (tty_driver_client_active, tty_driver_client_inactive, tty_driver_client_inactive), tty_driver: tty_driver_loaded (tty_driver_client_active, tty_driver_client_inactive, tty_driver_client_active) */
static __harness_mutex_t mutex0;

/* tty_driver: tty_driver_unloaded, tty_driver: tty_driver_loading, tty_driver: tty_driver_unloading, tty_client2: tty_client_wait_connection */
static __harness_mutex_t mutex1;

/* tty_driver: tty_driver_loaded (tty_driver_client_active, tty_driver_client_inactive, tty_driver_client_inactive), tty_driver: tty_driver_loaded (tty_driver_client_inactive, tty_driver_client_inactive, tty_driver_client_inactive), tty_driver: tty_driver_unloading, tty_driver: tty_driver_unloaded, tty_client2: tty_client_connected_state, tty_driver: tty_driver_loaded (tty_driver_client_active, tty_driver_client_inactive, tty_driver_client_active), tty_driver: tty_driver_loading, tty_driver: tty_driver_loaded (tty_driver_client_inactive, tty_driver_client_inactive, tty_driver_client_active), tty_client2: tty_client_disconnecting */
static __harness_mutex_t mutex2;

/* tty_client3: tty_client_wait_connection, tty_driver: tty_driver_loading, tty_driver: tty_driver_unloading, tty_driver: tty_driver_unloaded */
static __harness_mutex_t mutex3;

/* tty_client3: tty_client_disconnecting, tty_driver: tty_driver_loaded (tty_driver_client_active, tty_driver_client_active, tty_driver_client_inactive), tty_driver: tty_driver_loaded (tty_driver_client_active, tty_driver_client_inactive, tty_driver_client_inactive), tty_driver: tty_driver_loaded (tty_driver_client_inactive, tty_driver_client_inactive, tty_driver_client_inactive), tty_driver: tty_driver_loaded (tty_driver_client_inactive, tty_driver_client_active, tty_driver_client_inactive), tty_driver: tty_driver_unloaded, tty_driver: tty_driver_unloading, tty_client3: tty_client_connected_state, tty_driver: tty_driver_loading */
static __harness_mutex_t mutex4;

/* tty_driver: tty_driver_loaded (tty_driver_client_inactive, tty_driver_client_active, tty_driver_client_inactive), tty_driver: tty_driver_loaded (tty_driver_client_active, tty_driver_client_active, tty_driver_client_active), tty_client2: tty_client_nodriver, tty_driver: tty_driver_loaded (tty_driver_client_active, tty_driver_client_active, tty_driver_client_inactive), tty_driver: tty_driver_loaded (tty_driver_client_inactive, tty_driver_client_active, tty_driver_client_active) */
static __harness_mutex_t mutex5;

/* tty_driver: tty_driver_loaded (tty_driver_client_inactive, tty_driver_client_inactive, tty_driver_client_active), tty_driver: tty_driver_loaded (tty_driver_client_inactive, tty_driver_client_inactive, tty_driver_client_inactive), tty_driver: tty_driver_loaded (tty_driver_client_inactive, tty_driver_client_active, tty_driver_client_inactive), tty_driver: tty_driver_unloaded, tty_client1: tty_client_connected_state, tty_driver: tty_driver_unloading, tty_driver: tty_driver_loading, tty_driver: tty_driver_loaded (tty_driver_client_inactive, tty_driver_client_active, tty_driver_client_active), tty_client1: tty_client_disconnecting */
static __harness_mutex_t mutex6;

/* tty_client3: tty_client_nodriver, tty_driver: tty_driver_loaded (tty_driver_client_active, tty_driver_client_active, tty_driver_client_active), tty_driver: tty_driver_loaded (tty_driver_client_inactive, tty_driver_client_inactive, tty_driver_client_active), tty_driver: tty_driver_loaded (tty_driver_client_active, tty_driver_client_inactive, tty_driver_client_active), tty_driver: tty_driver_loaded (tty_driver_client_inactive, tty_driver_client_active, tty_driver_client_active) */
static __harness_mutex_t mutex7;

/* tty_driver: tty_driver_loading, tty_driver: tty_driver_unloading, tty_driver: tty_driver_unloaded, tty_client1: tty_client_wait_connection */
static __harness_mutex_t mutex8;


void *process_tty_client1(void *arg) {
  (void) arg; // Unused
  
  
      struct tty_struct tty;
      struct file file;
      const char content[] = "client0";
      
  
  {
    __harness_mutex_lock(&mutex0);
    process_tty_client1_init_barrier = 1;
    while (!process_tty_client2_init_barrier || !process_tty_client3_init_barrier || !process_tty_driver_init_barrier); // Wait for other processes
    label0 /* tty_client_nodriver */: if ((_harness_random % 2) == 0) {
      /* (tty_client_nodriver -> tty_client_nodriver) */
      goto label0; /* tty_client_nodriver */
    } else {
      __harness_mutex_unlock(&mutex0);
      /* (tty_client_nodriver -> tty_client_disconnected on tty_driver_loaded) */
      label1 /* tty_client_disconnected */: if ((_harness_random % 3) == 0) {
        /* (tty_client_disconnected -> tty_client_disconnected) */
        goto label1; /* tty_client_disconnected */
      } else if ((_harness_random % 3) == 0) {
        __harness_mutex_lock(&mutex8);
        /* (tty_client_disconnected -> tty_client_wait_connection) */
        label2 /* tty_client_wait_connection */: if ((_harness_random % 3) == 0) {
          /* (tty_client_wait_connection -> tty_client_wait_connection) */
          goto label2; /* tty_client_wait_connection */
        } else if ((_harness_random % 3) == 0) {
          __harness_mutex_lock(&mutex6);
          __harness_mutex_unlock(&mutex8);
          /* (tty_client_wait_connection -> tty_client_connected_state on tty_driver_grant_connection) */
          
          registered_tty_driver->ops->open(&tty, &file);
          
          label3 /* tty_client_connected_state */: if ((_harness_random % 2) == 0) {
            /* (tty_client_connected_state -> tty_client_connected_state) */
            
            registered_tty_driver->ops->write(&tty, content, sizeof(content));
            
            goto label3; /* tty_client_connected_state */
          } else {
            /* (tty_client_connected_state -> tty_client_disconnecting) */
            
            registered_tty_driver->ops->close(&tty, &file);
            
            label4 /* tty_client_disconnecting */: if ((_harness_random % 3) == 0) {
              /* (tty_client_disconnecting -> tty_client_disconnecting) */
              goto label4; /* tty_client_disconnecting */
            } else if ((_harness_random % 3) == 0) {
              __harness_mutex_unlock(&mutex6);
              /* (tty_client_disconnecting -> tty_client_disconnected) */
              goto label1; /* tty_client_disconnected */
            } else {
              __harness_mutex_lock(&mutex0);
              __harness_mutex_unlock(&mutex6);
              /* (tty_client_disconnecting -> tty_client_nodriver on tty_driver_unloading) */
              goto label0; /* tty_client_nodriver */
            }
          }
        } else {
          __harness_mutex_lock(&mutex0);
          __harness_mutex_unlock(&mutex8);
          /* (tty_client_wait_connection -> tty_client_nodriver on tty_driver_unloading) */
          goto label0; /* tty_client_nodriver */
        }
      } else {
        __harness_mutex_lock(&mutex0);
        /* (tty_client_disconnected -> tty_client_nodriver on tty_driver_unloading) */
        goto label0; /* tty_client_nodriver */
      }
    }
  }
  
  return NULL;
}

void *process_tty_client2(void *arg) {
  (void) arg; // Unused
  
  
      struct tty_struct tty;
      struct file file;
      const char content[] = "client1";
      
  
  {
    __harness_mutex_lock(&mutex5);
    process_tty_client2_init_barrier = 1;
    while (!process_tty_client1_init_barrier || !process_tty_client3_init_barrier || !process_tty_driver_init_barrier); // Wait for other processes
    label0 /* tty_client_nodriver */: if ((_harness_random % 2) == 0) {
      /* (tty_client_nodriver -> tty_client_nodriver) */
      goto label0; /* tty_client_nodriver */
    } else {
      __harness_mutex_unlock(&mutex5);
      /* (tty_client_nodriver -> tty_client_disconnected on tty_driver_loaded) */
      label1 /* tty_client_disconnected */: if ((_harness_random % 3) == 0) {
        /* (tty_client_disconnected -> tty_client_disconnected) */
        goto label1; /* tty_client_disconnected */
      } else if ((_harness_random % 3) == 0) {
        __harness_mutex_lock(&mutex1);
        /* (tty_client_disconnected -> tty_client_wait_connection) */
        label2 /* tty_client_wait_connection */: if ((_harness_random % 3) == 0) {
          /* (tty_client_wait_connection -> tty_client_wait_connection) */
          goto label2; /* tty_client_wait_connection */
        } else if ((_harness_random % 3) == 0) {
          __harness_mutex_lock(&mutex2);
          __harness_mutex_unlock(&mutex1);
          /* (tty_client_wait_connection -> tty_client_connected_state on tty_driver_grant_connection) */
          
          registered_tty_driver->ops->open(&tty, &file);
          
          label3 /* tty_client_connected_state */: if ((_harness_random % 2) == 0) {
            /* (tty_client_connected_state -> tty_client_connected_state) */
            
            registered_tty_driver->ops->write(&tty, content, sizeof(content));
            
            goto label3; /* tty_client_connected_state */
          } else {
            /* (tty_client_connected_state -> tty_client_disconnecting) */
            
            registered_tty_driver->ops->close(&tty, &file);
            
            label4 /* tty_client_disconnecting */: if ((_harness_random % 3) == 0) {
              /* (tty_client_disconnecting -> tty_client_disconnecting) */
              goto label4; /* tty_client_disconnecting */
            } else if ((_harness_random % 3) == 0) {
              __harness_mutex_unlock(&mutex2);
              /* (tty_client_disconnecting -> tty_client_disconnected) */
              goto label1; /* tty_client_disconnected */
            } else {
              __harness_mutex_lock(&mutex5);
              __harness_mutex_unlock(&mutex2);
              /* (tty_client_disconnecting -> tty_client_nodriver on tty_driver_unloading) */
              goto label0; /* tty_client_nodriver */
            }
          }
        } else {
          __harness_mutex_lock(&mutex5);
          __harness_mutex_unlock(&mutex1);
          /* (tty_client_wait_connection -> tty_client_nodriver on tty_driver_unloading) */
          goto label0; /* tty_client_nodriver */
        }
      } else {
        __harness_mutex_lock(&mutex5);
        /* (tty_client_disconnected -> tty_client_nodriver on tty_driver_unloading) */
        goto label0; /* tty_client_nodriver */
      }
    }
  }
  
  return NULL;
}

void *process_tty_client3(void *arg) {
  (void) arg; // Unused
  
  
      struct tty_struct tty;
      struct file file;
      const char content[] = "client2";
      
  
  {
    __harness_mutex_lock(&mutex7);
    process_tty_client3_init_barrier = 1;
    while (!process_tty_client1_init_barrier || !process_tty_client2_init_barrier || !process_tty_driver_init_barrier); // Wait for other processes
    label0 /* tty_client_nodriver */: if ((_harness_random % 2) == 0) {
      /* (tty_client_nodriver -> tty_client_nodriver) */
      goto label0; /* tty_client_nodriver */
    } else {
      __harness_mutex_unlock(&mutex7);
      /* (tty_client_nodriver -> tty_client_disconnected on tty_driver_loaded) */
      label1 /* tty_client_disconnected */: if ((_harness_random % 3) == 0) {
        /* (tty_client_disconnected -> tty_client_disconnected) */
        goto label1; /* tty_client_disconnected */
      } else if ((_harness_random % 3) == 0) {
        __harness_mutex_lock(&mutex3);
        /* (tty_client_disconnected -> tty_client_wait_connection) */
        label2 /* tty_client_wait_connection */: if ((_harness_random % 3) == 0) {
          /* (tty_client_wait_connection -> tty_client_wait_connection) */
          goto label2; /* tty_client_wait_connection */
        } else if ((_harness_random % 3) == 0) {
          __harness_mutex_lock(&mutex4);
          __harness_mutex_unlock(&mutex3);
          /* (tty_client_wait_connection -> tty_client_connected_state on tty_driver_grant_connection) */
          
          registered_tty_driver->ops->open(&tty, &file);
          
          label3 /* tty_client_connected_state */: if ((_harness_random % 2) == 0) {
            /* (tty_client_connected_state -> tty_client_connected_state) */
            
            registered_tty_driver->ops->write(&tty, content, sizeof(content));
            
            goto label3; /* tty_client_connected_state */
          } else {
            /* (tty_client_connected_state -> tty_client_disconnecting) */
            
            registered_tty_driver->ops->close(&tty, &file);
            
            label4 /* tty_client_disconnecting */: if ((_harness_random % 3) == 0) {
              /* (tty_client_disconnecting -> tty_client_disconnecting) */
              goto label4; /* tty_client_disconnecting */
            } else if ((_harness_random % 3) == 0) {
              __harness_mutex_unlock(&mutex4);
              /* (tty_client_disconnecting -> tty_client_disconnected) */
              goto label1; /* tty_client_disconnected */
            } else {
              __harness_mutex_lock(&mutex7);
              __harness_mutex_unlock(&mutex4);
              /* (tty_client_disconnecting -> tty_client_nodriver on tty_driver_unloading) */
              goto label0; /* tty_client_nodriver */
            }
          }
        } else {
          __harness_mutex_lock(&mutex7);
          __harness_mutex_unlock(&mutex3);
          /* (tty_client_wait_connection -> tty_client_nodriver on tty_driver_unloading) */
          goto label0; /* tty_client_nodriver */
        }
      } else {
        __harness_mutex_lock(&mutex7);
        /* (tty_client_disconnected -> tty_client_nodriver on tty_driver_unloading) */
        goto label0; /* tty_client_nodriver */
      }
    }
  }
  
  return NULL;
}

void *process_tty_driver(void *arg) {
  (void) arg; // Unused
  
  {
    __harness_mutex_lock(&mutex1);
    __harness_mutex_lock(&mutex2);
    __harness_mutex_lock(&mutex3);
    __harness_mutex_lock(&mutex4);
    __harness_mutex_lock(&mutex6);
    __harness_mutex_lock(&mutex8);
    process_tty_driver_init_barrier = 1;
    while (!process_tty_client1_init_barrier || !process_tty_client2_init_barrier || !process_tty_client3_init_barrier); // Wait for other processes
    label0 /* tty_driver_unloaded */: if ((_harness_random % 2) == 0) {
      /* (tty_driver_unloaded -> tty_driver_unloaded) */
      goto label0; /* tty_driver_unloaded */
    } else {
      /* (tty_driver_unloaded -> tty_driver_loading) */
      
      init_module();
      
      label1 /* tty_driver_loading */: if ((_harness_random % 2) == 0) {
        /* (tty_driver_loading -> tty_driver_loading) */
        goto label1; /* tty_driver_loading */
      } else {
        __harness_mutex_unlock(&mutex8);
        __harness_mutex_unlock(&mutex3);
        __harness_mutex_unlock(&mutex1);
        /* (tty_driver_loading -> tty_driver_loaded (tty_driver_client_inactive, tty_driver_client_inactive, tty_driver_client_inactive)) */
        label2 /* tty_driver_loaded (tty_driver_client_inactive, tty_driver_client_inactive, tty_driver_client_inactive) */: if ((_harness_random % 4) == 0) {
          __harness_mutex_lock(&mutex1);
          __harness_mutex_lock(&mutex3);
          __harness_mutex_lock(&mutex8);
          /* (tty_driver_loaded (tty_driver_client_inactive, tty_driver_client_inactive, tty_driver_client_inactive) -> tty_driver_unloading) */
          label3 /* tty_driver_unloading */: if ((_harness_random % 2) == 0) {
            /* (tty_driver_unloading -> tty_driver_unloading) */
            goto label3; /* tty_driver_unloading */
          } else {
            /* (tty_driver_unloading -> tty_driver_unloaded) */
            
            cleanup_module();
            
            goto label0; /* tty_driver_unloaded */
          }
        } else if ((_harness_random % 4) == 0) {
          __harness_mutex_lock(&mutex0);
          __harness_mutex_unlock(&mutex6);
          /* (tty_driver_loaded (tty_driver_client_inactive, tty_driver_client_inactive, tty_driver_client_inactive) -> tty_driver_loaded (tty_driver_client_active, tty_driver_client_inactive, tty_driver_client_inactive) on (tty_client_request_connection, _, _)) */
          label4 /* tty_driver_loaded (tty_driver_client_active, tty_driver_client_inactive, tty_driver_client_inactive) */: if ((_harness_random % 3) == 0) {
            __harness_mutex_lock(&mutex6);
            __harness_mutex_unlock(&mutex0);
            /* (tty_driver_loaded (tty_driver_client_active, tty_driver_client_inactive, tty_driver_client_inactive) -> tty_driver_loaded (tty_driver_client_inactive, tty_driver_client_inactive, tty_driver_client_inactive) on (tty_client_disconnect, _, _)) */
            goto label2; /* tty_driver_loaded (tty_driver_client_inactive, tty_driver_client_inactive, tty_driver_client_inactive) */
          } else if ((_harness_random % 3) == 0) {
            __harness_mutex_lock(&mutex5);
            __harness_mutex_unlock(&mutex2);
            /* (tty_driver_loaded (tty_driver_client_active, tty_driver_client_inactive, tty_driver_client_inactive) -> tty_driver_loaded (tty_driver_client_active, tty_driver_client_active, tty_driver_client_inactive) on (_, tty_client_request_connection, _)) */
            goto label5; /* tty_driver_loaded (tty_driver_client_active, tty_driver_client_active, tty_driver_client_inactive) */
          } else {
            __harness_mutex_lock(&mutex7);
            __harness_mutex_unlock(&mutex4);
            /* (tty_driver_loaded (tty_driver_client_active, tty_driver_client_inactive, tty_driver_client_inactive) -> tty_driver_loaded (tty_driver_client_active, tty_driver_client_inactive, tty_driver_client_active) on (_, _, tty_client_request_connection)) */
            goto label6; /* tty_driver_loaded (tty_driver_client_active, tty_driver_client_inactive, tty_driver_client_active) */
          }
        } else if ((_harness_random % 3) == 0) {
          __harness_mutex_lock(&mutex5);
          __harness_mutex_unlock(&mutex2);
          /* (tty_driver_loaded (tty_driver_client_inactive, tty_driver_client_inactive, tty_driver_client_inactive) -> tty_driver_loaded (tty_driver_client_inactive, tty_driver_client_active, tty_driver_client_inactive) on (_, tty_client_request_connection, _)) */
          label7 /* tty_driver_loaded (tty_driver_client_inactive, tty_driver_client_active, tty_driver_client_inactive) */: if ((_harness_random % 3) == 0) {
            __harness_mutex_lock(&mutex0);
            __harness_mutex_unlock(&mutex6);
            /* (tty_driver_loaded (tty_driver_client_inactive, tty_driver_client_active, tty_driver_client_inactive) -> tty_driver_loaded (tty_driver_client_active, tty_driver_client_active, tty_driver_client_inactive) on (tty_client_request_connection, _, _)) */
            label5 /* tty_driver_loaded (tty_driver_client_active, tty_driver_client_active, tty_driver_client_inactive) */: if ((_harness_random % 3) == 0) {
              __harness_mutex_lock(&mutex6);
              __harness_mutex_unlock(&mutex0);
              /* (tty_driver_loaded (tty_driver_client_active, tty_driver_client_active, tty_driver_client_inactive) -> tty_driver_loaded (tty_driver_client_inactive, tty_driver_client_active, tty_driver_client_inactive) on (tty_client_disconnect, _, _)) */
              goto label7; /* tty_driver_loaded (tty_driver_client_inactive, tty_driver_client_active, tty_driver_client_inactive) */
            } else if ((_harness_random % 3) == 0) {
              __harness_mutex_lock(&mutex2);
              __harness_mutex_unlock(&mutex5);
              /* (tty_driver_loaded (tty_driver_client_active, tty_driver_client_active, tty_driver_client_inactive) -> tty_driver_loaded (tty_driver_client_active, tty_driver_client_inactive, tty_driver_client_inactive) on (_, tty_client_disconnect, _)) */
              goto label4; /* tty_driver_loaded (tty_driver_client_active, tty_driver_client_inactive, tty_driver_client_inactive) */
            } else {
              __harness_mutex_lock(&mutex7);
              __harness_mutex_unlock(&mutex4);
              /* (tty_driver_loaded (tty_driver_client_active, tty_driver_client_active, tty_driver_client_inactive) -> tty_driver_loaded (tty_driver_client_active, tty_driver_client_active, tty_driver_client_active) on (_, _, tty_client_request_connection)) */
              label8 /* tty_driver_loaded (tty_driver_client_active, tty_driver_client_active, tty_driver_client_active) */: if ((_harness_random % 3) == 0) {
                __harness_mutex_lock(&mutex6);
                __harness_mutex_unlock(&mutex0);
                /* (tty_driver_loaded (tty_driver_client_active, tty_driver_client_active, tty_driver_client_active) -> tty_driver_loaded (tty_driver_client_inactive, tty_driver_client_active, tty_driver_client_active) on (tty_client_disconnect, _, _)) */
                goto label9; /* tty_driver_loaded (tty_driver_client_inactive, tty_driver_client_active, tty_driver_client_active) */
              } else if ((_harness_random % 3) == 0) {
                __harness_mutex_lock(&mutex2);
                __harness_mutex_unlock(&mutex5);
                /* (tty_driver_loaded (tty_driver_client_active, tty_driver_client_active, tty_driver_client_active) -> tty_driver_loaded (tty_driver_client_active, tty_driver_client_inactive, tty_driver_client_active) on (_, tty_client_disconnect, _)) */
                label6 /* tty_driver_loaded (tty_driver_client_active, tty_driver_client_inactive, tty_driver_client_active) */: if ((_harness_random % 3) == 0) {
                  __harness_mutex_lock(&mutex6);
                  __harness_mutex_unlock(&mutex0);
                  /* (tty_driver_loaded (tty_driver_client_active, tty_driver_client_inactive, tty_driver_client_active) -> tty_driver_loaded (tty_driver_client_inactive, tty_driver_client_inactive, tty_driver_client_active) on (tty_client_disconnect, _, _)) */
                  goto label10; /* tty_driver_loaded (tty_driver_client_inactive, tty_driver_client_inactive, tty_driver_client_active) */
                } else if ((_harness_random % 3) == 0) {
                  __harness_mutex_lock(&mutex5);
                  __harness_mutex_unlock(&mutex2);
                  /* (tty_driver_loaded (tty_driver_client_active, tty_driver_client_inactive, tty_driver_client_active) -> tty_driver_loaded (tty_driver_client_active, tty_driver_client_active, tty_driver_client_active) on (_, tty_client_request_connection, _)) */
                  goto label8; /* tty_driver_loaded (tty_driver_client_active, tty_driver_client_active, tty_driver_client_active) */
                } else {
                  __harness_mutex_lock(&mutex4);
                  __harness_mutex_unlock(&mutex7);
                  /* (tty_driver_loaded (tty_driver_client_active, tty_driver_client_inactive, tty_driver_client_active) -> tty_driver_loaded (tty_driver_client_active, tty_driver_client_inactive, tty_driver_client_inactive) on (_, _, tty_client_disconnect)) */
                  goto label4; /* tty_driver_loaded (tty_driver_client_active, tty_driver_client_inactive, tty_driver_client_inactive) */
                }
              } else {
                __harness_mutex_lock(&mutex4);
                __harness_mutex_unlock(&mutex7);
                /* (tty_driver_loaded (tty_driver_client_active, tty_driver_client_active, tty_driver_client_active) -> tty_driver_loaded (tty_driver_client_active, tty_driver_client_active, tty_driver_client_inactive) on (_, _, tty_client_disconnect)) */
                goto label5; /* tty_driver_loaded (tty_driver_client_active, tty_driver_client_active, tty_driver_client_inactive) */
              }
            }
          } else if ((_harness_random % 3) == 0) {
            __harness_mutex_lock(&mutex2);
            __harness_mutex_unlock(&mutex5);
            /* (tty_driver_loaded (tty_driver_client_inactive, tty_driver_client_active, tty_driver_client_inactive) -> tty_driver_loaded (tty_driver_client_inactive, tty_driver_client_inactive, tty_driver_client_inactive) on (_, tty_client_disconnect, _)) */
            goto label2; /* tty_driver_loaded (tty_driver_client_inactive, tty_driver_client_inactive, tty_driver_client_inactive) */
          } else {
            __harness_mutex_lock(&mutex7);
            __harness_mutex_unlock(&mutex4);
            /* (tty_driver_loaded (tty_driver_client_inactive, tty_driver_client_active, tty_driver_client_inactive) -> tty_driver_loaded (tty_driver_client_inactive, tty_driver_client_active, tty_driver_client_active) on (_, _, tty_client_request_connection)) */
            goto label9; /* tty_driver_loaded (tty_driver_client_inactive, tty_driver_client_active, tty_driver_client_active) */
          }
        } else {
          __harness_mutex_lock(&mutex7);
          __harness_mutex_unlock(&mutex4);
          /* (tty_driver_loaded (tty_driver_client_inactive, tty_driver_client_inactive, tty_driver_client_inactive) -> tty_driver_loaded (tty_driver_client_inactive, tty_driver_client_inactive, tty_driver_client_active) on (_, _, tty_client_request_connection)) */
          label10 /* tty_driver_loaded (tty_driver_client_inactive, tty_driver_client_inactive, tty_driver_client_active) */: if ((_harness_random % 3) == 0) {
            __harness_mutex_lock(&mutex0);
            __harness_mutex_unlock(&mutex6);
            /* (tty_driver_loaded (tty_driver_client_inactive, tty_driver_client_inactive, tty_driver_client_active) -> tty_driver_loaded (tty_driver_client_active, tty_driver_client_inactive, tty_driver_client_active) on (tty_client_request_connection, _, _)) */
            goto label6; /* tty_driver_loaded (tty_driver_client_active, tty_driver_client_inactive, tty_driver_client_active) */
          } else if ((_harness_random % 3) == 0) {
            __harness_mutex_lock(&mutex5);
            __harness_mutex_unlock(&mutex2);
            /* (tty_driver_loaded (tty_driver_client_inactive, tty_driver_client_inactive, tty_driver_client_active) -> tty_driver_loaded (tty_driver_client_inactive, tty_driver_client_active, tty_driver_client_active) on (_, tty_client_request_connection, _)) */
            label9 /* tty_driver_loaded (tty_driver_client_inactive, tty_driver_client_active, tty_driver_client_active) */: if ((_harness_random % 3) == 0) {
              __harness_mutex_lock(&mutex0);
              __harness_mutex_unlock(&mutex6);
              /* (tty_driver_loaded (tty_driver_client_inactive, tty_driver_client_active, tty_driver_client_active) -> tty_driver_loaded (tty_driver_client_active, tty_driver_client_active, tty_driver_client_active) on (tty_client_request_connection, _, _)) */
              goto label8; /* tty_driver_loaded (tty_driver_client_active, tty_driver_client_active, tty_driver_client_active) */
            } else if ((_harness_random % 3) == 0) {
              __harness_mutex_lock(&mutex2);
              __harness_mutex_unlock(&mutex5);
              /* (tty_driver_loaded (tty_driver_client_inactive, tty_driver_client_active, tty_driver_client_active) -> tty_driver_loaded (tty_driver_client_inactive, tty_driver_client_inactive, tty_driver_client_active) on (_, tty_client_disconnect, _)) */
              goto label10; /* tty_driver_loaded (tty_driver_client_inactive, tty_driver_client_inactive, tty_driver_client_active) */
            } else {
              __harness_mutex_lock(&mutex4);
              __harness_mutex_unlock(&mutex7);
              /* (tty_driver_loaded (tty_driver_client_inactive, tty_driver_client_active, tty_driver_client_active) -> tty_driver_loaded (tty_driver_client_inactive, tty_driver_client_active, tty_driver_client_inactive) on (_, _, tty_client_disconnect)) */
              goto label7; /* tty_driver_loaded (tty_driver_client_inactive, tty_driver_client_active, tty_driver_client_inactive) */
            }
          } else {
            __harness_mutex_lock(&mutex4);
            __harness_mutex_unlock(&mutex7);
            /* (tty_driver_loaded (tty_driver_client_inactive, tty_driver_client_inactive, tty_driver_client_active) -> tty_driver_loaded (tty_driver_client_inactive, tty_driver_client_inactive, tty_driver_client_inactive) on (_, _, tty_client_disconnect)) */
            goto label2; /* tty_driver_loaded (tty_driver_client_inactive, tty_driver_client_inactive, tty_driver_client_inactive) */
          }
        }
      }
    }
  }
  
  return NULL;
}


int main() {
  __harness_thread_t process_thread_tty_client1;
  __harness_thread_t process_thread_tty_client2;
  __harness_thread_t process_thread_tty_client3;
  __harness_thread_t process_thread_tty_driver;
  
  process_tty_client1_init_barrier = 0;
  process_tty_client2_init_barrier = 0;
  process_tty_client3_init_barrier = 0;
  process_tty_driver_init_barrier = 0;
  
  __harness_mutex_init(&mutex0, NULL);
  __harness_mutex_init(&mutex1, NULL);
  __harness_mutex_init(&mutex2, NULL);
  __harness_mutex_init(&mutex3, NULL);
  __harness_mutex_init(&mutex4, NULL);
  __harness_mutex_init(&mutex5, NULL);
  __harness_mutex_init(&mutex6, NULL);
  __harness_mutex_init(&mutex7, NULL);
  __harness_mutex_init(&mutex8, NULL);
  
  __harness_thread_create(&process_thread_tty_client1, NULL, process_tty_client1, NULL);
  __harness_thread_create(&process_thread_tty_client2, NULL, process_tty_client2, NULL);
  __harness_thread_create(&process_thread_tty_client3, NULL, process_tty_client3, NULL);
  __harness_thread_create(&process_thread_tty_driver, NULL, process_tty_driver, NULL);
  
  __harness_thread_join(process_thread_tty_client1, NULL);
  __harness_thread_join(process_thread_tty_client2, NULL);
  __harness_thread_join(process_thread_tty_client3, NULL);
  __harness_thread_join(process_thread_tty_driver, NULL);
  
  return 0;
}
