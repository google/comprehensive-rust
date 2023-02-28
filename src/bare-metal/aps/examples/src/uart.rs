// Copyright 2023 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

// ANCHOR: Example
use core::ptr::write_volatile;

/// Minimal driver for an 8250 UART. This only implements enough to work with
/// the emulated 8250 provided by crosvm, and won't work with real hardware.
#[derive(Debug)]
pub struct Uart {
    base_address: *mut u8,
}

impl Uart {
    /// Constructs a new instance of the UART driver for a device at the given
    /// base address.
    ///
    /// # Safety
    ///
    /// The given base address must point to the 8 MMIO control registers of an
    /// appropriate UART device, which must be mapped into the address space of
    /// the process as device memory and not have any other aliases.
    pub unsafe fn new(base_address: usize) -> Self {
        Self {
            base_address: base_address as *mut u8,
        }
    }

    /// Writes a single byte to the UART.
    pub fn write_byte(&self, byte: u8) {
        // Safe because we know that the base address points to the control
        // registers of a UART device which is appropriately mapped.
        unsafe {
            write_volatile(self.base_address, byte);
        }
    }
}
// ANCHOR_END: Example

// ANCHOR: Traits
use core::fmt::{self, Write};

impl Write for Uart {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.as_bytes() {
            self.write_byte(*c);
        }
        Ok(())
    }
}

// Safe because it just contains a pointer to device memory, which can be
// accessed from any context.
unsafe impl Send for Uart {}
// ANCHOR_END: Traits
