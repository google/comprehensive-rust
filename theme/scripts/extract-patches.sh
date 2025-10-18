#!/bin/bash

# Extract patches from the current book.js by comparing it to the original
# This is useful when updating customizations to regenerate patches

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
THEME_DIR="$(dirname "$SCRIPT_DIR")"
PATCHES_DIR="$THEME_DIR/patches"
ORIGINAL_FILE="$PATCHES_DIR/original/book.js"
CURRENT_FILE="$THEME_DIR/book.js"
TEMP_PATCH="$PATCHES_DIR/current-customizations.patch"

echo "Extracting patches from current book.js..."

# Check if files exist
if [ ! -f "$ORIGINAL_FILE" ]; then
    echo "Error: Original book.js not found at $ORIGINAL_FILE"
    exit 1
fi

if [ ! -f "$CURRENT_FILE" ]; then
    echo "Error: Current book.js not found at $CURRENT_FILE"
    exit 1
fi

# Generate unified diff
echo "Generating patch file..."
if diff -u "$ORIGINAL_FILE" "$CURRENT_FILE" > "$TEMP_PATCH"; then
    echo "No differences found - current book.js matches original"
    rm -f "$TEMP_PATCH"
    exit 0
else
    echo "Generated patch: $TEMP_PATCH"
    echo "Differences found:"
    echo "  Lines added: $(grep '^+' "$TEMP_PATCH" | wc -l)"
    echo "  Lines removed: $(grep '^-' "$TEMP_PATCH" | wc -l)"
    echo ""
    echo "To split this into logical patches:"
    echo "1. Edit the patch file to separate different features"
    echo "2. Save each feature as a separate .patch file in $PATCHES_DIR"
    echo "3. Test with ./scripts/apply-patches.sh"
    echo "4. Verify with ./scripts/verify-patches.sh"
fi

echo "Patch extraction completed"