#include <stdlib.h>
#include <stdio.h>
#include <stdbool.h>
#include <pthread.h>

extern _Atomic int RANDOM;


struct S1 {
    _Atomic unsigned int connections;
    _Atomic unsigned int value; // Remove _Atomic for data race
};
                            
static struct S1 *s1_ptr = NULL;


static pthread_mutex_t process_set_start_barrier_mtx;
static unsigned long process_set_start_barrier_epoch;
static unsigned long process_set_start_barrier_waiting;
static unsigned long process_set_start_barrier_size;


static void process_set_start_barrier_wait() {
  pthread_mutex_lock(&process_set_start_barrier_mtx);
  const unsigned int epoch = process_set_start_barrier_epoch;
  process_set_start_barrier_waiting++;
  pthread_mutex_unlock(&process_set_start_barrier_mtx);

  for (int wait = 1; wait;) {
    pthread_mutex_lock(&process_set_start_barrier_mtx);
    if (epoch != process_set_start_barrier_epoch) {
      wait = 0;
    } else if (process_set_start_barrier_waiting == process_set_start_barrier_size) {
      process_set_start_barrier_epoch++;
      process_set_start_barrier_waiting = 0;
      wait = 0;
    }
    pthread_mutex_unlock(&process_set_start_barrier_mtx);
  }
}

/* tty_driver: tty_driver_loaded (tty_driver_client_inactive, tty_driver_client_inactive), tty_driver: tty_driver_loaded (tty_driver_client_inactive, tty_driver_client_active), tty_client1: tty_client_connected_state, tty_client1: tty_client_disconnecting, tty_driver: tty_driver_unloaded, tty_driver: tty_driver_unloading, tty_driver: tty_driver_loading */
static pthread_mutex_t mutex0;

/* tty_client2: tty_client_wait_connection, tty_driver: tty_driver_unloaded, tty_driver: tty_driver_unloading, tty_driver: tty_driver_loading */
static pthread_mutex_t mutex1;

/* tty_driver: tty_driver_loaded (tty_driver_client_active, tty_driver_client_active), tty_client2: tty_client_nodriver, tty_driver: tty_driver_loaded (tty_driver_client_inactive, tty_driver_client_active) */
static pthread_mutex_t mutex2;

/* tty_driver: tty_driver_unloaded, tty_driver: tty_driver_unloading, tty_client1: tty_client_wait_connection, tty_driver: tty_driver_loading */
static pthread_mutex_t mutex3;

/* tty_driver: tty_driver_loaded (tty_driver_client_active, tty_driver_client_active), tty_driver: tty_driver_loaded (tty_driver_client_active, tty_driver_client_inactive), tty_client1: tty_client_nodriver */
static pthread_mutex_t mutex4;

/* tty_client2: tty_client_disconnecting, tty_driver: tty_driver_loaded (tty_driver_client_inactive, tty_driver_client_inactive), tty_driver: tty_driver_loaded (tty_driver_client_active, tty_driver_client_inactive), tty_driver: tty_driver_unloaded, tty_driver: tty_driver_unloading, tty_driver: tty_driver_loading, tty_client2: tty_client_connected_state */
static pthread_mutex_t mutex5;


void *process_tty_client1(void *arg) {
  (void) arg; // Unused
  
  {
    pthread_mutex_lock(&mutex4);
    process_set_start_barrier_wait();
    label0 /* tty_client_nodriver */: if ((RANDOM % 2) == 0) {
      /* (tty_client_nodriver -> tty_client_nodriver) */
      goto label0; /* tty_client_nodriver */
    } else {
      pthread_mutex_unlock(&mutex4);
      /* (tty_client_nodriver -> tty_client_disconnected on tty_driver_loaded) */
      label1 /* tty_client_disconnected */: if ((RANDOM % 3) == 0) {
        /* (tty_client_disconnected -> tty_client_disconnected) */
        goto label1; /* tty_client_disconnected */
      } else if ((RANDOM % 3) == 0) {
        pthread_mutex_lock(&mutex3);
        /* (tty_client_disconnected -> tty_client_wait_connection) */
        label2 /* tty_client_wait_connection */: if ((RANDOM % 3) == 0) {
          /* (tty_client_wait_connection -> tty_client_wait_connection) */
          goto label2; /* tty_client_wait_connection */
        } else if ((RANDOM % 3) == 0) {
          pthread_mutex_lock(&mutex0);
          pthread_mutex_unlock(&mutex3);
          /* (tty_client_wait_connection -> tty_client_connected_state on tty_driver_grant_connection) */
          
          s1_ptr->connections++;
          printf("Client 0 connect\n");
          
          label3 /* tty_client_connected_state */: if ((RANDOM % 2) == 0) {
            /* (tty_client_connected_state -> tty_client_connected_state) */
            
            s1_ptr->value++;
            printf("Client 0 use\n");
            
            goto label3; /* tty_client_connected_state */
          } else {
            /* (tty_client_connected_state -> tty_client_disconnecting) */
            
            s1_ptr->connections--;
            printf("Client 0 disconnect\n");
            
            label4 /* tty_client_disconnecting */: if ((RANDOM % 3) == 0) {
              /* (tty_client_disconnecting -> tty_client_disconnecting) */
              goto label4; /* tty_client_disconnecting */
            } else if ((RANDOM % 3) == 0) {
              pthread_mutex_unlock(&mutex0);
              /* (tty_client_disconnecting -> tty_client_disconnected) */
              goto label1; /* tty_client_disconnected */
            } else {
              pthread_mutex_lock(&mutex4);
              pthread_mutex_unlock(&mutex0);
              /* (tty_client_disconnecting -> tty_client_nodriver on tty_driver_unloading) */
              goto label0; /* tty_client_nodriver */
            }
          }
        } else {
          pthread_mutex_lock(&mutex4);
          pthread_mutex_unlock(&mutex3);
          /* (tty_client_wait_connection -> tty_client_nodriver on tty_driver_unloading) */
          goto label0; /* tty_client_nodriver */
        }
      } else {
        pthread_mutex_lock(&mutex4);
        /* (tty_client_disconnected -> tty_client_nodriver on tty_driver_unloading) */
        goto label0; /* tty_client_nodriver */
      }
    }
  }
  
  return NULL;
}

void *process_tty_client2(void *arg) {
  (void) arg; // Unused
  
  {
    pthread_mutex_lock(&mutex2);
    process_set_start_barrier_wait();
    label0 /* tty_client_nodriver */: if ((RANDOM % 2) == 0) {
      /* (tty_client_nodriver -> tty_client_nodriver) */
      goto label0; /* tty_client_nodriver */
    } else {
      pthread_mutex_unlock(&mutex2);
      /* (tty_client_nodriver -> tty_client_disconnected on tty_driver_loaded) */
      label1 /* tty_client_disconnected */: if ((RANDOM % 3) == 0) {
        /* (tty_client_disconnected -> tty_client_disconnected) */
        goto label1; /* tty_client_disconnected */
      } else if ((RANDOM % 3) == 0) {
        pthread_mutex_lock(&mutex1);
        /* (tty_client_disconnected -> tty_client_wait_connection) */
        label2 /* tty_client_wait_connection */: if ((RANDOM % 3) == 0) {
          /* (tty_client_wait_connection -> tty_client_wait_connection) */
          goto label2; /* tty_client_wait_connection */
        } else if ((RANDOM % 3) == 0) {
          pthread_mutex_lock(&mutex5);
          pthread_mutex_unlock(&mutex1);
          /* (tty_client_wait_connection -> tty_client_connected_state on tty_driver_grant_connection) */
          
          s1_ptr->connections++;
          printf("Client 1 connect\n");
          
          label3 /* tty_client_connected_state */: if ((RANDOM % 2) == 0) {
            /* (tty_client_connected_state -> tty_client_connected_state) */
            
            s1_ptr->value++;
            printf("Client 1 use\n");
            
            goto label3; /* tty_client_connected_state */
          } else {
            /* (tty_client_connected_state -> tty_client_disconnecting) */
            
            s1_ptr->connections--;
            printf("Client 1 disconnect\n");
            
            label4 /* tty_client_disconnecting */: if ((RANDOM % 3) == 0) {
              /* (tty_client_disconnecting -> tty_client_disconnecting) */
              goto label4; /* tty_client_disconnecting */
            } else if ((RANDOM % 3) == 0) {
              pthread_mutex_unlock(&mutex5);
              /* (tty_client_disconnecting -> tty_client_disconnected) */
              goto label1; /* tty_client_disconnected */
            } else {
              pthread_mutex_lock(&mutex2);
              pthread_mutex_unlock(&mutex5);
              /* (tty_client_disconnecting -> tty_client_nodriver on tty_driver_unloading) */
              goto label0; /* tty_client_nodriver */
            }
          }
        } else {
          pthread_mutex_lock(&mutex2);
          pthread_mutex_unlock(&mutex1);
          /* (tty_client_wait_connection -> tty_client_nodriver on tty_driver_unloading) */
          goto label0; /* tty_client_nodriver */
        }
      } else {
        pthread_mutex_lock(&mutex2);
        /* (tty_client_disconnected -> tty_client_nodriver on tty_driver_unloading) */
        goto label0; /* tty_client_nodriver */
      }
    }
  }
  
  return NULL;
}

void *process_tty_driver(void *arg) {
  (void) arg; // Unused
  
  
  struct S1 s1_impl;
  
  
  {
    pthread_mutex_lock(&mutex0);
    pthread_mutex_lock(&mutex1);
    pthread_mutex_lock(&mutex3);
    pthread_mutex_lock(&mutex5);
    process_set_start_barrier_wait();
    label0 /* tty_driver_unloaded */: if ((RANDOM % 2) == 0) {
      /* (tty_driver_unloaded -> tty_driver_unloaded) */
      goto label0; /* tty_driver_unloaded */
    } else {
      /* (tty_driver_unloaded -> tty_driver_loading) */
      
      s1_impl.connections = 0;
      s1_impl.value = 0;
      s1_ptr = &s1_impl;
      printf("Driver load\n");
      
      label1 /* tty_driver_loading */: if ((RANDOM % 2) == 0) {
        /* (tty_driver_loading -> tty_driver_loading) */
        goto label1; /* tty_driver_loading */
      } else {
        pthread_mutex_unlock(&mutex3);
        pthread_mutex_unlock(&mutex1);
        /* (tty_driver_loading -> tty_driver_loaded (tty_driver_client_inactive, tty_driver_client_inactive)) */
        label2 /* tty_driver_loaded (tty_driver_client_inactive, tty_driver_client_inactive) */: if ((RANDOM % 3) == 0) {
          pthread_mutex_lock(&mutex1);
          pthread_mutex_lock(&mutex3);
          /* (tty_driver_loaded (tty_driver_client_inactive, tty_driver_client_inactive) -> tty_driver_unloading) */
          label3 /* tty_driver_unloading */: if ((RANDOM % 2) == 0) {
            /* (tty_driver_unloading -> tty_driver_unloading) */
            goto label3; /* tty_driver_unloading */
          } else {
            /* (tty_driver_unloading -> tty_driver_unloaded) */
            
            printf("Driver unload\n");
            s1_ptr = NULL;
            
            goto label0; /* tty_driver_unloaded */
          }
        } else if ((RANDOM % 3) == 0) {
          pthread_mutex_lock(&mutex4);
          pthread_mutex_unlock(&mutex0);
          /* (tty_driver_loaded (tty_driver_client_inactive, tty_driver_client_inactive) -> tty_driver_loaded (tty_driver_client_active, tty_driver_client_inactive) on (tty_client_request_connection, _)) */
          label4 /* tty_driver_loaded (tty_driver_client_active, tty_driver_client_inactive) */: if ((RANDOM % 2) == 0) {
            pthread_mutex_lock(&mutex0);
            pthread_mutex_unlock(&mutex4);
            /* (tty_driver_loaded (tty_driver_client_active, tty_driver_client_inactive) -> tty_driver_loaded (tty_driver_client_inactive, tty_driver_client_inactive) on (tty_client_disconnect, _)) */
            goto label2; /* tty_driver_loaded (tty_driver_client_inactive, tty_driver_client_inactive) */
          } else {
            pthread_mutex_lock(&mutex2);
            pthread_mutex_unlock(&mutex5);
            /* (tty_driver_loaded (tty_driver_client_active, tty_driver_client_inactive) -> tty_driver_loaded (tty_driver_client_active, tty_driver_client_active) on (_, tty_client_request_connection)) */
            goto label5; /* tty_driver_loaded (tty_driver_client_active, tty_driver_client_active) */
          }
        } else {
          pthread_mutex_lock(&mutex2);
          pthread_mutex_unlock(&mutex5);
          /* (tty_driver_loaded (tty_driver_client_inactive, tty_driver_client_inactive) -> tty_driver_loaded (tty_driver_client_inactive, tty_driver_client_active) on (_, tty_client_request_connection)) */
          label6 /* tty_driver_loaded (tty_driver_client_inactive, tty_driver_client_active) */: if ((RANDOM % 2) == 0) {
            pthread_mutex_lock(&mutex4);
            pthread_mutex_unlock(&mutex0);
            /* (tty_driver_loaded (tty_driver_client_inactive, tty_driver_client_active) -> tty_driver_loaded (tty_driver_client_active, tty_driver_client_active) on (tty_client_request_connection, _)) */
            label5 /* tty_driver_loaded (tty_driver_client_active, tty_driver_client_active) */: if ((RANDOM % 2) == 0) {
              pthread_mutex_lock(&mutex0);
              pthread_mutex_unlock(&mutex4);
              /* (tty_driver_loaded (tty_driver_client_active, tty_driver_client_active) -> tty_driver_loaded (tty_driver_client_inactive, tty_driver_client_active) on (tty_client_disconnect, _)) */
              goto label6; /* tty_driver_loaded (tty_driver_client_inactive, tty_driver_client_active) */
            } else {
              pthread_mutex_lock(&mutex5);
              pthread_mutex_unlock(&mutex2);
              /* (tty_driver_loaded (tty_driver_client_active, tty_driver_client_active) -> tty_driver_loaded (tty_driver_client_active, tty_driver_client_inactive) on (_, tty_client_disconnect)) */
              goto label4; /* tty_driver_loaded (tty_driver_client_active, tty_driver_client_inactive) */
            }
          } else {
            pthread_mutex_lock(&mutex5);
            pthread_mutex_unlock(&mutex2);
            /* (tty_driver_loaded (tty_driver_client_inactive, tty_driver_client_active) -> tty_driver_loaded (tty_driver_client_inactive, tty_driver_client_inactive) on (_, tty_client_disconnect)) */
            goto label2; /* tty_driver_loaded (tty_driver_client_inactive, tty_driver_client_inactive) */
          }
        }
      }
    }
  }
  
  return NULL;
}


int main() {
  pthread_t process_thread_tty_client1;
  pthread_t process_thread_tty_client2;
  pthread_t process_thread_tty_driver;
  
  pthread_mutex_init(&process_set_start_barrier_mtx, NULL);
  process_set_start_barrier_epoch = 0;
  process_set_start_barrier_waiting = 0;
  process_set_start_barrier_size = 3;
  pthread_mutex_init(&mutex0, NULL);
  pthread_mutex_init(&mutex1, NULL);
  pthread_mutex_init(&mutex2, NULL);
  pthread_mutex_init(&mutex3, NULL);
  pthread_mutex_init(&mutex4, NULL);
  pthread_mutex_init(&mutex5, NULL);
  
  pthread_create(&process_thread_tty_client1, NULL, process_tty_client1, NULL);
  pthread_create(&process_thread_tty_client2, NULL, process_tty_client2, NULL);
  pthread_create(&process_thread_tty_driver, NULL, process_tty_driver, NULL);
  
  pthread_join(process_thread_tty_client1, NULL);
  pthread_join(process_thread_tty_client2, NULL);
  pthread_join(process_thread_tty_driver, NULL);
  
  return 0;
}
