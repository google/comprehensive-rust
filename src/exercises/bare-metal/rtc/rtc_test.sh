#!/bin/bash
# Copyright 2026 Google LLC
# SPDX-License-Identifier: Apache-2.0

set -euo pipefail

BIN_PATH="src/exercises/bare-metal/rtc/rtc.bin"

if [ ! -f "$BIN_PATH" ]; then
    echo "Error: rtc.bin not found at $BIN_PATH" >&2
    exit 1
fi

echo "Running rtc.bin under QEMU..."
qemu-system-aarch64 -machine virt,gic-version=3 -cpu max -serial mon:stdio -display none -kernel "$BIN_PATH"
