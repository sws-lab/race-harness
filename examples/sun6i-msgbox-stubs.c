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
#include "linux/bitops.h"
#include "linux/clk.h"
#include "linux/device.h"
#include "linux/interrupt.h"
#include "linux/io.h"
#include "linux/mailbox_controller.h"
#include "linux/of_irq.h"
#include "linux/platform_device.h"
#include "linux/reset.h"

// ____wrong_branch_error [include/linux/jump_label.h line 419 column 13]
bool ____wrong_branch_error(void) {
    __goblint_assert(0);
}

// __devm_reset_control_get [include/linux/reset.h line 97 column 23]
struct reset_control *__devm_reset_control_get(struct device *dev,
				     const char *id, int index, enum reset_control_flags flags) {
    return __kmalloc(sizeof(struct reset_control), GFP_KERNEL);
}

// __dynamic_dev_dbg [include/linux/dynamic_debug.h line 146 column 6]
void __dynamic_dev_dbg(struct _ddebug *descriptor, const struct device *dev,
		       const char *fmt, ...) {}

// _dev_err [include/linux/dev_printk.h line 50 column 6]
void _dev_err(const struct device *dev, const char *fmt, ...) {}

// clk_disable [include/linux/clk.h line 726 column 6]
void clk_disable(struct clk *clk) {}

// clk_enable [include/linux/clk.h line 698 column 5]
int clk_enable(struct clk *clk) {
    return 0;
}

// clk_prepare [include/linux/clk.h line 307 column 5]
int clk_prepare(struct clk *clk) {
    return 0;
}

// clk_unprepare [include/linux/clk.h line 357 column 6]
void clk_unprepare(struct clk *clk) {}

// devm_clk_get [include/linux/clk.h line 535 column 13]
struct clk *devm_clk_get(struct device *dev, const char *id) {
    return __kmalloc(sizeof(struct clk), GFP_KERNEL);
}

// devm_kmalloc [include/linux/device/devres.h line 47 column 1]
void * devm_kmalloc(struct device *dev, size_t size, gfp_t gfp) {
    return __kmalloc(size, gfp);
}

// devm_platform_ioremap_resource [include/linux/platform_device.h line 72 column 1]
void __iomem *devm_platform_ioremap_resource(struct platform_device *pdev,
			       unsigned int index) {
    return __kmalloc(sizeof(__iomem), GFP_KERNEL);
}

// devm_request_threaded_irq [include/linux/interrupt.h line 209 column 1]
int devm_request_threaded_irq(struct device *dev, unsigned int irq,
			  irq_handler_t handler, irq_handler_t thread_fn,
			  unsigned long irqflags, const char *devname,
			  void *dev_id) {
    return 0;
}

// irq_of_parse_and_map [include/linux/of_irq.h line 116 column 21]
unsigned int irq_of_parse_and_map(struct device_node *node, int index) {
    return 0;
}

// mbox_chan_received_data [include/linux/mailbox_controller.h line 132 column 6]
void mbox_chan_received_data(struct mbox_chan *chan, void *data) {}

// mbox_controller_register [include/linux/mailbox_controller.h line 130 column 5]
int mbox_controller_register(struct mbox_controller *mbox) {
    return 0;
}

// mbox_controller_unregister [include/linux/mailbox_controller.h line 131 column 6]
void mbox_controller_unregister(struct mbox_controller *mbox) {}

// reset_control_deassert [include/linux/reset.h line 75 column 5]
int reset_control_deassert(struct reset_control *rstc) {
    return 0;
}
