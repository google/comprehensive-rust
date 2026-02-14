// Copyright 2023 Google LLC
// SPDX-License-Identifier: Apache-2.0
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

use aarch64_rt::{ExceptionHandlers, RegisterStateRef, exception_handlers};
use arm_gic::gicv3::{GicCpuInterface, InterruptGroup};
use log::{error, info, trace};
use smccc::Hvc;
use smccc::psci::system_off;

struct Handlers;

impl ExceptionHandlers for Handlers {
    extern "C" fn sync_current(_state: RegisterStateRef) {
        error!("sync_current");
        system_off::<Hvc>().unwrap();
    }

    extern "C" fn irq_current(_state: RegisterStateRef) {
        trace!("irq_current");
        let intid =
            GicCpuInterface::get_and_acknowledge_interrupt(InterruptGroup::Group1)
                .expect("No pending interrupt");
        info!("IRQ {intid:?}");
    }

    extern "C" fn fiq_current(_state: RegisterStateRef) {
        error!("fiq_current");
        system_off::<Hvc>().unwrap();
    }

    extern "C" fn serror_current(_state: RegisterStateRef) {
        error!("serror_current");
        system_off::<Hvc>().unwrap();
    }

    extern "C" fn sync_lower(_state: RegisterStateRef) {
        error!("sync_lower");
        system_off::<Hvc>().unwrap();
    }

    extern "C" fn irq_lower(_state: RegisterStateRef) {
        error!("irq_lower");
        system_off::<Hvc>().unwrap();
    }

    extern "C" fn fiq_lower(_state: RegisterStateRef) {
        error!("fiq_lower");
        system_off::<Hvc>().unwrap();
    }

    extern "C" fn serror_lower(_state: RegisterStateRef) {
        error!("serror_lower");
        system_off::<Hvc>().unwrap();
    }
}

exception_handlers!(Handlers);
