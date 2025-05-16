#include <stdlib.h>
#include <stdio.h>
#include <inttypes.h>
#include <pthread.h>
// #include <unistd.h>

struct tty_driver {
  const struct tty_operations *ops;
};

struct tty_struct {
  void *driver_data;
};

struct tty_operations {
  void (*open)(struct tty_struct *tty);
  void (*write)(struct tty_struct *tty);
}; 

struct ttyprintk_port {
	pthread_mutex_t lock;
};

static pthread_mutex_t process_set_start_barrier_mtx;
static unsigned long process_set_start_barrier_epoch;
static unsigned long process_set_start_barrier_waiting;
static unsigned long process_set_start_barrier_size;

static pthread_mutex_t mutex1;
static pthread_mutex_t mutex2;

static struct tty_driver *ttyprintk_driver;
static struct ttyprintk_port tpk_port;

static void tpk_open(struct tty_struct *tty) {
  tty->driver_data = &tpk_port;
}

static void tpk_write(struct tty_struct *tty) {
	struct ttyprintk_port *tpkp = tty->driver_data;
	// if (tpkp != &tpk_port) abort();

	pthread_mutex_lock(&tpkp->lock);
  static int X = 0;
  X = (X + 1) & 0xff;
  printf("%d\n", X);
	pthread_mutex_unlock(&tpkp->lock);
}

static const struct tty_operations ttyprintk_ops = {
  .open = tpk_open,
	.write = tpk_write
};

static int ttyprintk_init(void) {
	pthread_mutex_init(&tpk_port.lock, NULL);

	static struct tty_driver drv;
	ttyprintk_driver = &drv;
	ttyprintk_driver->ops = &ttyprintk_ops;

	return 0;
}

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

static void *process_tty_client1(void *arg) {
  (void) arg; // Unused
  
  struct tty_struct tty;
  
  process_set_start_barrier_wait();
  for (;;) {
    pthread_mutex_lock(&mutex2);

    ttyprintk_driver->ops->open(&tty);
    for (int i = 0; i < 10; i++) {
      ttyprintk_driver->ops->write(&tty);
    }

    pthread_mutex_unlock(&mutex2);
  }
  
  return NULL;
}

static void *process_tty_client2(void *arg) {
  (void) arg; // Unused
  
  struct tty_struct tty;

  process_set_start_barrier_wait();
  for (;;) {
    pthread_mutex_lock(&mutex1);

    ttyprintk_driver->ops->open(&tty);
    for (int i = 0; i < 10; i++) {
      ttyprintk_driver->ops->write(&tty);
    }

    pthread_mutex_unlock(&mutex1);
  }
  
  return NULL;
}

static void *process_tty_driver(void *arg) {
  (void) arg; // Unused
  
  pthread_mutex_lock(&mutex1);
  pthread_mutex_lock(&mutex2);

  process_set_start_barrier_wait();
  // usleep(1000000);
  ttyprintk_init();
  
  pthread_mutex_unlock(&mutex2);
  pthread_mutex_unlock(&mutex1);
  
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
  pthread_mutex_init(&mutex1, NULL);
  pthread_mutex_init(&mutex2, NULL);

  pthread_create(&process_thread_tty_client1, NULL, process_tty_client1, NULL);
  pthread_create(&process_thread_tty_client2, NULL, process_tty_client2, NULL);
  pthread_create(&process_thread_tty_driver, NULL, process_tty_driver, NULL);
  
  pthread_join(process_thread_tty_driver, NULL);
  pthread_join(process_thread_tty_client1, NULL);
  pthread_join(process_thread_tty_client2, NULL);
  
  return 0;
}
