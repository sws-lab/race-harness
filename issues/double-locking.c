#include <pthread.h>

struct ttyprintk_port {
	pthread_mutex_t lock;
};

static struct ttyprintk_port tpk_port;
static struct ttyprintk_port *(*open)();

static struct ttyprintk_port *tpk_open() {
  return &tpk_port;
}

static void *process_tty_client1(void *arg) { 
  struct ttyprintk_port *tpkp = open();

  for (int i = 0; i < 1; i++) {
    pthread_mutex_lock(&tpkp->lock);
    pthread_mutex_unlock(&tpkp->lock);
  }

  return NULL;
}

static void *process_tty_driver(void *arg) {
	open = tpk_open;  
  return NULL;
}

int main() {
	pthread_mutex_init(&tpk_port.lock, NULL);

  pthread_t process_thread_tty_client1;
  pthread_t process_thread_tty_driver;

  // pthread_create(&process_thread_tty_driver, NULL, process_tty_driver, NULL);
  pthread_create(&process_thread_tty_client1, NULL, process_tty_client1, NULL);
  pthread_create(&process_thread_tty_driver, NULL, process_tty_driver, NULL);
  
  pthread_join(process_thread_tty_driver, NULL);
  pthread_join(process_thread_tty_client1, NULL);
  
  return 0;
}
