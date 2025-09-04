#!/usr/bin/env bash

set -xe

SCRIPT_DIR="$(dirname $(realpath $0))"
ROOT_DIR="$SCRIPT_DIR/.."

find "$SCRIPT_DIR" -name "*.json" -print | while read JSON_FILE; do
    BASENAME=`basename $JSON_FILE`
    if [[ "x$BASENAME" != "xcommon.json" ]]; then
        "$ROOT_DIR/driver.py" $@ "$JSON_FILE"
    fi
done
