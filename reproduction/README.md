To reproduce the results, prepare the following directory structure:
```bash
linux-kernel-goblint # https://github.com/sws-lab/linux-kernel-goblint 2f69d38
race-harness-impl # https://github.com/jprotopopov-ut/race-harness-impl f044f58
linux-verification-goblint # https://github.com/sws-lab/linux-verification-goblint a4cf2ef14
linux-verification-cil # https://github.com/sws-lab/linux-verification-cil aa943ed6
linux-6.14.9.tar.xz # sha256 390cdde032719925a08427270197ef55db4e90c09d454e9c3554157292c9f361
ltsmin-v3.0.2-linux.tgz # sha256 9112846d1b3f6c4db25179a5712ffc25b98c4c26799250875cba859808de07a1
eval.sh # Next to this README file
```

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
cd race-harness-impl
make -j
uv run driver.py # To initialize .venv
```

Build Goblint:
```bash
cd linux-verification-goblint
make setup
make dev
pushd ../linux-verification-cil
eval $(opam env --switch=$PWD/../linux-verification-goblint --set-switch)
opam pin goblint-cil .
popd
make
```

Run the evaluation:
```bash
export LTSMIN_DIR=$PWD/v3.0.2
export RACE_HARNESS_DIR=$PWD/race-harness-impl
./eval.sh results
```
