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

extern crate panic_halt as _;

use core::fmt::Write;
use cortex_m_rt::entry;
// ANCHOR_END: top
use lsm303agr::{Lsm303agr, MagOutputDataRate};
use microbit::{
    hal::{
        twim::Twim,
        uarte::{Baudrate, Parity, Uarte},
    },
    pac::twim0::frequency::FREQUENCY_A,
    Board,
};

// ANCHOR: main
#[entry]
fn main() -> ! {
    let board = Board::take().unwrap();

    // Configure serial port.
    let mut serial = Uarte::new(
        board.UARTE0,
        board.uart.into(),
        Parity::EXCLUDED,
        Baudrate::BAUD115200,
    );

    // Set up the I2C controller and IMU.
    // ANCHOR_END: main
    writeln!(serial, "Setting up IMU...").unwrap();
    let i2c = Twim::new(board.TWIM0, board.i2c_internal.into(), FREQUENCY_A::K100);
    let mut imu = Lsm303agr::new_with_i2c(i2c);
    imu.init().unwrap();
    imu.set_mag_odr(MagOutputDataRate::Hz50).unwrap();
    let mut imu = imu.into_mag_continuous().ok().unwrap();

    // ANCHOR: loop
    writeln!(serial, "Ready.").unwrap();

    loop {
        // Read compass data and log it to the serial port.
        // ANCHOR_END: loop
        while !imu.mag_status().unwrap().xyz_new_data {}
        let compass_reading = imu.mag_data().unwrap();
        writeln!(
            serial,
            "{},{},{}",
            compass_reading.x, compass_reading.y, compass_reading.z
        )
        .unwrap();
    }
}
