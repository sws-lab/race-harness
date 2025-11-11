#!/usr/bin/env bash

if [[ "x$RACE_HARNESS_DIR" == "x" ]]; then
    echo "RACE_HARNESS_DIR environment variable shall be defined" >&2
    exit -1
fi

if [[ "x$LTSMIN_DIR" == "x" ]]; then
    echo "LTSMIN_DIR environment variable shall be defined" >&2
    exit -1
fi

. "$RACE_HARNESS_DIR/.venv/bin/activate"
exec "$RACE_HARNESS_DIR/driver.py" \
    --encoding goblint-kernel --embed-header \
    --ltsmin "$LTSMIN_DIR" \
    --pins-stir "$RACE_HARNESS_DIR/pins-stir" \
    $@
