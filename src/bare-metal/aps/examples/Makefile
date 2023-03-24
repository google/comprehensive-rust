# Copyright 2023 Google LLC
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#      http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

UNAME := $(shell uname -s)
ifeq ($(UNAME),Linux)
	TARGET = aarch64-linux-gnu
else
	TARGET = aarch64-none-elf
endif
OBJCOPY = $(TARGET)-objcopy

.PHONY: build qemu qemu_logger qemu_minimal qemu_psci

all: minimal.bin improved.bin logger.bin

build:
	cargo build

improved.bin: build
	$(OBJCOPY) -O binary target/aarch64-unknown-none/debug/improved $@
logger.bin: build
	$(OBJCOPY) -O binary target/aarch64-unknown-none/debug/logger $@
minimal.bin: build
	$(OBJCOPY) -O binary target/aarch64-unknown-none/debug/minimal $@
psci.bin: build
	$(OBJCOPY) -O binary target/aarch64-unknown-none/debug/psci $@

qemu: improved.bin
	qemu-system-aarch64 -machine virt -cpu max -serial mon:stdio -display none -kernel $< -s
qemu_logger: logger.bin
	qemu-system-aarch64 -machine virt -cpu max -serial mon:stdio -display none -kernel $< -s
qemu_minimal: minimal.bin
	qemu-system-aarch64 -machine virt -cpu max -serial mon:stdio -display none -kernel $< -s
qemu_psci: psci.bin
	qemu-system-aarch64 -machine virt -cpu max -serial mon:stdio -display none -kernel $< -s

clean:
	cargo clean
	rm -f *.bin
