#!/usr/bin/env bash

WATCH_PATH="$1"
COMMAND_TO_EXECUTE="${@:2}"

inotifywait -m -e modify "$WATCH_PATH" --format '%w%f' |
while read -r FILE_PATH; do
    eval "$COMMAND_TO_EXECUTE"
done