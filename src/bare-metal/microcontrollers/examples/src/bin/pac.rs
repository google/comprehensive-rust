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
#![no_main]
#![no_std]

extern crate panic_halt as _;

use cortex_m_rt::entry;
use gd32f1::gd32f130::Peripherals;

#[entry]
fn main() -> ! {
    let p = Peripherals::take().unwrap();
    let gpioc = p.GPIOC;

    // Enable GPIOC.
    p.RCU.ahben.modify(|_, w| w.pcen().enabled());

    // Configure PC9 as a push-pull output.
    gpioc.pud.modify(|_, w| w.pud9().floating());
    gpioc.omode.modify(|_, w| w.om9().push_pull());
    gpioc.ctl.modify(|_, w| w.ctl9().output());

    // Set PC9 high to turn the LED on.
    gpioc.bop.write(|w| w.bop9().set());

    loop {}
}
