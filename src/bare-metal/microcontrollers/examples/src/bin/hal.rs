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
use gd32f1x0_hal::{pac::Peripherals, prelude::*};

#[entry]
fn main() -> ! {
    let p = Peripherals::take().unwrap();
    let mut rcu = p.RCU.constrain();

    // Enable GPIOC.
    let mut gpioc = p.GPIOC.split(&mut rcu.ahb);

    // Configure PC9 as a push-pull output.
    let mut led = gpioc.pc9.into_push_pull_output(&mut gpioc.config);

    // Set PC9 high to turn the LED on.
    led.set_high().unwrap();

    loop {}
}
