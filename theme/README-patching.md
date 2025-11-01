# Book.js Patch Management System

This directory contains a formalized patch management system for maintaining customizations to mdbook's `book.js` file. This system ensures that our customizations are preserved when updating mdbook versions while keeping them clearly documented and maintainable.

## Overview

Instead of directly editing `book.js`, we now:

1. **Store the original mdbook `book.js`** in `patches/original/`
2. **Keep customizations as patch files** in `patches/`
3. **Use modular enhancements** in separate JavaScript files
4. **Apply patches automatically** to generate the final `book.js`
5. **Verify patch integrity** in CI

## Directory Structure

```
theme/
├── book.js                                    # Generated file (do not edit directly)
├── comprehensive-rust-enhancements.js         # Modular enhancements
├── patches/
│   ├── original/
│   │   └── book.js                          # Original mdbook v0.4.40 book.js
│   ├── 001-enhanced-playground.patch         # Playground enhancements
│   └── 002-theme-improvements.patch          # Theme-related improvements
├── scripts/
│   ├── apply-patches.sh                     # Apply all patches
│   ├── extract-patches.sh                   # Extract patches from current book.js
│   └── verify-patches.sh                    # Verify patches apply cleanly
└── README-patching.md                       # This file
```

## Customizations Included

### 1. Enhanced Playground Functionality
- **Unused lint suppression**: Automatically adds `#![allow(unused)]` unless `warnunused` class is present
- **Analytics tracking**: Tracks playground usage with Google Analytics
- **Enhanced error handling**: Better error messages and stderr support  
- **Extended timeouts**: Increased from 6s to 15s for better reliability
- **Edition support**: Adds support for Rust 2024 edition
- **Test support**: Automatically runs `#[test]` functions when no main function is present

### 2. Theme Improvements
- **Theme ID validation**: Validates theme IDs to prevent invalid themes
- **Enhanced theme switching**: Better theme validation and fallback

### 3. Modular Design
- **Separate enhancement file**: `comprehensive-rust-enhancements.js` contains most custom logic
- **Minimal patches**: Patches only add integration points, not full implementations
- **Graceful fallbacks**: System works even if enhancement file fails to load

## Usage

### Daily Development

For normal development, you don't need to interact with the patch system. The `book.js` file is already generated and ready to use.

### Adding New Customizations

1. **First, try to add functionality to `comprehensive-rust-enhancements.js`**:
   ```javascript
   // Add your function to the enhancements file
   function myNewFeature() {
       // Your code here
   }
   
   // Export it
   window.comprehensiveRustEnhancements.myNewFeature = myNewFeature;
   ```

2. **If you need to modify `book.js` directly**:
   ```bash
   # Edit book.js directly for testing
   vim theme/book.js
   
   # Extract the changes as patches
   ./theme/scripts/extract-patches.sh
   
   # Split the generated patch into logical components
   # Edit theme/patches/current-customizations.patch
   # Save portions as new numbered patch files
   
   # Test the patches
   ./theme/scripts/verify-patches.sh
   ```

### Updating mdbook Version

When a new mdbook version is released:

1. **Download the new original `book.js`**:
   ```bash
   # Download from mdbook repository
   curl -o theme/patches/original/book.js \
     https://raw.githubusercontent.com/rust-lang/mdBook/v0.4.XX/src/theme/book.js
   ```

2. **Test patch compatibility**:
   ```bash
   ./theme/scripts/apply-patches.sh
   ```

3. **Resolve conflicts if any**:
   - If patches fail to apply, manually resolve conflicts
   - Update patch files as needed
   - Test thoroughly

4. **Verify everything works**:
   ```bash
   ./theme/scripts/verify-patches.sh
   mdbook serve  # Test the book
   ```

## Scripts Reference

### `apply-patches.sh`
Applies all patches to the original `book.js` to create the final version.

```bash
./theme/scripts/apply-patches.sh
```

### `verify-patches.sh`
Verifies that patches apply cleanly and produce the expected result.

```bash
./theme/scripts/verify-patches.sh
```

### `extract-patches.sh` 
Extracts differences between original and current `book.js` as patches.

```bash
./theme/scripts/extract-patches.sh
```

## CI Integration

The `.github/workflows/verify-book-js-patches.yml` workflow automatically:

- ✅ Verifies patches apply cleanly
- ✅ Checks that generated `book.js` matches the committed version
- ✅ Validates JavaScript syntax
- ✅ Ensures all required files exist
- ✅ Tests patch application from scratch

## Troubleshooting

### Patches Don't Apply
```bash
# Check which patch is failing
./theme/scripts/apply-patches.sh

# Manually apply patches one by one to debug
cd theme/patches
patch original/book.js < 001-enhanced-playground.patch
```

### Generated File Doesn't Match
```bash
# Extract current differences
./theme/scripts/extract-patches.sh

# Check what changed
cat theme/patches/current-customizations.patch
```

### JavaScript Errors
```bash
# Check syntax
node -c theme/book.js
node -c theme/comprehensive-rust-enhancements.js

# Test in browser dev tools
mdbook serve
# Open browser, check console for errors
```

## Benefits

- **✅ Maintainable**: Clear separation between original and custom code
- **✅ Traceable**: Each customization is documented as a patch  
- **✅ Testable**: CI verifies patches apply cleanly
- **✅ Updatable**: Easy to update mdbook versions
- **✅ Minimal**: Reduces patch size through modularization
- **✅ Robust**: Graceful fallbacks if enhancements fail
- **✅ Forward-compatible**: Preserves functionality across updates

## History

This system was created to solve issue #2924: "Formalize patching of `book.js`". Previously, `book.js` was directly edited, making mdbook updates risky and customizations hard to track.

## Related Files

- **`book.toml`**: Update to include `comprehensive-rust-enhancements.js` in `additional-js`
- **`.github/workflows/verify-book-js-patches.yml`**: CI workflow for validation
- **Issue #2924**: Original issue requesting this system