#!/bin/bash

# Verify that patches apply cleanly and result matches current book.js
# This script is used in CI to ensure patch integrity

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
THEME_DIR="$(dirname "$SCRIPT_DIR")"
CURRENT_FILE="$THEME_DIR/book.js"
BACKUP_FILE="$THEME_DIR/book.js.backup"

echo "Verifying patch integrity..."

# Create backup of current file
if [ -f "$CURRENT_FILE" ]; then
    cp "$CURRENT_FILE" "$BACKUP_FILE"
    echo "Created backup: $BACKUP_FILE"
fi

# Apply patches to generate new file
"$SCRIPT_DIR/apply-patches.sh"

# Compare with original if backup exists
if [ -f "$BACKUP_FILE" ]; then
    echo "Comparing generated file with original..."
    if diff -u "$BACKUP_FILE" "$CURRENT_FILE" > /dev/null; then
        echo "✅ Generated book.js matches the original - patches are correct"
        rm -f "$BACKUP_FILE"
        exit 0
    else
        echo "❌ Generated book.js differs from original"
        echo "Differences:"
        diff -u "$BACKUP_FILE" "$CURRENT_FILE" || true
        
        # Restore backup
        mv "$BACKUP_FILE" "$CURRENT_FILE"
        echo "Restored original file"
        exit 1
    fi
else
    echo "✅ Patches applied successfully (no original file to compare)"
    exit 0
fi