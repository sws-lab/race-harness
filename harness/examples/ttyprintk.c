void *harness_kernel_module_process_tty_client1(void *harness_kernel_module_process_arg) {
  (void) harness_kernel_module_process_arg; // UNUSED
  struct tty_struct tty;
  struct file file;
  const char content[] = "tty_client1";
  unsigned long harness_kernel_module_process_state = 0;
  switch (harness_kernel_module_process_state) {{
    case 0: /* tty_client_nodriver */
      switch (RANDOM() % 2) {
        case 0:
          harness_kernel_module_process_state = 0; /* tty_client_nodriver */
          break;
        
        case 1:
          harness_kernel_module_process_state = 1; /* tty_client_disconnected */
          break;
        
      }
      break;
    
    case 1: /* tty_client_disconnected */
      switch (RANDOM() % 3) {
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
      break;
    
    case 2: /* tty_client_wait_connection */
      switch (RANDOM() % 3) {
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
      break;
    
    case 3: /* tty_client_connected_state */
      switch (RANDOM() % 2) {
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
      break;
    
  }
  return NULL;
}

void *harness_kernel_module_process_tty_client2(void *harness_kernel_module_process_arg) {
  (void) harness_kernel_module_process_arg; // UNUSED
  struct tty_struct tty;
  struct file file;
  const char content[] = "tty_client2";
  unsigned long harness_kernel_module_process_state = 0;
  switch (harness_kernel_module_process_state) {{
    case 0: /* tty_client_nodriver */
      switch (RANDOM() % 2) {
        case 0:
          harness_kernel_module_process_state = 0; /* tty_client_nodriver */
          break;
        
        case 1:
          harness_kernel_module_process_state = 1; /* tty_client_disconnected */
          break;
        
      }
      break;
    
    case 1: /* tty_client_disconnected */
      switch (RANDOM() % 3) {
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
      break;
    
    case 2: /* tty_client_wait_connection */
      switch (RANDOM() % 3) {
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
      break;
    
    case 3: /* tty_client_connected_state */
      switch (RANDOM() % 2) {
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
      break;
    
  }
  return NULL;
}

void *harness_kernel_module_process_tty_driver(void *harness_kernel_module_process_arg) {
  (void) harness_kernel_module_process_arg; // UNUSED
  unsigned long harness_kernel_module_process_state = 0;
  switch (harness_kernel_module_process_state) {{
    case 0: /* tty_driver_unloaded */
      switch (RANDOM() % 2) {
        case 0:
          harness_kernel_module_process_state = 0; /* tty_driver_unloaded */
          break;
        
        case 1:
          {
            init_module();
          }
          harness_kernel_module_process_state = 1; /* tty_driver_loaded (tty_driver_client_inactive, tty_driver_client_inactive) */
          break;
        
      }
      break;
    
    case 1: /* tty_driver_loaded (tty_driver_client_inactive, tty_driver_client_inactive) */
      switch (RANDOM() % 3) {
        case 0:
          {
            cleanup_module();
          }
          harness_kernel_module_process_state = 0; /* tty_driver_unloaded */
          break;
        
        case 1:
          harness_kernel_module_process_state = 4; /* tty_driver_loaded (tty_driver_client_active, tty_driver_client_inactive) */
          break;
        
        case 2:
          harness_kernel_module_process_state = 2; /* tty_driver_loaded (tty_driver_client_inactive, tty_driver_client_active) */
          break;
        
      }
      break;
    
    case 2: /* tty_driver_loaded (tty_driver_client_inactive, tty_driver_client_active) */
      switch (RANDOM() % 2) {
        case 0:
          harness_kernel_module_process_state = 3; /* tty_driver_loaded (tty_driver_client_active, tty_driver_client_active) */
          break;
        
        case 1:
          harness_kernel_module_process_state = 1; /* tty_driver_loaded (tty_driver_client_inactive, tty_driver_client_inactive) */
          break;
        
      }
      break;
    
    case 3: /* tty_driver_loaded (tty_driver_client_active, tty_driver_client_active) */
      switch (RANDOM() % 2) {
        case 0:
          harness_kernel_module_process_state = 2; /* tty_driver_loaded (tty_driver_client_inactive, tty_driver_client_active) */
          break;
        
        case 1:
          harness_kernel_module_process_state = 4; /* tty_driver_loaded (tty_driver_client_active, tty_driver_client_inactive) */
          break;
        
      }
      break;
    
    case 4: /* tty_driver_loaded (tty_driver_client_active, tty_driver_client_inactive) */
      switch (RANDOM() % 2) {
        case 0:
          harness_kernel_module_process_state = 1; /* tty_driver_loaded (tty_driver_client_inactive, tty_driver_client_inactive) */
          break;
        
        case 1:
          harness_kernel_module_process_state = 3; /* tty_driver_loaded (tty_driver_client_active, tty_driver_client_active) */
          break;
        
      }
      break;
    
  }
  return NULL;
}

int main(void) {
  harness_kernel_module_process_tty_client1(NULL);
  harness_kernel_module_process_tty_client2(NULL);
  harness_kernel_module_process_tty_driver(NULL);
  
  return 0;
}

