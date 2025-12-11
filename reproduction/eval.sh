#!/usr/bin/env bash

OUTDIR="$(realpath $1)"
mkdir -p "$OUTDIR"

tar xvf ./linux-6.14.9.tar.xz
pushd linux-6.14.9
patch -p0 < ../race-harness/patches/linux-6.14.9-minimal.patch
popd

pushd race-harness 
find rh_examples -name "*.json" -print | while read TASK_FILEPATH; do
        TASK_FILENAME="$(basename $TASK_FILEPATH)"
        ./driver.py --db ../linux-6.14.9.db \
                --goblint $PWD/../race-harness-goblint/goblint \
                --harness-compiler ./rh_generator.sh \
                "rh_examples/$TASK_FILENAME" 2>&1 | tee "$OUTDIR/minpatch-$TASK_FILENAME.log"
done
popd

pushd linux-6.14.9
patch -p0 < ../race-harness/patches/linux-6.14.9-seeded.patch
popd

pushd race-harness 
find rh_examples -name "*.json" -print | while read TASK_FILEPATH; do
        TASK_FILENAME="$(basename $TASK_FILEPATH)"
        ./driver.py --db ../linux-6.14.9.db \
                --goblint $PWD/../race-harness-goblint/goblint \
                --harness-compiler ./rh_generator.sh \
                "rh_examples/$TASK_FILENAME" 2>&1 | tee "$OUTDIR/seeded-$TASK_FILENAME.log"
done
popd
