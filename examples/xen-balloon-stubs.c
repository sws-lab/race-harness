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
#include "linux/kernel.h"
#include "linux/errno.h"
#include "linux/mm_types.h"
#include "linux/capability.h"
#include "linux/memory_hotplug.h"
#include "xen/xen.h"
#include "xen/balloon.h"
#include "xen/xenbus.h"
#include "xen/features.h"
#include "xen/page.h"
#include "xen/mem-reservation.h"

struct notifier_block *balloon_notifier;
struct xenbus_watch *balloon_watch;

// balloon_stats [include/xen/balloon.h line 25 column 29]
struct balloon_stats balloon_stats;

// max_mem_size [include/linux/memory_hotplug.h line 143 column 12]
u64 max_mem_size;

// xen_domain_type [include/xen/xen.h line 14 column 29]
enum xen_domain_type xen_domain_type;

// xen_start_flags [include/xen/xen.h line 30 column 17]
uint32_t xen_start_flags;

// balloon_set_new_target [include/xen/balloon.h line 27 column 6]
void balloon_set_new_target(unsigned long target) {}

// register_xenbus_watch [include/xen/xenbus.h line 188 column 5]
int register_xenbus_watch(struct xenbus_watch *watch) {
    balloon_watch = watch;
    return 1;
}

// register_xenstore_notifier [include/xen/xenbus.h line 185 column 5]
int register_xenstore_notifier(struct notifier_block *nb) {
    balloon_notifier = nb;
    return 1;
}

// xenbus_scanf [include/xen/xenbus.h line 167 column 5]
int xenbus_scanf(struct xenbus_transaction t,
		 const char *dir, const char *node, const char *fmt, ...) {
    return 1;
}

int subsys_system_register(const struct bus_type *subsys,
    const struct attribute_group **groups) {
    return __harness_rand();
}

int device_register(struct device *dev) {
    return __harness_rand();
}

void bus_unregister(const struct bus_type *bus) {}
