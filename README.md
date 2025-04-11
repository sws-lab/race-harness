# Integration for Linux kernel analysis with Goblint
This is an on-going prototype integration to assist analysis of Linux kernel source code with Goblint.

Roughly, the following workflow is envisioned (focused primarily on module analysis):
1. Linux kernel is built normally using Clang (perhaps, with `allmodconfig` option to facilitate linkage of kernel modules).
2. Compilation database is generated from the Linux kernel build directory. The compilation database contains information on build target
dependencies, build commands, etc. Using the compilation database, it should be possible to identify and isolate source files of any given module.
3. For chosen module, a set of stub files is MANUALLY prepared. Stub files shall contain definitions of all external functions and global data used
by the module. Included definitions shall coarsely imitate actual kernel functions and include useful assetions to guide the analyzer.
Some stub files can be shared between different modules. Creation of stub files can be assisted by a separate script that scans the module
source code, extracts all undefined symbols and provides stub file skeleton containing empty definitions for identified symbols.
4. The analyzer is executed on a given set of files (actual module source code + stub files). Invocation of the analyzer can
be automated by a script.

Thus, the only work-intensive manual step is preparation of stub functions. Everything else (separation of module code,
generation of stub skeleton, invocation of Goblint) will be reasonably automated by a set of scripts.

Proposed workflow focuses exclusively on kernel module analysis. Analyzing kernel core subsystems is trickier, because
boundaries for analysis cannot be determined automatically. The user will need to list a set of object/source files corresponding
to a subsystem they wish to analyze, whereas other transitive dependencies, stub skeleton generation and analyzer invocation
can be done as described previously. Automatic discovery of subsystem code in a fine-grained manner requires some additional insights.

## Dependencies
Install LLVM, Clang, lld, python3-clang (tested from verion 19 from https://apt.llvm.org/).

## Usage
At the moment, compilation database generation, module boundary inference and stub skeleton generator has been implemented. Example usage:
```bash
mkdir kernel
cd kernel
wget https://cdn.kernel.org/pub/linux/kernel/v6.x/linux-6.14.1.tar.xz
tar xvf linux-6.14.1.tar.xz
cd linux-6.14.1.tar.xz
make allmodconfig LLVM=-19
make all LLVM=-19 -j$(nproc)
cd ../..

# Explore available modules
compile_db/extract_compilation_database.py --build-dir kernel/linux-6.14.1 --db kernel/linux-6.14.1.db
compile_db/query_builds.py --db kernel/linux-6.14.1.db
compile_db/query_module_base_deps.py  --db kernel/linux-6.14.1.db --build-id 27fc3ea7-1240-4223-9977-56c56a22c9f0 # UUID as printed by the previous command

# Determine chosen module dependencies and prepare stubs
./stub_generator/stub_generator.py --db kernel/linux-6.14.1.db --build-id 27fc3ea7-1240-4223-9977-56c56a22c9f0 drivers/char/ttyprintk.ko --blacklist ".*builtin.*" --blacklist ".*compiletime.*" --blacklist ".*fortify.*" > stubs.c # Generate stub skeleton
# Fill-in stubs.c
./stub_generator/stub_generator.py --db kernel/linux-6.14.1.db --build-id 27fc3ea7-1240-4223-9977-56c56a22c9f0 drivers/char/ttyprintk.ko stubs.c --blacklist ".*builtin.*" --blacklist ".*compiletime.*" --blacklist ".*fortify.*" # See what is missing
```