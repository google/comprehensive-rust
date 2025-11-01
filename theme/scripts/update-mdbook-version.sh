#!/bin/bash

# Update to a new mdbook version
# This script downloads the new book.js, tests patches, and handles conflicts

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
THEME_DIR="$(dirname "$SCRIPT_DIR")"
PATCHES_DIR="$THEME_DIR/patches"
ORIGINAL_DIR="$PATCHES_DIR/original"
ORIGINAL_FILE="$ORIGINAL_DIR/book.js"

# Check if version argument is provided
if [ $# -eq 0 ]; then
    echo "Usage: $0 <mdbook-version>"
    echo "Example: $0 v0.4.41"
    exit 1
fi

MDBOOK_VERSION="$1"
TEMP_FILE="$ORIGINAL_DIR/book.js.new"
BACKUP_FILE="$ORIGINAL_DIR/book.js.backup"

echo "Updating mdbook to version: $MDBOOK_VERSION"

# Create backup of current original
if [ -f "$ORIGINAL_FILE" ]; then
    cp "$ORIGINAL_FILE" "$BACKUP_FILE"
    echo "Created backup of current original: $BACKUP_FILE"
fi

# Download new book.js
echo "Downloading new book.js from mdbook $MDBOOK_VERSION..."
URL="https://raw.githubusercontent.com/rust-lang/mdBook/$MDBOOK_VERSION/src/theme/book.js"

if curl -f -o "$TEMP_FILE" "$URL"; then
    echo "Successfully downloaded new book.js"
else
    echo "Error: Failed to download book.js from $URL"
    echo "Please check if the version $MDBOOK_VERSION exists"
    exit 1
fi

# Test if patches still apply to the new version
echo "Testing if patches apply to new version..."
cp "$TEMP_FILE" "$ORIGINAL_FILE"

if "$SCRIPT_DIR/apply-patches.sh"; then
    echo "✅ All patches apply successfully to $MDBOOK_VERSION"
    rm -f "$TEMP_FILE" "$BACKUP_FILE"
    
    echo "Update completed successfully!"
    echo ""
    echo "Next steps:"
    echo "1. Test the book: mdbook serve"
    echo "2. Verify functionality in browser"
    echo "3. Run tests: cargo test && npm test"
    echo "4. Commit the changes"
    
else
    echo "❌ Patches failed to apply to $MDBOOK_VERSION"
    echo "Restoring original file..."
    
    if [ -f "$BACKUP_FILE" ]; then
        mv "$BACKUP_FILE" "$ORIGINAL_FILE"
        echo "Restored original file"
    fi
    
    rm -f "$TEMP_FILE"
    
    echo ""
    echo "Manual conflict resolution needed:"
    echo "1. Check which patches failed"
    echo "2. Manually resolve conflicts in patch files"
    echo "3. Test with: ./scripts/apply-patches.sh"
    echo "4. Verify with: ./scripts/verify-patches.sh"
    echo "5. Re-run this script"
    
    exit 1
fi