#!/bin/bash

# Apply patches to create the final book.js
# This script applies all patches from the patches/ directory to the original book.js

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
THEME_DIR="$(dirname "$SCRIPT_DIR")"
PATCHES_DIR="$THEME_DIR/patches"
ORIGINAL_FILE="$PATCHES_DIR/original/book.js"
FINAL_FILE="$THEME_DIR/book.js"
TEMP_FILE="$THEME_DIR/book.js.temp"

echo "Applying patches to create book.js..."

# Check if original file exists
if [ ! -f "$ORIGINAL_FILE" ]; then
    echo "Error: Original book.js not found at $ORIGINAL_FILE"
    exit 1
fi

# Copy original file to temp location
cp "$ORIGINAL_FILE" "$TEMP_FILE"

# Apply patches in order
for patch_file in "$PATCHES_DIR"/*.patch; do
    if [ -f "$patch_file" ]; then
        echo "Applying patch: $(basename "$patch_file")"
        if ! patch "$TEMP_FILE" < "$patch_file"; then
            echo "Error: Failed to apply patch $(basename "$patch_file")"
            rm -f "$TEMP_FILE"
            exit 1
        fi
    fi
done

# Move temp file to final location
mv "$TEMP_FILE" "$FINAL_FILE"

echo "Successfully created book.js with all patches applied"
echo "Generated: $FINAL_FILE"

# Verify the result exists and is not empty
if [ ! -s "$FINAL_FILE" ]; then
    echo "Error: Generated book.js is empty"
    exit 1
fi

echo "Patch application completed successfully"