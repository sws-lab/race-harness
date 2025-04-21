// This is a stub skeleton for Linux kernel module verification.
// The skeleton contains variable and function declarations external to the module.
// Please fill-in appropriate function code and variable values to obtain complete definitions
// for verifier to work on.
//
// Make sure that your stub file compiles correctly in the kernel build directory. Compiler command line and
// include list are preliminary, feel free to edit those as needed.
//
// CLANG COMMAND LINE: -Wp,-MMD,./..module-common.o.d -nostdinc -I./arch/x86/include -I./arch/x86/include/generated -I./include -I./include -I./arch/x86/include/uapi -I./arch/x86/include/generated/uapi -I./include/uapi -I./include/generated/uapi -include ./include/linux/compiler-version.h -include ./include/linux/kconfig.h -include ./include/linux/compiler_types.h -D__KERNEL__ --target=x86_64-linux-gnu -fintegrated-as -Werror=unknown-warning-option -Werror=ignored-optimization-argument -Werror=option-ignored -Werror=unused-command-line-argument -Werror -std=gnu11 -fshort-wchar -funsigned-char -fno-common -fno-PIE -fno-strict-aliasing -mno-sse -mno-mmx -mno-sse2 -mno-3dnow -mno-avx -fcf-protection=branch -fno-jump-tables -m64 -falign-loops=1 -mno-80387 -mno-fp-ret-in-387 -mstack-alignment=8 -mskip-rax-setup -mtune=generic -mno-red-zone -mcmodel=kernel -Wno-sign-compare -fno-asynchronous-unwind-tables -mretpoline-external-thunk -mindirect-branch-cs-prefix -mfunction-return=thunk-extern -mharden-sls=all -fpatchable-function-entry=59,59 -fno-delete-null-pointer-checks -O2 -fstack-protector-strong -ftrivial-auto-var-init=pattern -fno-stack-clash-protection -fzero-call-used-regs=used-gpr -pg -mfentry -DCC_USING_NOP_MCOUNT -DCC_USING_FENTRY -falign-functions=64 -fstrict-flex-arrays=3 -fno-strict-overflow -fno-stack-check -Wall -Wundef -Werror=implicit-function-declaration -Werror=implicit-int -Werror=return-type -Werror=strict-prototypes -Wno-format-security -Wno-trigraphs -Wno-frame-address -Wno-address-of-packed-member -Wmissing-declarations -Wmissing-prototypes -Wframe-larger-than=2048 -Wno-gnu -Wno-format-overflow-non-kprintf -Wno-format-truncation-non-kprintf -Wvla -Wno-pointer-sign -Wcast-function-type -Wimplicit-fallthrough -Werror=date-time -Werror=incompatible-pointer-types -Wenum-conversion -Wextra -Wunused -Wno-unused-but-set-variable -Wno-unused-const-variable -Wno-format-overflow -Wno-override-init -Wno-pointer-to-enum-cast -Wno-tautological-constant-out-of-range-compare -Wno-unaligned-access -Wno-enum-compare-conditional -Wno-missing-field-initializers -Wno-type-limits -Wno-shift-negative-value -Wno-enum-enum-conversion -Wno-sign-compare -Wno-unused-parameter -DRANDSTRUCT -frandomize-layout-seed-file=./scripts/basic/randstruct.seed -fsanitize=array-bounds -fsanitize=shift -fsanitize=bool -fsanitize=enum -fsanitize-coverage=trace-pc -fsanitize-coverage=trace-cmp -fsanitize=thread -fno-optimize-sibling-calls -mllvm -tsan-compound-read-before-write=1 -mllvm -tsan-distinguish-volatile=1 -fdebug-info-for-profiling -mllvm -enable-fs-discriminator=true -mllvm -improved-fs-discriminator=true -gmlt -fbasic-block-sections=labels -DMODULE '-DKBUILD_BASENAME=".module_common"' '-DKBUILD_MODNAME=".module_common.o"' -D__KBUILD_MODNAME=kmod_.module_common.o -c
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

extern volatile int RANDOM;

// tty_std_termios [include/linux/tty.h line 379 column 24]
struct ktermios tty_std_termios = {0};

// __raw_spin_lock_init [include/linux/spinlock.h line 101 column 15]
extern void __raw_spin_lock_init(raw_spinlock_t *lock, const char *name,
				   struct lock_class_key *key, short inner) {
	spin_lock_init(lock);
}

// __read_overflow [include/linux/fortify-string.h line 57 column 6]
void __read_overflow(void) {}

// __read_overflow2 [include/linux/fortify-string.h line 58 column 6]
void __read_overflow2(void) {}

// __read_overflow2_field [include/linux/fortify-string.h line 59 column 6]
void __read_overflow2_field(size_t avail, size_t wanted) {}

// __real_kmemdup [include/linux/fortify-string.h line 754 column 14]
void *__real_kmemdup(const void *src, size_t len, gfp_t gfp) {
	return NULL;
}

// __real_memchr_inv [include/linux/fortify-string.h line 742 column 7]
void *__real_memchr_inv(const void *s, int c, size_t n) {
	return NULL;
}

// __real_memscan [include/linux/fortify-string.h line 699 column 14]
extern void *__real_memscan(void *s, int x, __kernel_size_t b) {
	return NULL;
}

// __real_strlcat [include/linux/fortify-string.h line 333 column 15]
extern size_t __real_strlcat(char *p, const char *q, size_t avail) {
	return 0;
}

// __real_strnlen [include/linux/fortify-string.h line 208 column 24]
extern __kernel_size_t __real_strnlen(const char *s, __kernel_size_t x) {
	return RANDOM;
}

// __real_strscpy [include/linux/fortify-string.h line 276 column 16]
extern ssize_t __real_strscpy(char *s, const char *x, size_t b) {
	return RANDOM;
}

// __tty_alloc_driver [include/linux/tty_driver.h line 475 column 20]
struct tty_driver *__tty_alloc_driver(unsigned int lines, struct module *owner,
		unsigned long flags) {
	static struct tty_driver drv = {0};
	return &drv;
}
		

// __write_overflow [include/linux/fortify-string.h line 60 column 6]
void __write_overflow(void) {}

// __write_overflow_field [include/linux/fortify-string.h line 61 column 6]
void __write_overflow_field(size_t avail, size_t wanted) {}

// _printk [include/linux/printk.h line 159 column 5]
__printf(1, 2) __cold
int _printk(const char *fmt, ...) {
	return 0;
}

// _raw_spin_lock_irqsave [include/linux/spinlock_api_smp.h line 32 column 26]
unsigned long __lockfunc _raw_spin_lock_irqsave(raw_spinlock_t *lock) {
	return spin_trylock(lock);
}

// _raw_spin_unlock_irqrestore [include/linux/spinlock_api_smp.h line 43 column 1]
void __lockfunc
_raw_spin_unlock_irqrestore(raw_spinlock_t *lock, unsigned long flags) {
	spin_unlock(lock);
}

// init_module [include/linux/module.h line 76 column 12]
extern int init_module(void) {
	return RANDOM % 1;
}

// register_console [include/linux/console.h line 627 column 13]
extern void register_console(struct console *c) {
}

// unregister_console [include/linux/console.h line 628 column 12]
extern int unregister_console(struct console *c) {
	return RANDOM % 1;
}

// tty_driver_kref_put [include/linux/tty_driver.h line 479 column 6]
void tty_driver_kref_put(struct tty_driver *driver) {}

// tty_port_close [include/linux/tty_port.h line 245 column 6]
void tty_port_close(struct tty_port *port, struct tty_struct *tty,
		struct file *filp) {}

// tty_port_destroy [include/linux/tty_port.h line 161 column 6]
void tty_port_destroy(struct tty_port *port) {}

// tty_port_hangup [include/linux/tty_port.h line 237 column 6]
void tty_port_hangup(struct tty_port *port) {}

// tty_port_init [include/linux/tty_port.h line 140 column 6]
void tty_port_init(struct tty_port *port) {}

// tty_port_link_device [include/linux/tty_port.h line 141 column 6]
void tty_port_link_device(struct tty_port *port, struct tty_driver *driver,
		unsigned index) {}

// tty_port_open [include/linux/tty_port.h line 249 column 5]
int tty_port_open(struct tty_port *port, struct tty_struct *tty,
		struct file *filp) {
	return RANDOM % 1;
}

// tty_register_driver [include/linux/tty_driver.h line 575 column 5]
int tty_register_driver(struct tty_driver *driver) {
	return RANDOM % 1;
}

// tty_unregister_driver [include/linux/tty_driver.h line 576 column 6]
void tty_unregister_driver(struct tty_driver *driver) {
}

extern void cleanup_module(void) {
}