# RaceHarness data availability repository

The implementation is structured as follows:
* This repository contains:
   * `rh_examples` --- environment models, payloads, task-specific Goblint configurations.
   * `patches` --- minimally necessary and seeded patches for the Linux kernel modules of the evaluation set.
   * `driver.py`, `goblint_driver` --- auxiliary scripts for Goblint execution.
   * `stub_generator` and `compile_db` --- auxiliary scripts for generating stubs and scanning Linux kernel build tree.
   * `reproduction/eval.sh` --- script automating the evaluation run.
* [https://github.com/sws-lab/race-harness-generator](https://github.com/sws-lab/race-harness-generator) --- race harness generator implementation
* [https://github.com/sws-lab/race-harness-goblint](https://github.com/sws-lab/race-harness-goblint) --- Goblint fork for Linux kernel verification
* [https://github.com/sws-lab/race-harness-cil](https://github.com/sws-lab/race-harness-cil) --- CIL fork for Linux kernel verification

Reproducing the results requires additional materials:
* Linux 6.14.9 source code --- [https://www.kernel.org/pub/linux/kernel/v6.x/linux-6.14.9.tar.xz](https://www.kernel.org/pub/linux/kernel/v6.x/linux-6.14.9.tar.xz) (sha256 390cdde032719925a08427270197ef55db4e90c09d454e9c3554157292c9f361)
* LTSmin 3.0.2 --- [https://github.com/utwente-fmt/ltsmin/releases/download/v3.0.2/ltsmin-v3.0.2-linux.tgz](https://github.com/utwente-fmt/ltsmin/releases/download/v3.0.2/ltsmin-v3.0.2-linux.tgz) (sha256 9112846d1b3f6c4db25179a5712ffc25b98c4c26799250875cba859808de07a1)

## Reproduction guide
To reproduce the results, prepare the following directory structure:
```bash
race-harness # https://github.com/sws-lab/race-harness (current commit)
race-harness-generator # https://github.com/sws-lab/race-harness-generator f044f58
race-harness-goblint # https://github.com/sws-lab/race-harness-goblint a4cf2ef14
race-harness-cil # https://github.com/sws-lab/race-harness-cil aa943ed6
linux-6.14.9.tar.xz # https://www.kernel.org/pub/linux/kernel/v6.x/linux-6.14.9.tar.xz sha256 390cdde032719925a08427270197ef55db4e90c09d454e9c3554157292c9f361
ltsmin-v3.0.2-linux.tgz # https://github.com/utwente-fmt/ltsmin/releases/download/v3.0.2/ltsmin-v3.0.2-linux.tgz sha256 9112846d1b3f6c4db25179a5712ffc25b98c4c26799250875cba859808de07a1
eval.sh # From reproduction directory
```

You should also install all dependencies necessary to build the Linux kernel, Goblint and CIL (as documented in respective README's).

Then, build Linux kernel as follows:
```bash
tar xvf linux-6.14.9.tar.xz
cd linux-6.14.9
make allmodconfig LLVM=-18 # Update LLVM version if needed
make LLVM=-18 -j
```

Unpack LTSmin:
```bash 
tar xvf ltsmin-v3.0.2-linux.tgz
```

Build race harness generator:
```bash
cd race-harness-generator
make -j$(nproc)
uv run driver.py # To initialize .venv
```

Build Goblint:
```bash
cd race-harness-goblint
make setup
make dev
pushd ../race-harness-cil
eval $(opam env --switch=$PWD/../race-harness-goblint --set-switch)
opam pin goblint-cil .
popd
make
```

Run the evaluation:
```bash
export LTSMIN_DIR=$PWD/v3.0.2
export RACE_HARNESS_DIR=$PWD/race-harness-generator
./eval.sh results
```
