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
#include "linux/input.h"
#include "linux/i2c.h"
#include "drivers/input/misc/adxl34x.h"

const struct adxl34x_bus_ops *registered_bops;

// adxl34x_groups [drivers/input/misc/adxl34x.h line 28 column 38]
const struct attribute_group *adxl34x_groups[1];

// adxl34x_pm [drivers/input/misc/adxl34x.h line 27 column 32]
const struct dev_pm_ops adxl34x_pm;

// _dev_err [include/linux/dev_printk.h line 50 column 6]
void _dev_err(const struct device *dev, const char *fmt, ...) {}

// adxl34x_probe [drivers/input/misc/adxl34x.h line 23 column 17]
struct adxl34x *adxl34x_probe(struct device *dev, int irq,
			      bool fifo_delay_default,
			      const struct adxl34x_bus_ops *bops) {
    registered_bops = bops;
    return kmalloc(sizeof(struct adxl34x), GFP_KERNEL);
}

// i2c_del_driver [include/linux/i2c.h line 896 column 6]
void i2c_del_driver(struct i2c_driver *driver) {}

// i2c_register_driver [include/linux/i2c.h line 895 column 5]
int i2c_register_driver(struct module *owner, struct i2c_driver *driver) {
    return 0;
}

// i2c_smbus_read_byte_data [include/linux/i2c.h line 154 column 5]
s32 i2c_smbus_read_byte_data(const struct i2c_client *client, u8 command) {
    return __harness_rand();
}

// i2c_smbus_read_i2c_block_data [include/linux/i2c.h line 182 column 5]
s32 i2c_smbus_read_i2c_block_data(const struct i2c_client *client,
				  u8 command, u8 length, u8 *values) {
    return __harness_rand();
}

// i2c_smbus_write_byte_data [include/linux/i2c.h line 155 column 5]
s32 i2c_smbus_write_byte_data(const struct i2c_client *client,
			      u8 command, u8 value) {
    return __harness_rand();
}

// i2c_transfer_buffer_flags [include/linux/i2c.h line 65 column 5]
int i2c_transfer_buffer_flags(const struct i2c_client *client,
			      char *buf, int count, u16 flags) {
    return 0;
}
