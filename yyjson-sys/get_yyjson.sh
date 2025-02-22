#!/bin/bash

set -e

REPO="git@github.com:ibireme/yyjson.git"
SRC_DIR="yyjson_source"
TARGET="yyjson"

# clone the yyjson repository
git clone --depth 1 "$REPO" "$SRC_DIR"

# remove existing yyjson dir if present
if [ -d "$TARGET" ]; then
	rm -rf "$TARGET"
fi

# Move the src directory to the current directory as 'yyjson'
mv "$SRC_DIR/src" "$TARGET"

# Remove the original repository directory
rm -rf "$SRC_DIR"

echo "yyjson source moved to ./$TARGET and $SRC_DIR removed."
