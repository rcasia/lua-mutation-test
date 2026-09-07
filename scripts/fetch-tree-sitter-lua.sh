#!/usr/bin/env bash
set -euo pipefail

REPO_URL="https://github.com/tree-sitter-grammars/tree-sitter-lua.git"
DEST_DIR="tree-sitter-lua"

if [ -d "$DEST_DIR/.git" ]; then
    echo "tree-sitter-lua already cloned; pulling latest..."
    git -C "$DEST_DIR" pull --ff-only
else
    echo "Cloning tree-sitter-lua..."
    git clone "$REPO_URL" "$DEST_DIR"
fi

echo "tree-sitter-lua is ready at $DEST_DIR"
