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

// ANCHOR: top
#![no_main]
#![no_std]

mod exceptions;
mod gicv3;
mod logger;
mod pl011;
// ANCHOR_END: top
mod pl031;

use crate::gicv3::{irq_enable, wfi, IntId, SgiTarget, Trigger};
use crate::pl031::Rtc;
use chrono::{TimeZone, Utc};
use core::hint::spin_loop;
// ANCHOR: imports
use crate::gicv3::GicV3;
use crate::pl011::Uart;
use core::panic::PanicInfo;
use log::{error, info, trace, LevelFilter};
use psci::system_off;

/// Base addresses of the GICv3.
const GICD_BASE_ADDRESS: *mut u64 = 0x800_0000 as _;
const GICR_BASE_ADDRESS: *mut u64 = 0x80A_0000 as _;

/// Base address of the primary PL011 UART.
const PL011_BASE_ADDRESS: *mut u32 = 0x900_0000 as _;
// ANCHOR_END: imports

/// Base address of the PL031 RTC.
const PL031_BASE_ADDRESS: *mut u32 = 0x901_0000 as _;
/// The IRQ used by the PL031 RTC.
const PL031_IRQ: IntId = IntId::spi(2);

// ANCHOR: main
#[no_mangle]
extern "C" fn main(x0: u64, x1: u64, x2: u64, x3: u64) {
    // Safe because `PL011_BASE_ADDRESS` is the base address of a PL011 device,
    // and nothing else accesses that address range.
    let uart = unsafe { Uart::new(PL011_BASE_ADDRESS) };
    logger::init(uart, LevelFilter::Trace).unwrap();

    info!("main({:#x}, {:#x}, {:#x}, {:#x})", x0, x1, x2, x3);

    // Safe because `GICD_BASE_ADDRESS` and `GICR_BASE_ADDRESS` are the base
    // addresses of a GICv3 distributor and redistributor respectively, and
    // nothing else accesses those address ranges.
    let mut gic = unsafe { GicV3::new(GICD_BASE_ADDRESS, GICR_BASE_ADDRESS) };
    gic.setup();
    // ANCHOR_END: main

    // Test sending an SGI.
    let sgi_intid = IntId::sgi(3);
    GicV3::set_priority_mask(0xff);
    gic.set_interrupt_priority(sgi_intid, 0x80);
    irq_enable();
    gic.enable_interrupt(sgi_intid, true);
    assert_eq!(gic.gicd_pending(0), 0);
    assert_eq!(gic.gicr_pending(), 0);
    assert_eq!(gic.gicd_active(0), 0);
    assert_eq!(gic.gicr_active(), 0);
    info!("Sending SGI");

    GicV3::send_sgi(
        sgi_intid,
        SgiTarget::List {
            affinity3: 0,
            affinity2: 0,
            affinity1: 0,
            target_list: 0b1,
        },
    );
    info!("Sent SGI");
    assert_eq!(gic.gicd_pending(0), 0);
    assert_eq!(gic.gicr_pending(), 0);
    assert_eq!(gic.gicd_active(0), 0);
    assert_eq!(gic.gicr_active(), 0);
    loop {
        wfi();
    }

    // Safe because `PL031_BASE_ADDRESS` is the base address of a PL031 device,
    // and nothing else accesses that address range.
    let mut rtc = unsafe { Rtc::new(PL031_BASE_ADDRESS) };
    let timestamp = rtc.read();
    let time = Utc.timestamp_opt(timestamp.into(), 0).unwrap();
    info!("RTC: {time}");

    GicV3::set_priority_mask(0xff);
    gic.set_interrupt_priority(PL031_IRQ, 0x80);
    gic.set_trigger(PL031_IRQ, Trigger::Level);
    irq_enable();
    gic.enable_interrupt(PL031_IRQ, true);

    // Wait for 3 seconds, without interrupts.
    let target = timestamp + 3;
    rtc.set_match(target);
    info!(
        "Waiting for {}",
        Utc.timestamp_opt(target.into(), 0).unwrap()
    );
    trace!(
        "matched={}, interrupt_pending={}",
        rtc.matched(),
        rtc.interrupt_pending()
    );
    while !rtc.matched() {
        spin_loop();
    }
    trace!(
        "matched={}, interrupt_pending={}",
        rtc.matched(),
        rtc.interrupt_pending()
    );
    info!("Finished waiting");

    // Wait another 3 seconds for an interrupt.
    let target = timestamp + 6;
    info!(
        "Waiting for {}",
        Utc.timestamp_opt(target.into(), 0).unwrap()
    );
    rtc.set_match(target);
    rtc.clear_interrupt();
    rtc.enable_interrupt(true);
    trace!(
        "matched={}, interrupt_pending={}",
        rtc.matched(),
        rtc.interrupt_pending()
    );
    while !rtc.interrupt_pending() {
        wfi();
    }
    trace!(
        "matched={}, interrupt_pending={}",
        rtc.matched(),
        rtc.interrupt_pending()
    );
    info!("Finished waiting");

    // ANCHOR: main_end
    system_off().unwrap();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    error!("{info}");
    system_off().unwrap();
    loop {}
}
// ANCHOR_END: main_end
