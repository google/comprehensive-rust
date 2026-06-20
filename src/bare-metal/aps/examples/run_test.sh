#!/bin/bash
# Copyright 2026 Google LLC
# SPDX-License-Identifier: Apache-2.0

set -euo pipefail

BIN_NAME="$1"
BIN_PATH="src/bare-metal/aps/examples/${BIN_NAME}.bin"
# Detect if running as an automated test under 'bazel test'
NON_INTERACTIVE="${TEST_SRCDIR:-}"

if [ ! -f "$BIN_PATH" ]; then
    echo "Error: Binary not found at $BIN_PATH" >&2
    exit 1
fi

echo "Running ${BIN_NAME}.bin under QEMU..."
if [ -n "$NON_INTERACTIVE" ]; then
    echo "q" | qemu-system-aarch64 -machine virt -cpu max -serial mon:stdio -display none -kernel "$BIN_PATH"
else
    qemu-system-aarch64 -machine virt -cpu max -serial mon:stdio -display none -kernel "$BIN_PATH"
fi
