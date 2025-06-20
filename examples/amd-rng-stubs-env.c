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
#include "linux/delay.h"
#include "linux/hw_random.h"
#include "linux/io.h"
#include "linux/pci.h"

struct hwrng *registered_rng;

// ioport_resource [include/linux/ioport.h line 228 column 24]
struct resource ioport_resource;

// kmalloc_caches [include/linux/slab.h line 622 column 21]
kmem_buckets kmalloc_caches[NR_KMALLOC_TYPES];

// __release_region [include/linux/ioport.h line 355 column 13]
void __release_region(struct resource *, resource_size_t,
				resource_size_t) {}

// __request_region [include/linux/ioport.h line 346 column 26]
struct resource * __request_region(struct resource *,
					resource_size_t start,
					resource_size_t n,
					const char *name, int flags) {
	return __kmalloc(sizeof(struct resource), GFP_KERNEL);
}

// _dev_err [include/linux/dev_printk.h line 50 column 6]
void _dev_err(const struct device *dev, const char *fmt, ...) {}

// hwrng_register [include/linux/hw_random.h line 58 column 12]
int hwrng_register(struct hwrng *rng) {
	registered_rng = rng;
	return 0;
}

// hwrng_unregister [include/linux/hw_random.h line 61 column 13]
void hwrng_unregister(struct hwrng *rng) {}

// ioport_map [include/asm-generic/iomap.h line 92 column 22]
void __iomem *ioport_map(unsigned long port, unsigned int nr) {
	return __kmalloc(sizeof(char), GFP_KERNEL);
}

// ioport_unmap [include/asm-generic/iomap.h line 93 column 13]
void ioport_unmap(void __iomem *) {}

// ioread32 [include/asm-generic/iomap.h line 32 column 21]
unsigned int ioread32(const void __iomem *) {
	return __harness_rand();
}

void iounmap(volatile void __iomem *addr) {}

void __iomem *ioremap(resource_size_t offset, unsigned long size) {
	return __kmalloc(sizeof(char), GFP_KERNEL);
}

// pci_dev_put [include/linux/pci.h line 1188 column 6]
void pci_dev_put(struct pci_dev *dev) {}

// pci_get_device [include/linux/pci.h line 1216 column 17]
struct pci_dev *pci_get_device(unsigned int vendor, unsigned int device,
			       struct pci_dev *from) {
    return __kmalloc(sizeof(struct pci_dev), GFP_KERNEL);
}

// pci_match_id [include/linux/pci.h line 1633 column 29]
const struct pci_device_id *pci_match_id(const struct pci_device_id *ids,
					 struct pci_dev *dev) {
    return __kmalloc(sizeof(struct pci_device_id), GFP_KERNEL);
}

// pci_read_config_byte [include/linux/pci.h line 1253 column 5]
int pci_read_config_byte(const struct pci_dev *dev, int where, u8 *val) {
	*val = __harness_rand();
	return 0;
}

// pci_read_config_dword [include/linux/pci.h line 1255 column 5]
int pci_read_config_dword(const struct pci_dev *dev, int where, u32 *val) {
	*val = __harness_rand();
	return 0;
}

// pci_write_config_byte [include/linux/pci.h line 1256 column 5]
int pci_write_config_byte(const struct pci_dev *dev, int where, u8 val) {
	return 0;
}

// usleep_range_state [include/linux/delay.h line 63 column 6]
void usleep_range_state(unsigned long min, unsigned long max,
			unsigned int state) {}

int stop_machine(int (*fn)(void *), void *data, const struct cpumask *cpus) {
	return 0;
}