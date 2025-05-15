#!/bin/bash
# Get the directory of the script
# depends on input / tmp directory
TMP_DIR="$1"
INPUT_DIR="/input"

echo "Copying files to $TMP_DIR"

cp -r "$INPUT_DIR"/* "$TMP_DIR"/

echo "Done."