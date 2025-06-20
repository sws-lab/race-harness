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
#include "linux/device.h"
#include "linux/parport.h"
#include "linux/ctype.h"
#include "linux/poll.h"
#include "uapi/linux/major.h"
#include "uapi/linux/ppdev.h"

const struct file_operations *registered_chrdev;

// __invalid_size_argument_for_IOC [include/asm-generic/ioctl.h line 11 column 21]
unsigned int __invalid_size_argument_for_IOC;

// jiffies [include/linux/jiffies.h line 86 column 76]
unsigned long volatile __cacheline_aligned_in_smp __jiffy_arch_data jiffies;

// kmalloc_caches [include/linux/slab.h line 622 column 21]
kmem_buckets kmalloc_caches[NR_KMALLOC_TYPES];

// ____wrong_branch_error [include/linux/jump_label.h line 419 column 13]
bool ____wrong_branch_error(void) {
	return 1;
}

// __bad_size_call_parameter [include/linux/percpu-defs.h line 310 column 13]
void __bad_size_call_parameter(void) {}

// __cond_resched [include/linux/kernel.h line 67 column 12]
int __cond_resched(void) {
	return 0;
}

// __dynamic_dev_dbg [include/linux/dynamic_debug.h line 146 column 6]
void __dynamic_dev_dbg(struct _ddebug *descriptor, const struct device *dev,
		       const char *fmt, ...) {}

// __dynamic_pr_debug [include/linux/dynamic_debug.h line 141 column 6]
void __dynamic_pr_debug(struct _ddebug *descriptor, const char *fmt, ...) {}

// __init_waitqueue_head [include/linux/wait.h line 62 column 13]
void __init_waitqueue_head(struct wait_queue_head *wq_head, const char *name, struct lock_class_key *) {}

// __kcsan_check_access [include/linux/kcsan-checks.h line 37 column 6]
void __kcsan_check_access(const volatile void *ptr, size_t size, int type) {}

// __might_resched [include/linux/kernel.h line 88 column 13]
void __might_resched(const char *file, int line, unsigned int offsets) {}

// __parport_register_driver [include/linux/parport.h line 285 column 18]
int __must_check __parport_register_driver(struct parport_driver *,
					   struct module *,
					   const char *mod_name) {
    return 0;
}

// __register_chrdev [include/linux/fs.h line 2911 column 12]
int __register_chrdev(unsigned int major, unsigned int baseminor,
			     unsigned int count, const char *name,
			     const struct file_operations *fops) {
	registered_chrdev = fops;
    return 0;
}

// __unregister_chrdev [include/linux/fs.h line 2914 column 13]
void __unregister_chrdev(unsigned int major, unsigned int baseminor,
				unsigned int count, const char *name) {}

// __usecs_to_jiffies [include/linux/jiffies.h line 542 column 22]
unsigned long __usecs_to_jiffies(const unsigned int u) {
	return u;
}

// __wake_up [include/linux/wait.h line 210 column 5]
int __wake_up(struct wait_queue_head *wq_head, unsigned int mode, int nr, void *key) {
	return 1;
}

// _copy_from_user [include/linux/uaccess.h line 187 column 1]
unsigned long _copy_from_user(void *, const void __user *, unsigned long) {
	return 0;
}

// class_register [include/linux/device/class.h line 78 column 18]
int class_register(const struct class *class) {
	return 0;
}

// class_unregister [include/linux/device/class.h line 79 column 6]
void class_unregister(const struct class *class) {}

// compat_ptr_ioctl [include/linux/fs.h line 2045 column 13]
long compat_ptr_ioctl(struct file *file, unsigned int cmd,
					unsigned long arg) {
	return 0;
}

// device_create [include/linux/device.h line 1128 column 1]
struct device * device_create(const struct class *cls, struct device *parent, dev_t devt,
	      void *drvdata, const char *fmt, ...) {
    return __kmalloc(sizeof(struct device), GFP_KERNEL);
}

// device_destroy [include/linux/device.h line 1134 column 6]
void device_destroy(const struct class *cls, dev_t devt) {}

// ida_alloc_range [include/linux/idr.h line 257 column 5]
int ida_alloc_range(struct ida *, unsigned int min, unsigned int max, gfp_t) {
	return 0;
}

// ida_free [include/linux/idr.h line 258 column 6]
void ida_free(struct ida *, unsigned int id) {}

// jiffies_to_timespec64 [include/linux/jiffies.h line 592 column 13]
void jiffies_to_timespec64(const unsigned long jiffies,
				  struct timespec64 *value) {}

// jiffies_to_usecs [include/linux/jiffies.h line 438 column 21]
unsigned int jiffies_to_usecs(const unsigned long j) {
	return j;
}

// kasprintf [include/linux/sprintf.h line 16 column 31]
char *kasprintf(gfp_t gfp, const char *fmt, ...) {
	return __kmalloc(sizeof(char) * (__harness_rand() + 1), gfp);
}

// mutex_lock_nested [include/linux/mutex.h line 157 column 13]
void mutex_lock_nested(struct mutex *lock, unsigned int subclass) {
	_raw_spin_lock(lock);
}

// parport_claim_or_block [include/linux/parport.h line 378 column 12]
int parport_claim_or_block(struct pardevice *dev) {
	return 0;
}

// parport_find_number [include/linux/parport.h line 339 column 24]
struct parport *parport_find_number (int) {
	return __kmalloc(sizeof(struct parport), GFP_KERNEL);
}

// parport_negotiate [include/linux/parport.h line 446 column 12]
int parport_negotiate (struct parport *, int mode) {
	return 0;
}

// parport_put_port [include/linux/parport.h line 347 column 13]
void parport_put_port (struct parport *) {}

// parport_read [include/linux/parport.h line 448 column 16]
ssize_t parport_read (struct parport *, void *buf, size_t len) {
	return len;
}

// parport_register_dev_model [include/linux/parport.h line 363 column 1]
struct pardevice *
parport_register_dev_model(struct parport *port, const char *name,
			   const struct pardev_cb *par_dev_cb, int cnt) {
    return __kmalloc(sizeof(struct pardevice), GFP_KERNEL);
}

// parport_release [include/linux/parport.h line 388 column 13]
void parport_release(struct pardevice *dev) {}

// parport_set_timeout [include/linux/parport.h line 451 column 13]
long parport_set_timeout (struct pardevice *, long inactivity) {
	return 0;
}

// parport_unregister_device [include/linux/parport.h line 367 column 13]
void parport_unregister_device(struct pardevice *dev) {}

// parport_unregister_driver [include/linux/parport.h line 324 column 6]
void parport_unregister_driver(struct parport_driver *) {}

// parport_write [include/linux/parport.h line 447 column 16]
ssize_t parport_write (struct parport *, const void *buf, size_t len) {
	return len;
}
