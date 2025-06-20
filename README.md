# Integration for Linux kernel analysis with Goblint
This is an on-going prototype integration to assist analysis of Linux kernel
source code with Goblint.

Roughly, the following workflow is implemented (focused primarily on module
analysis):
1. Linux kernel is built normally using Clang (perhaps, with `allmodconfig`
   option to facilitate linkage of kernel modules).
2. Compilation database is generated from the Linux kernel build directory. The
commands, etc. Using the compilation database, it should be possible to identify
compilation database contains information on build target dependencies, build
and isolate source files of any given module.
3. For chosen module, a set of stub files is MANUALLY prepared. Stub files shall
contain definitions of all external functions and global data used by the
module. Included definitions shall coarsely imitate actual kernel functions and
include useful assetions to guide the analyzer. Some stub files can be shared
between different modules. Creation of stub files can be assisted by a separate
script that scans the module source code, extracts all undefined symbols and
provides stub file skeleton containing empty definitions for identified symbols.
4. A model of module interaction with the environment (i.e. kernel) is MANUALLY
defined by user. The model represents possible module and environment states as
communicating processes. A verification harness is automatically generated from
the model. The harness represents kernel interaction with the module as C code.
4. The analyzer is executed on a given set of files (actual module source code +
   stub files + harness). Invocation of the analyzer is automated by a script.

Thus, two work-intensive manual steps are preparation of stub functions and
environment model. Everything else (separation of module code, generation of
stub skeleton, generation of harness, invocation of Goblint) is reasonably
automated by a set of scripts.

Proposed workflow focuses exclusively on kernel module analysis, and in
particular, on data race verification. Analyzing kernel core subsystems is
trickier, because boundaries for analysis cannot be determined automatically.
The user will need to list a set of object/source files corresponding to a
subsystem they wish to analyze, whereas other transitive dependencies, stub
skeleton generation and analyzer invocation can be done as described previously.
Automatic discovery of subsystem code in a fine-grained manner requires some
additional insights.

## Dependencies
Install LLVM, Clang, lld, python3-clang (tested from verion 19 from
https://apt.llvm.org/). Patched version of Goblint and Cil shall be used from
https://github.com/sws-lab/linux-verification-goblint  and
https://github.com/sws-lab/linux-verification-cil respectively.

## Usage
Example usage:
```bash
# Build the kernel with Clang. Use allmodconfig to maximize module segregation.
mkdir kernel
cd kernel
wget https://cdn.kernel.org/pub/linux/kernel/v6.x/linux-6.14.1.tar.xz
tar xvf linux-6.14.1.tar.xz
cd linux-6.14.1.tar.xz
make allmodconfig LLVM=-19
make all LLVM=-19 -j$(nproc)
cd ../..

# Prepare compilation database
compile_db/extract_compilation_database.py --build-dir kernel/linux-6.14.1 --db kernel/linux-6.14.1.db

# Explore available modules
compile_db/query_compilation_database.py  --db kernel/linux-6.14.1.db --query-builds
compile_db/query_compilation_database.py  --db kernel/linux-6.14.1.db --build-id 27fc3ea7-1240-4223-9977-56c56a22c9f0 --query-base-deps # UUID as printed by the previous command; can be omitted to use the latest added build
compile_db/query_compilation_database.py  --db kernel/linux-6.14.1.db --build-id 27fc3ea7-1240-4223-9977-56c56a22c9f0 --target drivers/char/ttyprintk.ko --query-all-deps # See above comment

# Determine chosen module dependencies and prepare stubs
./stub_generator/stub_generator.py --db kernel/linux-6.14.1.db --build-id 27fc3ea7-1240-4223-9977-56c56a22c9f0 drivers/char/ttyprintk.ko --blacklist ".*builtin.*" --blacklist ".*compiletime.*" --blacklist ".*fortify.*" > ~/ttyprintk-stubs.c 
./stub_generator/stub_generator.py --db kernel/linux-6.14.1.db .examples/generic-stubs.c  drivers/input/misc/pcspkr.ko --blacklist ".*builtin.*" --blacklist ".*compiletime.*" --blacklist ".*fortify.*" --blacklist "pcpu_hot" --blacklist "const_pcpu_hot" --include "pcspkr_platform_driver_init" --include "pcspkr_platform_driver_exit" > ~/pcspkr-stubs.c
# Generate stub skeleton
# Fill-in stubs.c
# See examples/*stubs.c
./stub_generator/stub_generator.py --db kernel/linux-6.14.1.db --build-id 27fc3ea7-1240-4223-9977-56c56a22c9f0 drivers/char/ttyprintk.ko ~/ttyprintk-stubs.c  --blacklist ".*builtin.*" --blacklist ".*compiletime.*" --blacklist ".*fortify.*" # See what is missing

# Build harness compiler
cd harness2
cargo build --release

# Edit harnesses
# See harness2/examples

# Edit driver configuration
# See examples/*.json

# Run Goblint on the chosen configuration
./driver.py --db kernel/linux-6.14.1.db --goblint ~/goblint/analyzer/goblint --harness-compiler harness2/target/release/harness2 examples/ttyprintk.json
# Or some other module
/driver.py --db kernel/linux-6.14.1.db --goblint ~/goblint/analyzer/goblint --harness-compiler harness2/target/release/harness2 examples/pcspkr.json
```

### Kernel module execution hack
This is an example of using special harness to actually run a kernel module
```bash
HARNESS_KERNEL_EXECUTABLE=yes ./harness2/target/release/harness2 harness2/examples/ttyprintk-executable.harness > ~/2.c
clang-19 -I. -Wp,-MMD,./..module-common.o.d -I./arch/x86/include -I./arch/x86/include/generated -I./include -I./include -I./arch/x86/include/uapi -I./arch/x86/include/generated/uapi -I./include/uapi -I./include/generated/uapi -include ./include/linux/compiler-version.h -include ./include/linux/kconfig.h -include ./include/linux/compiler_types.h -D__KERNEL__  -std=gnu11 -Wno-sign-compare   -DCC_USING_NOP_MCOUNT -DCC_USING_FENTRY -fno-strict-overflow -fno-stack-check  -Wno-format-security -Wno-trigraphs  -Wno-address-of-packed-member -Wmissing-declarations -Wmissing-prototypes -Wno-gnu -Wno-format-overflow-non-kprintf -Wno-format-truncation-non-kprintf -Wno-unused-but-set-variable -Wno-unused-const-variable -Wno-format-overflow -Wno-override-init -Wno-pointer-to-enum-cast -Wno-tautological-constant-out-of-range-compare -Wno-unaligned-access -Wno-enum-compare-conditional -Wno-missing-field-initializers -Wno-type-limits -Wno-shift-negative-value -Wno-enum-enum-conversion -Wno-sign-compare -Wno-unused-parameter -Wno-pointer-sign -fno-optimize-sibling-calls -fcf-protection -DMODULE '-DKBUILD_BASENAME=".module_common"' '-DKBUILD_MODNAME=".module_common.o"' -D__KBUILD_MODNAME=kmod_.module_common.o -o ~/2 -O2 -ggdb ~/2.c drivers/char/ttyprintk.c /home/jprotopopov/goblint-linux-kernel/examples/ttyprintk-exec-stubs.c -fPIE -fsanitize=thread # Adjust paths as necessary
~/2
```