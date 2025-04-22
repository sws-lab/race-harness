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

struct ktermios tty_std_termios = {0};

struct tty_driver *__tty_alloc_driver(unsigned int lines, struct module *owner, unsigned long flags) {
  static struct tty_driver drv = {0};
  drv.owner = owner;
  drv.flags = flags;
  return &drv;
}

static struct tty_driver *registered_tty_driver = NULL;
int tty_register_driver(struct tty_driver *driver) {
  registered_tty_driver = driver;
  return 0;
}

void tty_unregister_driver(struct tty_driver *driver) {
  registered_tty_driver = NULL;
}

void tty_driver_kref_put(struct tty_driver *driver) {}

void tty_port_init(struct tty_port *port) {
  *port = (struct tty_port){0};
}

int tty_port_open(struct tty_port *port, struct tty_struct *tty, struct file *filp) {
	return 0;
}

void tty_port_destroy(struct tty_port *port) {}

void tty_port_hangup(struct tty_port *port) {}

void tty_port_link_device(struct tty_port *port, struct tty_driver *driver, unsigned index) {}

static struct console *registered_console = NULL;
void register_console(struct console *c) {
  registered_console = c;
}

int unregister_console(struct console *c) {
  registered_console = NULL;
  return 0;
}

int main(void) {
// Init module
  init_module();
  
  // Randomly drive the tty
#define MAX_OPEN_TTYS 32
#define MAX_WRITE_BUF_LENGTH 256
  struct tty_struct open_ttys[MAX_OPEN_TTYS];
  struct file open_tty_files[MAX_OPEN_TTYS];
  u8 write_buf[MAX_WRITE_BUF_LENGTH];
  unsigned int open_ttys_top = 0;
  for (; RANDOM % 4096 != 0;) {
    switch (RANDOM % 3) {
      case 0:
        if (open_ttys_top < MAX_OPEN_TTYS) {
          registered_tty_driver->ops->open(&open_ttys[open_ttys_top], &open_tty_files[open_ttys_top]);
          open_ttys_top++;
        }
        break;
    
      case 1:
        if (open_ttys_top > 0) {
          open_ttys_top--;
          registered_tty_driver->ops->hangup(&open_ttys[open_ttys_top]);
          registered_tty_driver->ops->close(&open_ttys[open_ttys_top], &open_tty_files[open_ttys_top]);
        }
        break;

      case 2:
        if (open_ttys_top > 0) {
          unsigned int tty = RANDOM % open_ttys_top;
          unsigned int length = registered_tty_driver->ops->write_room(&open_ttys[tty]);
          if (length > MAX_WRITE_BUF_LENGTH) {
            length = MAX_WRITE_BUF_LENGTH;
          }
          for (unsigned int i = 0; i < length; i++) {
            write_buf[i] = RANDOM;
          }
          registered_tty_driver->ops->write(&open_ttys[tty], write_buf, length);
        }
        break;
    }
  }

  // Cleanup module
  cleanup_module();
  return 0;
}
