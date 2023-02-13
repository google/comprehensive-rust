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

#![no_main]
#![no_std]

extern crate panic_halt as _;

use cortex_m_rt::entry;
use gd32f1x0_hal::{
    gpio::{
        gpioc::{PC1, PC2, PC9},
        Floating, Input, OpenDrain, Output, PushPull,
    },
    pac::Peripherals,
    prelude::*,
};

// ANCHOR: Example
#[entry]
fn main() -> ! {
    let p = Peripherals::take().unwrap();
    let mut rcu = p.RCU.constrain();
    let mut gpioc = p.GPIOC.split(&mut rcu.ahb);
    let pc9: PC9<Input<Floating>> = gpioc.pc9;
    // let pc9_again = gpioc.pc9; // Error, moved.
    if pc9.is_high().unwrap() {
        // ...
    }
    let mut pc9_output: PC9<Output<OpenDrain>> =
        pc9.into_open_drain_output(&mut gpioc.config);
    pc9_output.set_high().unwrap();
    // pc9.is_high(); // Error, moved.

    let _pc1: PC1<Output<OpenDrain>> = gpioc.pc1.into_open_drain_output(&mut gpioc.config);
    let _pc2: PC2<Output<PushPull>> = gpioc.pc2.into_push_pull_output(&mut gpioc.config);

    loop {}
}
