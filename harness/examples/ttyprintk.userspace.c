#include <stdlib.h>
#include <stdio.h>
#include <stdbool.h>
#include <pthread.h>


#include <stdlib.h>
#include <stdio.h>
#include <pthread.h>

struct S1 {
    _Atomic unsigned int connections;
    _Atomic unsigned int value;
};
                            
static struct S1 *s1_ptr;


static pthread_barrier_t harness_init_barrier;

/* tty_driver: tty_driver_loaded (tty_driver_client_active, tty_driver_client_inactive, tty_driver_client_inactive), tty_driver: tty_driver_loaded (tty_driver_client_inactive, tty_driver_client_inactive, tty_driver_client_inactive), tty_driver: tty_driver_loaded (tty_driver_client_inactive, tty_driver_client_inactive, tty_driver_client_active), tty_driver: tty_driver_loading, tty_driver: tty_driver_unloaded, tty_client2: tty_client_connected_state, tty_driver: tty_driver_unloading, tty_driver: tty_driver_loaded (tty_driver_client_active, tty_driver_client_inactive, tty_driver_client_active), tty_client2: tty_client_disconnecting */
static pthread_mutex_t mutex0;

/* tty_driver: tty_driver_loaded (tty_driver_client_inactive, tty_driver_client_active, tty_driver_client_active), tty_client3: tty_client_nodriver, tty_driver: tty_driver_loaded (tty_driver_client_active, tty_driver_client_inactive, tty_driver_client_active), tty_driver: tty_driver_loaded (tty_driver_client_inactive, tty_driver_client_inactive, tty_driver_client_active), tty_driver: tty_driver_loaded (tty_driver_client_active, tty_driver_client_active, tty_driver_client_active) */
static pthread_mutex_t mutex1;

/* tty_driver: tty_driver_loaded (tty_driver_client_inactive, tty_driver_client_active, tty_driver_client_active), tty_driver: tty_driver_loaded (tty_driver_client_active, tty_driver_client_active, tty_driver_client_inactive), tty_driver: tty_driver_loaded (tty_driver_client_inactive, tty_driver_client_active, tty_driver_client_inactive), tty_driver: tty_driver_loaded (tty_driver_client_active, tty_driver_client_active, tty_driver_client_active), tty_client2: tty_client_nodriver */
static pthread_mutex_t mutex2;

/* tty_driver: tty_driver_unloading, tty_client1: tty_client_wait_connection, tty_driver: tty_driver_loading, tty_driver: tty_driver_unloaded */
static pthread_mutex_t mutex3;

/* tty_driver: tty_driver_unloading, tty_client3: tty_client_wait_connection, tty_driver: tty_driver_loading, tty_driver: tty_driver_unloaded */
static pthread_mutex_t mutex4;

/* tty_driver: tty_driver_unloading, tty_client2: tty_client_wait_connection, tty_driver: tty_driver_loading, tty_driver: tty_driver_unloaded */
static pthread_mutex_t mutex5;

/* tty_driver: tty_driver_loaded (tty_driver_client_inactive, tty_driver_client_inactive, tty_driver_client_inactive), tty_client1: tty_client_connected_state, tty_driver: tty_driver_loaded (tty_driver_client_inactive, tty_driver_client_active, tty_driver_client_inactive), tty_driver: tty_driver_loaded (tty_driver_client_inactive, tty_driver_client_inactive, tty_driver_client_active), tty_driver: tty_driver_loading, tty_driver: tty_driver_unloaded, tty_driver: tty_driver_unloading, tty_driver: tty_driver_loaded (tty_driver_client_inactive, tty_driver_client_active, tty_driver_client_active), tty_client1: tty_client_disconnecting */
static pthread_mutex_t mutex6;

/* tty_driver: tty_driver_loaded (tty_driver_client_active, tty_driver_client_inactive, tty_driver_client_inactive), tty_client1: tty_client_nodriver, tty_driver: tty_driver_loaded (tty_driver_client_active, tty_driver_client_active, tty_driver_client_inactive), tty_driver: tty_driver_loaded (tty_driver_client_active, tty_driver_client_inactive, tty_driver_client_active), tty_driver: tty_driver_loaded (tty_driver_client_active, tty_driver_client_active, tty_driver_client_active) */
static pthread_mutex_t mutex7;

/* tty_driver: tty_driver_loaded (tty_driver_client_active, tty_driver_client_inactive, tty_driver_client_inactive), tty_driver: tty_driver_loaded (tty_driver_client_inactive, tty_driver_client_inactive, tty_driver_client_inactive), tty_driver: tty_driver_loaded (tty_driver_client_active, tty_driver_client_active, tty_driver_client_inactive), tty_driver: tty_driver_loaded (tty_driver_client_inactive, tty_driver_client_active, tty_driver_client_inactive), tty_client3: tty_client_disconnecting, tty_driver: tty_driver_loading, tty_driver: tty_driver_unloaded, tty_driver: tty_driver_unloading, tty_client3: tty_client_connected_state */
static pthread_mutex_t mutex8;


void *process_tty_client1(void *arg) {
  (void) arg; // Unused
  
  {
    pthread_mutex_lock(&mutex7);
    pthread_barrier_wait(&harness_init_barrier);
    label0 /* tty_client_nodriver */: if ((rand() % 2) == 0) {
      /* (tty_client_nodriver -> tty_client_nodriver) */
      goto label0; /* tty_client_nodriver */
    } else {
      pthread_mutex_unlock(&mutex7);
      /* (tty_client_nodriver -> tty_client_disconnected on tty_driver_loaded) */
      label1 /* tty_client_disconnected */: if ((rand() % 3) == 0) {
        /* (tty_client_disconnected -> tty_client_disconnected) */
        goto label1; /* tty_client_disconnected */
      } else if ((rand() % 3) == 0) {
        pthread_mutex_lock(&mutex3);
        /* (tty_client_disconnected -> tty_client_wait_connection) */
        label2 /* tty_client_wait_connection */: if ((rand() % 3) == 0) {
          /* (tty_client_wait_connection -> tty_client_wait_connection) */
          goto label2; /* tty_client_wait_connection */
        } else if ((rand() % 3) == 0) {
          pthread_mutex_unlock(&mutex3);
          pthread_mutex_lock(&mutex6);
          /* (tty_client_wait_connection -> tty_client_connected_state on tty_driver_grant_connection) */
          
          s1_ptr->connections++;
          printf("Client 0 connected\n");
          
          label3 /* tty_client_connected_state */: if ((rand() % 2) == 0) {
            /* (tty_client_connected_state -> tty_client_connected_state) */
            
            s1_ptr->value++;
            printf("Client 0 active\n");
            
            goto label3; /* tty_client_connected_state */
          } else {
            /* (tty_client_connected_state -> tty_client_disconnecting) */
            
            s1_ptr->connections--;
            printf("Client 0 disconnected\n");
            
            label4 /* tty_client_disconnecting */: if ((rand() % 3) == 0) {
              /* (tty_client_disconnecting -> tty_client_disconnecting) */
              goto label4; /* tty_client_disconnecting */
            } else if ((rand() % 3) == 0) {
              pthread_mutex_unlock(&mutex6);
              /* (tty_client_disconnecting -> tty_client_disconnected) */
              goto label1; /* tty_client_disconnected */
            } else {
              pthread_mutex_unlock(&mutex6);
              pthread_mutex_lock(&mutex7);
              /* (tty_client_disconnecting -> tty_client_nodriver on tty_driver_unloading) */
              goto label0; /* tty_client_nodriver */
            }
          }
        } else {
          pthread_mutex_unlock(&mutex3);
          pthread_mutex_lock(&mutex7);
          /* (tty_client_wait_connection -> tty_client_nodriver on tty_driver_unloading) */
          goto label0; /* tty_client_nodriver */
        }
      } else {
        pthread_mutex_lock(&mutex7);
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
    pthread_barrier_wait(&harness_init_barrier);
    label0 /* tty_client_nodriver */: if ((rand() % 2) == 0) {
      /* (tty_client_nodriver -> tty_client_nodriver) */
      goto label0; /* tty_client_nodriver */
    } else {
      pthread_mutex_unlock(&mutex2);
      /* (tty_client_nodriver -> tty_client_disconnected on tty_driver_loaded) */
      label1 /* tty_client_disconnected */: if ((rand() % 3) == 0) {
        /* (tty_client_disconnected -> tty_client_disconnected) */
        goto label1; /* tty_client_disconnected */
      } else if ((rand() % 3) == 0) {
        pthread_mutex_lock(&mutex5);
        /* (tty_client_disconnected -> tty_client_wait_connection) */
        label2 /* tty_client_wait_connection */: if ((rand() % 3) == 0) {
          /* (tty_client_wait_connection -> tty_client_wait_connection) */
          goto label2; /* tty_client_wait_connection */
        } else if ((rand() % 3) == 0) {
          pthread_mutex_unlock(&mutex5);
          pthread_mutex_lock(&mutex0);
          /* (tty_client_wait_connection -> tty_client_connected_state on tty_driver_grant_connection) */
          
          s1_ptr->connections++;
          printf("Client 1 connected\n");
          
          label3 /* tty_client_connected_state */: if ((rand() % 2) == 0) {
            /* (tty_client_connected_state -> tty_client_connected_state) */
            
            s1_ptr->value++;
            printf("Client 1 active\n");
            
            goto label3; /* tty_client_connected_state */
          } else {
            /* (tty_client_connected_state -> tty_client_disconnecting) */
            
            s1_ptr->connections--;
            printf("Client 1 disconnected\n");
            
            label4 /* tty_client_disconnecting */: if ((rand() % 3) == 0) {
              /* (tty_client_disconnecting -> tty_client_disconnecting) */
              goto label4; /* tty_client_disconnecting */
            } else if ((rand() % 3) == 0) {
              pthread_mutex_unlock(&mutex0);
              /* (tty_client_disconnecting -> tty_client_disconnected) */
              goto label1; /* tty_client_disconnected */
            } else {
              pthread_mutex_unlock(&mutex0);
              pthread_mutex_lock(&mutex2);
              /* (tty_client_disconnecting -> tty_client_nodriver on tty_driver_unloading) */
              goto label0; /* tty_client_nodriver */
            }
          }
        } else {
          pthread_mutex_unlock(&mutex5);
          pthread_mutex_lock(&mutex2);
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

void *process_tty_client3(void *arg) {
  (void) arg; // Unused
  
  {
    pthread_mutex_lock(&mutex1);
    pthread_barrier_wait(&harness_init_barrier);
    label0 /* tty_client_nodriver */: if ((rand() % 2) == 0) {
      /* (tty_client_nodriver -> tty_client_nodriver) */
      goto label0; /* tty_client_nodriver */
    } else {
      pthread_mutex_unlock(&mutex1);
      /* (tty_client_nodriver -> tty_client_disconnected on tty_driver_loaded) */
      label1 /* tty_client_disconnected */: if ((rand() % 3) == 0) {
        /* (tty_client_disconnected -> tty_client_disconnected) */
        goto label1; /* tty_client_disconnected */
      } else if ((rand() % 3) == 0) {
        pthread_mutex_lock(&mutex4);
        /* (tty_client_disconnected -> tty_client_wait_connection) */
        label2 /* tty_client_wait_connection */: if ((rand() % 3) == 0) {
          /* (tty_client_wait_connection -> tty_client_wait_connection) */
          goto label2; /* tty_client_wait_connection */
        } else if ((rand() % 3) == 0) {
          pthread_mutex_unlock(&mutex4);
          pthread_mutex_lock(&mutex8);
          /* (tty_client_wait_connection -> tty_client_connected_state on tty_driver_grant_connection) */
          
          s1_ptr->connections++;
          printf("Client 2 connected\n");
          
          label3 /* tty_client_connected_state */: if ((rand() % 2) == 0) {
            /* (tty_client_connected_state -> tty_client_connected_state) */
            
            s1_ptr->value++;
            printf("Client 2 active\n");
            
            goto label3; /* tty_client_connected_state */
          } else {
            /* (tty_client_connected_state -> tty_client_disconnecting) */
            
            s1_ptr->connections--;
            printf("Client 2 disconnected\n");
            
            label4 /* tty_client_disconnecting */: if ((rand() % 3) == 0) {
              /* (tty_client_disconnecting -> tty_client_disconnecting) */
              goto label4; /* tty_client_disconnecting */
            } else if ((rand() % 3) == 0) {
              pthread_mutex_unlock(&mutex8);
              /* (tty_client_disconnecting -> tty_client_disconnected) */
              goto label1; /* tty_client_disconnected */
            } else {
              pthread_mutex_unlock(&mutex8);
              pthread_mutex_lock(&mutex1);
              /* (tty_client_disconnecting -> tty_client_nodriver on tty_driver_unloading) */
              goto label0; /* tty_client_nodriver */
            }
          }
        } else {
          pthread_mutex_unlock(&mutex4);
          pthread_mutex_lock(&mutex1);
          /* (tty_client_wait_connection -> tty_client_nodriver on tty_driver_unloading) */
          goto label0; /* tty_client_nodriver */
        }
      } else {
        pthread_mutex_lock(&mutex1);
        /* (tty_client_disconnected -> tty_client_nodriver on tty_driver_unloading) */
        goto label0; /* tty_client_nodriver */
      }
    }
  }
  
  return NULL;
}

void *process_tty_driver(void *arg) {
  (void) arg; // Unused
  
  
  static struct S1 s1 = {0};
  
  
  {
    pthread_mutex_lock(&mutex0);
    pthread_mutex_lock(&mutex3);
    pthread_mutex_lock(&mutex4);
    pthread_mutex_lock(&mutex5);
    pthread_mutex_lock(&mutex6);
    pthread_mutex_lock(&mutex8);
    pthread_barrier_wait(&harness_init_barrier);
    label0 /* tty_driver_unloaded */: if ((rand() % 2) == 0) {
      /* (tty_driver_unloaded -> tty_driver_unloaded) */
      goto label0; /* tty_driver_unloaded */
    } else {
      /* (tty_driver_unloaded -> tty_driver_loading) */
      
      s1_ptr = &s1;
      s1_ptr->connections = 0;
      s1_ptr->value = 0;
      printf("Driver loaded\n");
      
      label1 /* tty_driver_loading */: if ((rand() % 2) == 0) {
        /* (tty_driver_loading -> tty_driver_loading) */
        goto label1; /* tty_driver_loading */
      } else {
        pthread_mutex_unlock(&mutex5);
        pthread_mutex_unlock(&mutex4);
        pthread_mutex_unlock(&mutex3);
        /* (tty_driver_loading -> tty_driver_loaded (tty_driver_client_inactive, tty_driver_client_inactive, tty_driver_client_inactive)) */
        label2 /* tty_driver_loaded (tty_driver_client_inactive, tty_driver_client_inactive, tty_driver_client_inactive) */: if ((rand() % 4) == 0) {
          pthread_mutex_lock(&mutex3);
          pthread_mutex_lock(&mutex4);
          pthread_mutex_lock(&mutex5);
          /* (tty_driver_loaded (tty_driver_client_inactive, tty_driver_client_inactive, tty_driver_client_inactive) -> tty_driver_unloading) */
          label3 /* tty_driver_unloading */: if ((rand() % 2) == 0) {
            /* (tty_driver_unloading -> tty_driver_unloading) */
            goto label3; /* tty_driver_unloading */
          } else {
            /* (tty_driver_unloading -> tty_driver_unloaded) */
            
            printf("Driver unloaded\n");
            s1_ptr = NULL;
            
            goto label0; /* tty_driver_unloaded */
          }
        } else if ((rand() % 4) == 0) {
          pthread_mutex_unlock(&mutex6);
          pthread_mutex_lock(&mutex7);
          /* (tty_driver_loaded (tty_driver_client_inactive, tty_driver_client_inactive, tty_driver_client_inactive) -> tty_driver_loaded (tty_driver_client_active, tty_driver_client_inactive, tty_driver_client_inactive) on (tty_client_request_connection, _, _)) */
          label4 /* tty_driver_loaded (tty_driver_client_active, tty_driver_client_inactive, tty_driver_client_inactive) */: if ((rand() % 3) == 0) {
            pthread_mutex_unlock(&mutex7);
            pthread_mutex_lock(&mutex6);
            /* (tty_driver_loaded (tty_driver_client_active, tty_driver_client_inactive, tty_driver_client_inactive) -> tty_driver_loaded (tty_driver_client_inactive, tty_driver_client_inactive, tty_driver_client_inactive) on (tty_client_disconnect, _, _)) */
            goto label2; /* tty_driver_loaded (tty_driver_client_inactive, tty_driver_client_inactive, tty_driver_client_inactive) */
          } else if ((rand() % 3) == 0) {
            pthread_mutex_unlock(&mutex0);
            pthread_mutex_lock(&mutex2);
            /* (tty_driver_loaded (tty_driver_client_active, tty_driver_client_inactive, tty_driver_client_inactive) -> tty_driver_loaded (tty_driver_client_active, tty_driver_client_active, tty_driver_client_inactive) on (_, tty_client_request_connection, _)) */
            goto label5; /* tty_driver_loaded (tty_driver_client_active, tty_driver_client_active, tty_driver_client_inactive) */
          } else {
            pthread_mutex_unlock(&mutex8);
            pthread_mutex_lock(&mutex1);
            /* (tty_driver_loaded (tty_driver_client_active, tty_driver_client_inactive, tty_driver_client_inactive) -> tty_driver_loaded (tty_driver_client_active, tty_driver_client_inactive, tty_driver_client_active) on (_, _, tty_client_request_connection)) */
            goto label6; /* tty_driver_loaded (tty_driver_client_active, tty_driver_client_inactive, tty_driver_client_active) */
          }
        } else if ((rand() % 3) == 0) {
          pthread_mutex_unlock(&mutex0);
          pthread_mutex_lock(&mutex2);
          /* (tty_driver_loaded (tty_driver_client_inactive, tty_driver_client_inactive, tty_driver_client_inactive) -> tty_driver_loaded (tty_driver_client_inactive, tty_driver_client_active, tty_driver_client_inactive) on (_, tty_client_request_connection, _)) */
          label7 /* tty_driver_loaded (tty_driver_client_inactive, tty_driver_client_active, tty_driver_client_inactive) */: if ((rand() % 3) == 0) {
            pthread_mutex_unlock(&mutex6);
            pthread_mutex_lock(&mutex7);
            /* (tty_driver_loaded (tty_driver_client_inactive, tty_driver_client_active, tty_driver_client_inactive) -> tty_driver_loaded (tty_driver_client_active, tty_driver_client_active, tty_driver_client_inactive) on (tty_client_request_connection, _, _)) */
            label5 /* tty_driver_loaded (tty_driver_client_active, tty_driver_client_active, tty_driver_client_inactive) */: if ((rand() % 3) == 0) {
              pthread_mutex_unlock(&mutex7);
              pthread_mutex_lock(&mutex6);
              /* (tty_driver_loaded (tty_driver_client_active, tty_driver_client_active, tty_driver_client_inactive) -> tty_driver_loaded (tty_driver_client_inactive, tty_driver_client_active, tty_driver_client_inactive) on (tty_client_disconnect, _, _)) */
              goto label7; /* tty_driver_loaded (tty_driver_client_inactive, tty_driver_client_active, tty_driver_client_inactive) */
            } else if ((rand() % 3) == 0) {
              pthread_mutex_unlock(&mutex2);
              pthread_mutex_lock(&mutex0);
              /* (tty_driver_loaded (tty_driver_client_active, tty_driver_client_active, tty_driver_client_inactive) -> tty_driver_loaded (tty_driver_client_active, tty_driver_client_inactive, tty_driver_client_inactive) on (_, tty_client_disconnect, _)) */
              goto label4; /* tty_driver_loaded (tty_driver_client_active, tty_driver_client_inactive, tty_driver_client_inactive) */
            } else {
              pthread_mutex_unlock(&mutex8);
              pthread_mutex_lock(&mutex1);
              /* (tty_driver_loaded (tty_driver_client_active, tty_driver_client_active, tty_driver_client_inactive) -> tty_driver_loaded (tty_driver_client_active, tty_driver_client_active, tty_driver_client_active) on (_, _, tty_client_request_connection)) */
              label8 /* tty_driver_loaded (tty_driver_client_active, tty_driver_client_active, tty_driver_client_active) */: if ((rand() % 3) == 0) {
                pthread_mutex_unlock(&mutex7);
                pthread_mutex_lock(&mutex6);
                /* (tty_driver_loaded (tty_driver_client_active, tty_driver_client_active, tty_driver_client_active) -> tty_driver_loaded (tty_driver_client_inactive, tty_driver_client_active, tty_driver_client_active) on (tty_client_disconnect, _, _)) */
                goto label9; /* tty_driver_loaded (tty_driver_client_inactive, tty_driver_client_active, tty_driver_client_active) */
              } else if ((rand() % 3) == 0) {
                pthread_mutex_unlock(&mutex2);
                pthread_mutex_lock(&mutex0);
                /* (tty_driver_loaded (tty_driver_client_active, tty_driver_client_active, tty_driver_client_active) -> tty_driver_loaded (tty_driver_client_active, tty_driver_client_inactive, tty_driver_client_active) on (_, tty_client_disconnect, _)) */
                label6 /* tty_driver_loaded (tty_driver_client_active, tty_driver_client_inactive, tty_driver_client_active) */: if ((rand() % 3) == 0) {
                  pthread_mutex_unlock(&mutex7);
                  pthread_mutex_lock(&mutex6);
                  /* (tty_driver_loaded (tty_driver_client_active, tty_driver_client_inactive, tty_driver_client_active) -> tty_driver_loaded (tty_driver_client_inactive, tty_driver_client_inactive, tty_driver_client_active) on (tty_client_disconnect, _, _)) */
                  goto label10; /* tty_driver_loaded (tty_driver_client_inactive, tty_driver_client_inactive, tty_driver_client_active) */
                } else if ((rand() % 3) == 0) {
                  pthread_mutex_unlock(&mutex0);
                  pthread_mutex_lock(&mutex2);
                  /* (tty_driver_loaded (tty_driver_client_active, tty_driver_client_inactive, tty_driver_client_active) -> tty_driver_loaded (tty_driver_client_active, tty_driver_client_active, tty_driver_client_active) on (_, tty_client_request_connection, _)) */
                  goto label8; /* tty_driver_loaded (tty_driver_client_active, tty_driver_client_active, tty_driver_client_active) */
                } else {
                  pthread_mutex_unlock(&mutex1);
                  pthread_mutex_lock(&mutex8);
                  /* (tty_driver_loaded (tty_driver_client_active, tty_driver_client_inactive, tty_driver_client_active) -> tty_driver_loaded (tty_driver_client_active, tty_driver_client_inactive, tty_driver_client_inactive) on (_, _, tty_client_disconnect)) */
                  goto label4; /* tty_driver_loaded (tty_driver_client_active, tty_driver_client_inactive, tty_driver_client_inactive) */
                }
              } else {
                pthread_mutex_unlock(&mutex1);
                pthread_mutex_lock(&mutex8);
                /* (tty_driver_loaded (tty_driver_client_active, tty_driver_client_active, tty_driver_client_active) -> tty_driver_loaded (tty_driver_client_active, tty_driver_client_active, tty_driver_client_inactive) on (_, _, tty_client_disconnect)) */
                goto label5; /* tty_driver_loaded (tty_driver_client_active, tty_driver_client_active, tty_driver_client_inactive) */
              }
            }
          } else if ((rand() % 3) == 0) {
            pthread_mutex_unlock(&mutex2);
            pthread_mutex_lock(&mutex0);
            /* (tty_driver_loaded (tty_driver_client_inactive, tty_driver_client_active, tty_driver_client_inactive) -> tty_driver_loaded (tty_driver_client_inactive, tty_driver_client_inactive, tty_driver_client_inactive) on (_, tty_client_disconnect, _)) */
            goto label2; /* tty_driver_loaded (tty_driver_client_inactive, tty_driver_client_inactive, tty_driver_client_inactive) */
          } else {
            pthread_mutex_unlock(&mutex8);
            pthread_mutex_lock(&mutex1);
            /* (tty_driver_loaded (tty_driver_client_inactive, tty_driver_client_active, tty_driver_client_inactive) -> tty_driver_loaded (tty_driver_client_inactive, tty_driver_client_active, tty_driver_client_active) on (_, _, tty_client_request_connection)) */
            goto label9; /* tty_driver_loaded (tty_driver_client_inactive, tty_driver_client_active, tty_driver_client_active) */
          }
        } else {
          pthread_mutex_unlock(&mutex8);
          pthread_mutex_lock(&mutex1);
          /* (tty_driver_loaded (tty_driver_client_inactive, tty_driver_client_inactive, tty_driver_client_inactive) -> tty_driver_loaded (tty_driver_client_inactive, tty_driver_client_inactive, tty_driver_client_active) on (_, _, tty_client_request_connection)) */
          label10 /* tty_driver_loaded (tty_driver_client_inactive, tty_driver_client_inactive, tty_driver_client_active) */: if ((rand() % 3) == 0) {
            pthread_mutex_unlock(&mutex6);
            pthread_mutex_lock(&mutex7);
            /* (tty_driver_loaded (tty_driver_client_inactive, tty_driver_client_inactive, tty_driver_client_active) -> tty_driver_loaded (tty_driver_client_active, tty_driver_client_inactive, tty_driver_client_active) on (tty_client_request_connection, _, _)) */
            goto label6; /* tty_driver_loaded (tty_driver_client_active, tty_driver_client_inactive, tty_driver_client_active) */
          } else if ((rand() % 3) == 0) {
            pthread_mutex_unlock(&mutex0);
            pthread_mutex_lock(&mutex2);
            /* (tty_driver_loaded (tty_driver_client_inactive, tty_driver_client_inactive, tty_driver_client_active) -> tty_driver_loaded (tty_driver_client_inactive, tty_driver_client_active, tty_driver_client_active) on (_, tty_client_request_connection, _)) */
            label9 /* tty_driver_loaded (tty_driver_client_inactive, tty_driver_client_active, tty_driver_client_active) */: if ((rand() % 3) == 0) {
              pthread_mutex_unlock(&mutex6);
              pthread_mutex_lock(&mutex7);
              /* (tty_driver_loaded (tty_driver_client_inactive, tty_driver_client_active, tty_driver_client_active) -> tty_driver_loaded (tty_driver_client_active, tty_driver_client_active, tty_driver_client_active) on (tty_client_request_connection, _, _)) */
              goto label8; /* tty_driver_loaded (tty_driver_client_active, tty_driver_client_active, tty_driver_client_active) */
            } else if ((rand() % 3) == 0) {
              pthread_mutex_unlock(&mutex2);
              pthread_mutex_lock(&mutex0);
              /* (tty_driver_loaded (tty_driver_client_inactive, tty_driver_client_active, tty_driver_client_active) -> tty_driver_loaded (tty_driver_client_inactive, tty_driver_client_inactive, tty_driver_client_active) on (_, tty_client_disconnect, _)) */
              goto label10; /* tty_driver_loaded (tty_driver_client_inactive, tty_driver_client_inactive, tty_driver_client_active) */
            } else {
              pthread_mutex_unlock(&mutex1);
              pthread_mutex_lock(&mutex8);
              /* (tty_driver_loaded (tty_driver_client_inactive, tty_driver_client_active, tty_driver_client_active) -> tty_driver_loaded (tty_driver_client_inactive, tty_driver_client_active, tty_driver_client_inactive) on (_, _, tty_client_disconnect)) */
              goto label7; /* tty_driver_loaded (tty_driver_client_inactive, tty_driver_client_active, tty_driver_client_inactive) */
            }
          } else {
            pthread_mutex_unlock(&mutex1);
            pthread_mutex_lock(&mutex8);
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
  pthread_t process_thread_tty_client1;
  pthread_t process_thread_tty_client2;
  pthread_t process_thread_tty_client3;
  pthread_t process_thread_tty_driver;
  
  pthread_barrier_init(&harness_init_barrier, NULL, 4);
  
  pthread_mutex_init(&mutex0, NULL);
  pthread_mutex_init(&mutex1, NULL);
  pthread_mutex_init(&mutex2, NULL);
  pthread_mutex_init(&mutex3, NULL);
  pthread_mutex_init(&mutex4, NULL);
  pthread_mutex_init(&mutex5, NULL);
  pthread_mutex_init(&mutex6, NULL);
  pthread_mutex_init(&mutex7, NULL);
  pthread_mutex_init(&mutex8, NULL);
  
  pthread_create(&process_thread_tty_client1, NULL, process_tty_client1, NULL);
  pthread_create(&process_thread_tty_client2, NULL, process_tty_client2, NULL);
  pthread_create(&process_thread_tty_client3, NULL, process_tty_client3, NULL);
  pthread_create(&process_thread_tty_driver, NULL, process_tty_driver, NULL);
  
  pthread_join(process_thread_tty_client1, NULL);
  pthread_join(process_thread_tty_client2, NULL);
  pthread_join(process_thread_tty_client3, NULL);
  pthread_join(process_thread_tty_driver, NULL);
  
  return 0;
}
