// This is a stub skeleton for Linux kernel module verification.
// The skeleton contains variable and function declarations external to the module.
// Please fill-in appropriate function code and variable values to obtain complete definitions
// for verifier to work on.
//
// Make sure that your stub file compiles correctly in the kernel build directory. Compiler command line and
// include list are preliminary, so feel free to edit the stub as needed.
//
// When added to stub_generator script as an input, complete stub along with respective kernel modules shall
// produce an empty skeleton.
//
// CLANG COMMAND LINE: -I. -Wp,-MMD,./..module-common.o.d -nostdinc -I./arch/x86/include -I./arch/x86/include/generated -I./include -I./include -I./arch/x86/include/uapi -I./arch/x86/include/generated/uapi -I./include/uapi -I./include/generated/uapi -include ./include/linux/compiler-version.h -include ./include/linux/kconfig.h -include ./include/linux/compiler_types.h -D__KERNEL__ --target=x86_64-linux-gnu -fintegrated-as -Werror=unknown-warning-option -Werror=ignored-optimization-argument -Werror=option-ignored -Werror=unused-command-line-argument -Werror -std=gnu11 -fshort-wchar -funsigned-char -fno-common -fno-PIE -fno-strict-aliasing -mno-sse -mno-mmx -mno-sse2 -mno-3dnow -mno-avx -fcf-protection=branch -fno-jump-tables -m64 -falign-loops=1 -mno-80387 -mno-fp-ret-in-387 -mstack-alignment=8 -mskip-rax-setup -mtune=generic -mno-red-zone -mcmodel=kernel -Wno-sign-compare -fno-asynchronous-unwind-tables -mretpoline-external-thunk -mindirect-branch-cs-prefix -mfunction-return=thunk-extern -mharden-sls=all -fpatchable-function-entry=59,59 -fno-delete-null-pointer-checks -O2 -fstack-protector-strong -ftrivial-auto-var-init=pattern -fno-stack-clash-protection -fzero-call-used-regs=used-gpr -pg -mfentry -DCC_USING_NOP_MCOUNT -DCC_USING_FENTRY -falign-functions=64 -fstrict-flex-arrays=3 -fno-strict-overflow -fno-stack-check -Wall -Wundef -Werror=implicit-function-declaration -Werror=implicit-int -Werror=return-type -Werror=strict-prototypes -Wno-format-security -Wno-trigraphs -Wno-frame-address -Wno-address-of-packed-member -Wmissing-declarations -Wmissing-prototypes -Wframe-larger-than=2048 -Wno-gnu -Wno-format-overflow-non-kprintf -Wno-format-truncation-non-kprintf -Wvla -Wno-pointer-sign -Wcast-function-type -Wimplicit-fallthrough -Werror=date-time -Werror=incompatible-pointer-types -Wenum-conversion -Wextra -Wunused -Wno-unused-but-set-variable -Wno-unused-const-variable -Wno-format-overflow -Wno-override-init -Wno-pointer-to-enum-cast -Wno-tautological-constant-out-of-range-compare -Wno-unaligned-access -Wno-enum-compare-conditional -Wno-missing-field-initializers -Wno-type-limits -Wno-shift-negative-value -Wno-enum-enum-conversion -Wno-sign-compare -Wno-unused-parameter -DRANDSTRUCT -frandomize-layout-seed-file=./scripts/basic/randstruct.seed -fsanitize=array-bounds -fsanitize=shift -fsanitize=bool -fsanitize=enum -fsanitize-coverage=trace-pc -fsanitize-coverage=trace-cmp -fsanitize=thread -fno-optimize-sibling-calls -mllvm -tsan-compound-read-before-write=1 -mllvm -tsan-distinguish-volatile=1 -fdebug-info-for-profiling -mllvm -enable-fs-discriminator=true -mllvm -improved-fs-discriminator=true -gmlt -fbasic-block-sections=labels -DMODULE '-DKBUILD_BASENAME=".module_common"' '-DKBUILD_MODNAME=".module_common.o"' -D__KBUILD_MODNAME=kmod_.module_common.o
#include "linux/kconfig.h"
#include "linux/compiler_types.h"
#include "asm/orc_header.h"
#include "linux/build-salt.h"
#include "linux/elfnote-lto.h"
#include "linux/export-internal.h"
#include "linux/module.h"
#include "linux/utsname.h"
#include "uapi/linux/major.h"
#include "linux/poll.h"
#include "linux/blkdev.h"
#include "linux/interrupt.h"
#include "linux/kthread.h"
#include "linux/ptrace.h"
#include "linux/irq.h"
#include "linux/syscalls.h"
#include "linux/suspend.h"
#include "linux/siphash.h"
#include "linux/sched/isolation.h"
#include "crypto/chacha.h"
#include "crypto/blake2s.h"
#include "vdso/getrandom.h"
#include "vdso/datapage.h"
#include "vdso/vsyscall.h"
#include "asm/archrandom.h"
#include "generated/asm/irq_regs.h"

// __cpu_online_mask [include/linux/cpumask.h line 116 column 23]
struct cpumask __cpu_online_mask = {0};

// __invalid_size_argument_for_IOC [include/asm-generic/ioctl.h line 11 column 21]
unsigned int __invalid_size_argument_for_IOC = 0;

// __per_cpu_offset [include/asm-generic/percpu.h line 19 column 22]
unsigned long __per_cpu_offset[NR_CPUS] = {0};

// boot_cpu_data [arch/x86/include/asm/processor.h line 214 column 27]
struct cpuinfo_x86	boot_cpu_data = {0};

// current_stack_pointer [arch/x86/include/asm/asm.h line 222 column 24]
register unsigned long current_stack_pointer = 0;

// debug_locks_silent [include/linux/debug_locks.h line 11 column 12]
int debug_locks_silent = 0;

// init_uts_ns [include/linux/utsname.h line 30 column 29]
struct uts_namespace init_uts_ns = {0};

// jiffies [include/linux/jiffies.h line 86 column 76]
unsigned long volatile __cacheline_aligned_in_smp __jiffy_arch_data jiffies = 0;

// kmalloc_caches [include/linux/slab.h line 622 column 21]
kmem_buckets kmalloc_caches[NR_KMALLOC_TYPES];

// nr_cpu_ids [include/linux/cpumask.h line 30 column 21]
unsigned int nr_cpu_ids = 0;

// oops_in_progress [include/linux/printk.h line 17 column 12]
int oops_in_progress = 0;

// pv_ops [arch/x86/include/asm/paravirt_types.h line 240 column 39]
struct paravirt_patch_template pv_ops = {0};

// static_key_initialized [include/linux/jump_label.h line 80 column 13]
bool static_key_initialized = 0;

// system_unbound_wq [include/linux/workqueue.h line 461 column 33]
struct workqueue_struct *system_unbound_wq = 0;

// vdso_data [arch/x86/include/asm/vdso/vsyscall.h line 18 column 26]
struct vdso_data *vdso_data = 0;

// ____wrong_branch_error [include/linux/jump_label.h line 419 column 13]
bool ____wrong_branch_error(void) {
	__goblint_assert(0);
}

// ___ratelimit [include/linux/ratelimit_types.h line 44 column 12]
int ___ratelimit(struct ratelimit_state *rs, const char *func) {
	return 0;
}

// __bad_size_call_parameter [include/linux/percpu-defs.h line 310 column 13]
void __bad_size_call_parameter(void) {
	__goblint_assert(0);
}

// __bitmap_and [include/linux/bitmap.h line 164 column 6]
bool __bitmap_and(unsigned long *dst, const unsigned long *bitmap1,
		 const unsigned long *bitmap2, unsigned int nbits) {
    return 0;
}

// __bitmap_weight [include/linux/bitmap.h line 179 column 14]
unsigned int __bitmap_weight(const unsigned long *bitmap, unsigned int nbits) {
	return 0;
}

// __cmpxchg_wrong_size [arch/x86/include/asm/cmpxchg.h line 15 column 13]
void __cmpxchg_wrong_size(void) {
	__goblint_assert(0);
}

// __cond_resched [include/linux/kernel.h line 67 column 12]
int __cond_resched(void) {
	return 0;
}

// __kcsan_check_access [include/linux/kcsan-checks.h line 37 column 6]
void __kcsan_check_access(const volatile void *ptr, size_t size, int type) {}

// __kmalloc [/home/jprotopopov/goblint-linux-kernel/examples/generic-stubs.c line 97 column 9]
;

// __might_fault [include/linux/kernel.h line 161 column 6]
void __might_fault(const char *file, int line) {}

// __might_resched [include/linux/kernel.h line 88 column 13]
void __might_resched(const char *file, int line, unsigned int offsets) {}

// __might_sleep [include/linux/kernel.h line 89 column 13]
void __might_sleep(const char *file, int line) {}

// __register_sysctl_init [include/linux/sysctl.h line 230 column 13]
void __register_sysctl_init(const char *path, const struct ctl_table *table,
				 const char *table_name, size_t table_size) {}

// __this_cpu_preempt_check [include/linux/percpu-defs.h line 313 column 13]
void __this_cpu_preempt_check(const char *op) {}

// __wake_up [include/linux/wait.h line 210 column 5]
int __wake_up(struct wait_queue_head *wq_head, unsigned int mode, int nr, void *key) {
	return 1;
}

// __warn_printk [include/asm-generic/bug.h line 93 column 28]
void __warn_printk(const char *fmt, ...) {}

// __xadd_wrong_size [arch/x86/include/asm/cmpxchg.h line 17 column 13]
void __xadd_wrong_size(void) {}

// _copy_from_iter [include/linux/uio.h line 192 column 8]
size_t _copy_from_iter(void *addr, size_t bytes, struct iov_iter *i) {
	return __harness_rand();
}

// _copy_to_iter [include/linux/uio.h line 191 column 8]
size_t _copy_to_iter(const void *addr, size_t bytes, struct iov_iter *i) {
	return __harness_rand();
}

// _find_first_bit [include/linux/find.h line 21 column 22]
unsigned long _find_first_bit(const unsigned long *addr, unsigned long size) {
	return __harness_rand() % size;
}

// _find_next_bit [include/linux/find.h line 11 column 15]
unsigned long _find_next_bit(const unsigned long *addr1, unsigned long nbits,
				unsigned long start) {
	return start + __harness_rand() % nbits;
}

// _printk_deferred [include/linux/printk.h line 164 column 27]
int _printk_deferred(const char *fmt, ...) {}

// add_timer_on [include/linux/timer.h line 150 column 13]
void add_timer_on(struct timer_list *timer, int cpu) {}

// atomic_notifier_call_chain [include/linux/notifier.h line 169 column 12]
int atomic_notifier_call_chain(struct atomic_notifier_head *nh,
		unsigned long val, void *v) {
	return 0;
}

// blake2s_final [include/crypto/blake2s.h line 87 column 6]
void blake2s_final(struct blake2s_state *state, u8 *out) {}

// blake2s_update [include/crypto/blake2s.h line 86 column 6]
void blake2s_update(struct blake2s_state *state, const u8 *in, size_t inlen) {}

// blocking_notifier_call_chain [include/linux/notifier.h line 171 column 12]
int blocking_notifier_call_chain(struct blocking_notifier_head *nh,
		unsigned long val, void *v) {
	return 0;
}

// blocking_notifier_chain_register [include/linux/notifier.h line 148 column 12]
int blocking_notifier_chain_register(struct blocking_notifier_head *nh,
		struct notifier_block *nb) {
	return 0;
}

// blocking_notifier_chain_unregister [include/linux/notifier.h line 162 column 12]
int blocking_notifier_chain_unregister(struct blocking_notifier_head *nh,
		struct notifier_block *nb) {
	return 0;
}

// chacha_block_generic [include/crypto/chacha.h line 33 column 6]
void chacha_block_generic(u32 *state, u8 *stream, int nrounds) {}

// compat_ptr_ioctl [include/linux/fs.h line 2045 column 13]
long compat_ptr_ioctl(struct file *file, unsigned int cmd,
					unsigned long arg) {
	return 0;
}

// copy_splice_read [include/linux/fs.h line 3364 column 9]
ssize_t copy_splice_read(struct file *in, loff_t *ppos,
			 struct pipe_inode_info *pipe,
			 size_t len, unsigned int flags) {
    return __harness_rand() % (len + 1);
}

// debug_locks_off [include/linux/debug_locks.h line 22 column 12]
int debug_locks_off(void) {
	return 0;
}

// debug_smp_processor_id [include/linux/smp.h line 269 column 23]
unsigned int debug_smp_processor_id(void) {
	return 0;
}

// delayed_work_timer_fn [include/linux/workqueue_types.h line 14 column 6]
void delayed_work_timer_fn(struct timer_list *t) {}

// destroy_timer_on_stack [include/linux/timer.h line 127 column 13]
void destroy_timer_on_stack(struct timer_list *timer) {}

// fasync_helper [include/linux/fs.h line 1215 column 12]
int fasync_helper(int, struct file *, int, struct fasync_struct **) {
	return 0;
}

// finish_wait [include/linux/wait.h line 1196 column 6]
void finish_wait(struct wait_queue_head *wq_head, struct wait_queue_entry *wq_entry) {}

// generate_random_uuid [include/linux/uuid.h line 96 column 6]
void generate_random_uuid(unsigned char uuid[16]) {}

// housekeeping_cpumask [include/linux/sched/isolation.h line 30 column 30]
const struct cpumask *housekeeping_cpumask(enum hk_type type) {
	return &__cpu_online_mask;
}

// import_ubuf [include/linux/uio.h line 372 column 5]
int import_ubuf(int type, void __user *buf, size_t len, struct iov_iter *i) {
	return 0;
}

// init_timer_on_stack_key [include/linux/timer.h line 75 column 13]
void init_timer_on_stack_key(struct timer_list *timer,
				    void (*func)(struct timer_list *),
				    unsigned int flags, const char *name,
				    struct lock_class_key *key) {}

// init_wait_entry [include/linux/wait.h line 286 column 13]
void init_wait_entry(struct wait_queue_entry *wq_entry, int flags) {}

// iter_file_splice_write [include/linux/fs.h line 3367 column 16]
ssize_t iter_file_splice_write(struct pipe_inode_info *,
		struct file *, loff_t *, size_t, unsigned int) {
    return __harness_rand();
}

// kill_fasync [include/linux/fs.h line 1222 column 13]
void kill_fasync(struct fasync_struct **, int, int) {}

// kstrtobool [include/linux/kstrtox.h line 98 column 18]
int kstrtobool(const char *s, bool *res) {
	return 0;	
}

// kthread_should_stop [include/linux/kthread.h line 91 column 6]
bool kthread_should_stop(void) {
	return 0;
}

// ktime_get [include/linux/timekeeping.h line 73 column 16]
ktime_t ktime_get(void) {
	return __harness_rand();
}

// ktime_get_seconds [include/linux/timekeeping.h line 58 column 17]
time64_t ktime_get_seconds(void) {
	return __harness_rand();
}

// ktime_get_with_offset [include/linux/timekeeping.h line 74 column 16]
ktime_t ktime_get_with_offset(enum tk_offsets offs) {
	return __harness_rand();
}

// lock_acquire [include/linux/lockdep.h line 227 column 13]
void lock_acquire(struct lockdep_map *lock, unsigned int subclass,
			 int trylock, int read, int check,
			 struct lockdep_map *nest_lock, unsigned long ip) {
	_raw_spin_lock(lock);
}

// lock_release [include/linux/lockdep.h line 231 column 13]
void lock_release(struct lockdep_map *lock, unsigned long ip) {
	raw_spin_unlock(lock);
}

// noop_llseek [include/linux/fs.h line 3373 column 15]
loff_t noop_llseek(struct file *file, loff_t offset, int whence) {
	return __harness_rand();
}

// preempt_count_add [include/linux/preempt.h line 195 column 13]
extern void preempt_count_add(int val) {}

// preempt_count_sub [include/linux/preempt.h line 196 column 13]
extern void preempt_count_sub(int val) {}

// prepare_to_wait_event [include/linux/wait.h line 1195 column 6]
long prepare_to_wait_event(struct wait_queue_head *wq_head, struct wait_queue_entry *wq_entry, int state) {
	return 0;
}

// proc_dointvec [include/linux/sysctl.h line 70 column 5]
int proc_dointvec(const struct ctl_table *, int, void *, size_t *, loff_t *) {
	return 0;
}

// proc_dostring [include/linux/sysctl.h line 67 column 5]
int proc_dostring(const struct ctl_table *, int, void *, size_t *, loff_t *) {
	return 0;
}

// queue_delayed_work_on [include/linux/workqueue.h line 590 column 13]
bool queue_delayed_work_on(int cpu, struct workqueue_struct *wq,
			struct delayed_work *work, unsigned long delay) {
    return 1;	
}

// queue_work_on [include/linux/workqueue.h line 586 column 13]
bool queue_work_on(int cpu, struct workqueue_struct *wq,
			struct work_struct *work) {
    return 1;
}

// random_get_entropy_fallback [include/linux/timex.h line 65 column 15]
unsigned long random_get_entropy_fallback(void) {
	return __harness_rand();
}

// raw_notifier_chain_register [include/linux/notifier.h line 150 column 12]
int raw_notifier_chain_register(struct raw_notifier_head *nh,
		struct notifier_block *nb) {
	return 0;
}

// register_pm_notifier [include/linux/suspend.h line 439 column 12]
int register_pm_notifier(struct notifier_block *nb) {
	return 0;
}

// schedule [include/linux/sched.h line 319 column 17]
void schedule(void) {}

// schedule_timeout [include/linux/sched.h line 314 column 13]
long schedule_timeout(long timeout) {
	return __harness_rand();
}

// schedule_timeout_interruptible [include/linux/sched.h line 315 column 13]
long schedule_timeout_interruptible(long timeout) {
	return __harness_rand();
}

// static_key_enable [include/linux/jump_label.h line 235 column 13]
void static_key_enable(struct static_key *key) {}

// timer_delete_sync [include/linux/timer.h line 166 column 12]
int timer_delete_sync(struct timer_list *timer) {
	return 0;
}

// trace_hardirqs_off [include/linux/irqflags.h line 50 column 13]
void trace_hardirqs_off(void) {}

// trace_hardirqs_on [include/linux/irqflags.h line 49 column 13]
void trace_hardirqs_on(void) {}

// try_to_del_timer_sync [include/linux/timer.h line 165 column 12]
int try_to_del_timer_sync(struct timer_list *timer) {
	return 0;
}

// warn_bogus_irq_restore [include/linux/irqflags.h line 155 column 13]
void warn_bogus_irq_restore(void) {}

