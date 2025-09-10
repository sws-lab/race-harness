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
#include "fs/fuse/fuse_i.h"
#include "fs/fuse/dev_uring_i.h"
#include "fs/fuse/fuse_dev_i.h"
#include "linux/io_uring/cmd.h"

// __cpu_possible_mask [include/linux/cpumask.h line 115 column 23]
struct cpumask __cpu_possible_mask;

// debug_locks [include/linux/debug_locks.h line 10 column 12]
int debug_locks;

// jiffies [include/linux/jiffies.h line 86 column 76]
unsigned long volatile __cacheline_aligned_in_smp __jiffy_arch_data jiffies;

// kmalloc_caches [include/linux/slab.h line 622 column 21]
kmem_buckets kmalloc_caches[NR_KMALLOC_TYPES];

// nr_cpu_ids [include/linux/cpumask.h line 30 column 21]
unsigned int nr_cpu_ids;

// system_wq [include/linux/workqueue.h line 458 column 33]
struct workqueue_struct *system_wq;

// ___ratelimit [include/linux/ratelimit_types.h line 44 column 12]
int ___ratelimit(struct ratelimit_state *rs, const char *func);

// __bad_size_call_parameter [include/linux/percpu-defs.h line 310 column 13]
void __bad_size_call_parameter(void) {}

// __bitmap_weight [include/linux/bitmap.h line 179 column 14]
unsigned int __bitmap_weight(const unsigned long *bitmap, unsigned int nbits) {
	return 0;
}

// __init_waitqueue_head [include/linux/wait.h line 62 column 13]
void __init_waitqueue_head(struct wait_queue_head *wq_head, const char *name, struct lock_class_key *) {}

// __init_work [include/linux/workqueue.h line 260 column 13]
void __init_work(struct work_struct *work, int onstack) {}

// __io_uring_cmd_do_in_task [include/linux/io_uring/cmd.h line 54 column 6]
void __io_uring_cmd_do_in_task(struct io_uring_cmd *ioucmd,
			    void (*task_work_cb)(struct io_uring_cmd *, unsigned),
			    unsigned flags) {}

// __kcsan_check_access [include/linux/kcsan-checks.h line 37 column 6]
void __kcsan_check_access(const volatile void *ptr, size_t size, int type) {}

// __list_add_valid_or_report [include/linux/list.h line 53 column 35]
bool __list_valid_slowpath __list_add_valid_or_report(struct list_head *new,
							     struct list_head *prev,
							     struct list_head *next) {
	return 0;
}

// __list_del_entry_valid_or_report [include/linux/list.h line 96 column 35]
bool __list_valid_slowpath __list_del_entry_valid_or_report(struct list_head *entry) {
	return 0;
}

// __wake_up [include/linux/wait.h line 210 column 5]
int __wake_up(struct wait_queue_head *wq_head, unsigned int mode, int nr, void *key) {
	return 0;
}

// __warn_printk [include/asm-generic/bug.h line 93 column 28]
void __warn_printk(const char *fmt, ...) {}

// __xadd_wrong_size [arch/x86/include/asm/cmpxchg.h line 17 column 13]
void __xadd_wrong_size(void) {}

// _copy_from_user [include/linux/uaccess.h line 187 column 1]
unsigned long _copy_from_user(void *, const void __user *, unsigned long len) {
	return len;
}

// delayed_work_timer_fn [include/linux/workqueue_types.h line 14 column 6]
void delayed_work_timer_fn(struct timer_list *t) {}

// fuse_copy_args [fs/fuse/fuse_dev_i.h line 56 column 5]
int fuse_copy_args(struct fuse_copy_state *cs, unsigned int numargs,
		   unsigned int argpages, struct fuse_arg *args,
		   int zeroing) {
    return 0;
}

// fuse_copy_init [fs/fuse/fuse_dev_i.h line 54 column 6]
void fuse_copy_init(struct fuse_copy_state *cs, int write,
			   struct iov_iter *iter) {}

// fuse_copy_out_args [fs/fuse/fuse_dev_i.h line 59 column 5]
int fuse_copy_out_args(struct fuse_copy_state *cs, struct fuse_args *args,
		       unsigned int nbytes) {
    return 0;
}

// fuse_dev_end_requests [fs/fuse/fuse_dev_i.h line 52 column 6]
void fuse_dev_end_requests(struct list_head *head) {}

// fuse_get_unique [fs/fuse/fuse_i.h line 1424 column 5]
u64 fuse_get_unique(struct fuse_iqueue *fiq) {
	return 0;
}

// fuse_pqueue_init [fs/fuse/fuse_i.h line 1253 column 6]
void fuse_pqueue_init(struct fuse_pqueue *fpq) {}

// fuse_remove_pending_req [fs/fuse/fuse_dev_i.h line 64 column 6]
bool fuse_remove_pending_req(struct fuse_req *req, spinlock_t *lock) {
	return 0;
}

// fuse_req_hash [fs/fuse/fuse_dev_i.h line 49 column 14]
unsigned int fuse_req_hash(u64 unique) {
	return 0;
}

// fuse_request_end [fs/fuse/fuse_i.h line 1216 column 6]
void fuse_request_end(struct fuse_req *req) {}

// fuse_request_find [fs/fuse/fuse_dev_i.h line 50 column 18]
struct fuse_req *fuse_request_find(struct fuse_pqueue *fpq, u64 unique) {
    return __kmalloc(sizeof(struct fuse_req), GFP_KERNEL);
}

// import_iovec [include/linux/uio.h line 366 column 9]
ssize_t import_iovec(int type, const struct iovec __user *uvec,
		 unsigned nr_segs, unsigned fast_segs, struct iovec **iovp,
		 struct iov_iter *i) {
    return 0;
}

// import_ubuf [include/linux/uio.h line 372 column 5]
int import_ubuf(int type, void __user *buf, size_t len, struct iov_iter *i) {
	return 0;
}

// init_timer_key [include/linux/timer.h line 70 column 6]
void init_timer_key(struct timer_list *timer,
		    void (*func)(struct timer_list *), unsigned int flags,
		    const char *name, struct lock_class_key *key) {}

// io_uring_cmd_done [include/linux/io_uring/cmd.h line 51 column 6]
void io_uring_cmd_done(struct io_uring_cmd *cmd, ssize_t ret, u64 res2,
			unsigned issue_flags) {}

// io_uring_cmd_mark_cancelable [include/linux/io_uring/cmd.h line 62 column 6]
void io_uring_cmd_mark_cancelable(struct io_uring_cmd *cmd,
		unsigned int issue_flags) {}

// lock_is_held_type [include/linux/lockdep.h line 245 column 12]
int lock_is_held_type(const struct lockdep_map *lock, int read) {
	return 0;
}

// lockdep_init_map_type [include/linux/lockdep.h line 128 column 13]
void lockdep_init_map_type(struct lockdep_map *lock, const char *name,
	struct lock_class_key *key, int subclass, u8 inner, u8 outer, u8 lock_type) {}

// queue_delayed_work_on [include/linux/workqueue.h line 590 column 13]
bool queue_delayed_work_on(int cpu, struct workqueue_struct *wq,
			struct delayed_work *work, unsigned long delay) {
    return 0;
}
