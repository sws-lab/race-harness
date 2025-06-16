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
#include "linux/tty.h"
#include "linux/mm.h"
#include "linux/kbd_kern.h"
#include "linux/vt_kern.h"
#include "linux/selection.h"
#include "linux/console.h"
#include "linux/tty_flip.h"

// fg_console [include/linux/vt_kern.h line 22 column 12]
int fg_console = 0;

// kmalloc_caches [include/linux/slab.h line 622 column 21]
kmem_buckets kmalloc_caches[NR_KMALLOC_TYPES];

// vc_cons [include/linux/console_struct.h line 172 column 18]
static struct vc_data vc_data = {
    .vc_num = 0,
    .vc_cols = 100,
    .vc_rows = 100,
    .vc_size_row = 100
};
struct vc vc_cons [MAX_NR_CONSOLES] = {
    {
        .d = &vc_data
    }
};

// __bad_size_call_parameter [include/linux/percpu-defs.h line 310 column 13]
void __bad_size_call_parameter(void) {
    __goblint_assert(0);
}

// __kcsan_check_access [include/linux/kcsan-checks.h line 37 column 6]
void __kcsan_check_access(const volatile void *ptr, size_t size, int type) {}

// __warn_printk [include/asm-generic/bug.h line 93 column 28]
void __warn_printk(const char *fmt, ...) {}

// __xchg_wrong_size [arch/x86/include/asm/cmpxchg.h line 13 column 13]
void __xchg_wrong_size(void) {
    __goblint_assert(0);    
}

// _copy_from_user [include/linux/uaccess.h line 187 column 1]
unsigned long _copy_from_user(void *, const void __user *, unsigned long) {}

// add_wait_queue [include/linux/wait.h line 164 column 13]
void add_wait_queue(struct wait_queue_head *wq_head, struct wait_queue_entry *wq_entry) {}

// complement_pos [include/linux/selection.h line 38 column 6]
void complement_pos(struct vc_data *vc, int offset) {}

// console_lock [include/linux/console.h line 629 column 13]
void console_lock(void) {}

// console_unlock [include/linux/console.h line 631 column 13]
void console_unlock(void) {}

// default_wake_function [include/linux/wait.h line 16 column 5]
int default_wake_function(struct wait_queue_entry *wq_entry, unsigned mode, int flags, void *key) {}

// inverse_translate [include/linux/consolemap.h line 25 column 5]
u16 inverse_translate(const struct vc_data *conp, u16 glyph, bool use_unicode) {}

// invert_screen [include/linux/selection.h line 39 column 6]
void invert_screen(struct vc_data *vc, int offset, int count, bool viewed) {}

// mouse_report [include/linux/selection.h line 24 column 6]
void mouse_report(struct tty_struct *tty, int butt, int mrx, int mry) {}

// mouse_reporting [include/linux/selection.h line 23 column 5]
int mouse_reporting(void) {}

// mutex_lock_nested [include/linux/mutex.h line 157 column 13]
void mutex_lock_nested(struct mutex *lock, unsigned int subclass) {
	_raw_spin_lock(lock);
}

// poke_blanked_console [include/linux/vt_kern.h line 34 column 6]
void poke_blanked_console(void) {}

// remove_wait_queue [include/linux/wait.h line 167 column 13]
void remove_wait_queue(struct wait_queue_head *wq_head, struct wait_queue_entry *wq_entry) {}

// schedule [include/linux/sched.h line 319 column 17]
void schedule(void) {}

// screen_glyph [include/linux/selection.h line 36 column 5]
extern u16 glyphs16[];
u16 screen_glyph(const struct vc_data *vc, int offset) {
    return glyphs16[offset];
}

// screen_glyph_unicode [include/linux/selection.h line 37 column 5]
extern u32 glyphs[];
u32 screen_glyph_unicode(const struct vc_data *vc, int offset) {
    return glyphs[offset];
}

// tty_buffer_lock_exclusive [include/linux/tty_flip.h line 89 column 6]
void tty_buffer_lock_exclusive(struct tty_port *port) {}

// tty_buffer_unlock_exclusive [include/linux/tty_flip.h line 90 column 6]
void tty_buffer_unlock_exclusive(struct tty_port *port) {}

// tty_ldisc_deref [include/linux/tty_ldisc.h line 280 column 6]
void tty_ldisc_deref(struct tty_ldisc *) {}

// tty_ldisc_receive_buf [include/linux/tty_flip.h line 86 column 8]
size_t tty_ldisc_receive_buf(struct tty_ldisc *ld, const u8 *p, const u8 *f,
			     size_t count) {
    return __harness_rand() % (count + 1);
}

// tty_ldisc_ref_wait [include/linux/tty_ldisc.h line 281 column 19]
struct tty_ldisc *tty_ldisc_ref_wait(struct tty_struct *) {
    static struct tty_ldisc ldisc;
    return &ldisc;
}

// vt_do_kdgkbmode [include/linux/vt_kern.h line 165 column 5]
int vt_do_kdgkbmode(unsigned int console) {
    return 0;
}
