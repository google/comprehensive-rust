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

#![allow(unused)]

use bitflags::bitflags;
use core::arch::asm;
use core::fmt::{self, Debug, Formatter};
use core::hint::spin_loop;
use core::mem::size_of;
use core::ptr::{addr_of, addr_of_mut};
use log::trace;

/// Reads and returns the value of the given aarch64 system register.
macro_rules! read_sysreg {
    ($name:ident) => {
        {
            let mut value: u64;
            ::core::arch::asm!(
                concat!("mrs {value:x}, ", ::core::stringify!($name)),
                value = out(reg) value,
                options(nomem, nostack),
            );
            value
        }
    }
}

/// Writes the given value to the given aarch64 system register.
macro_rules! write_sysreg {
    ($name:ident, $value:expr) => {
        {
            let v: u64 = $value;
            ::core::arch::asm!(
                concat!("msr ", ::core::stringify!($name), ", {value:x}"),
                value = in(reg) v,
                options(nomem, nostack),
            )
        }
    }
}

/// The offset in bytes from `RD_base` to `SGI_base`.
const SGI_OFFSET: usize = 0x10000;

/// An interrupt ID.
#[derive(Copy, Clone, Eq, Ord, PartialOrd, PartialEq)]
pub struct IntId(u32);

impl IntId {
    /// The ID of the first Software Generated Interrupt.
    const SGI_START: u32 = 0;

    /// The ID of the first Private Peripheral Interrupt.
    const PPI_START: u32 = 16;

    /// The ID of the first Shared Peripheral Interrupt.
    const SPI_START: u32 = 32;

    /// The first special interrupt ID.
    const SPECIAL_START: u32 = 1020;

    /// Returns the interrupt ID for the given Software Generated Interrupt.
    pub const fn sgi(sgi: u32) -> Self {
        assert!(sgi < Self::PPI_START);
        Self(Self::SGI_START + sgi)
    }

    /// Returns the interrupt ID for the given Private Peripheral Interrupt.
    pub const fn ppi(ppi: u32) -> Self {
        assert!(ppi < Self::SPI_START - Self::PPI_START);
        Self(Self::PPI_START + ppi)
    }

    /// Returns the interrupt ID for the given Shared Peripheral Interrupt.
    pub const fn spi(spi: u32) -> Self {
        assert!(spi < Self::SPECIAL_START);
        Self(Self::SPI_START + spi)
    }

    /// Returns whether this interrupt ID is for a Software Generated Interrupt.
    fn is_sgi(self) -> bool {
        self.0 < Self::PPI_START
    }

    /// Returns whether this interrupt ID is private to a core, i.e. it is an
    /// SGI or PPI.
    fn is_private(self) -> bool {
        self.0 < Self::SPI_START
    }
}

impl Debug for IntId {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        if self.0 < Self::PPI_START {
            write!(f, "SGI {}", self.0 - Self::SGI_START)
        } else if self.0 < Self::SPI_START {
            write!(f, "PPI {}", self.0 - Self::PPI_START)
        } else if self.0 < Self::SPECIAL_START {
            write!(f, "SPI {}", self.0 - Self::SPI_START)
        } else {
            write!(f, "Special IntId {}", self.0)
        }
    }
}

impl From<IntId> for u32 {
    fn from(intid: IntId) -> Self {
        intid.0
    }
}

bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq)]
    struct GicdCtlr: u32 {
        const RWP = 1 << 31;
        const nASSGIreq = 1 << 8;
        const E1NWF = 1 << 7;
        const DS = 1 << 6;
        const ARE_NS = 1 << 5;
        const ARE_S = 1 << 4;
        const EnableGrp1S = 1 << 2;
        const EnableGrp1NS = 1 << 1;
        const EnableGrp0 = 1 << 0;
    }
}

/// GIC Distributor registers.
#[repr(C, align(8))]
struct GICD {
    /// Distributor control register.
    ctlr: GicdCtlr,
    /// Interrupt controller type register.
    typer: u32,
    /// Distributor implementer identification register.
    iidr: u32,
    /// Interrupt controller type register 2.
    typer2: u32,
    /// Error reporting status register.
    statusr: u32,
    _reserved0: [u32; 3],
    /// Implementation defined registers.
    implementation_defined: [u32; 8],
    /// Set SPI register.
    setspi_nsr: u32,
    _reserved1: u32,
    /// Clear SPI register.
    clrspi_nsr: u32,
    _reserved2: u32,
    /// Set SPI secure register.
    setspi_sr: u32,
    _reserved3: u32,
    /// Clear SPI secure register.
    clrspi_sr: u32,
    _reserved4: [u32; 9],
    /// Interrupt group registers.
    igroupr: [u32; 32],
    /// Interrupt set-enable registers.
    isenabler: [u32; 32],
    /// Interrupt clear-enable registers.
    icenabler: [u32; 32],
    /// Interrupt set-pending registers.
    ispendr: [u32; 32],
    /// Interrupt clear-pending registers.
    icpendr: [u32; 32],
    /// Interrupt set-active registers.
    isactiver: [u32; 32],
    /// Interrupt clear-active registers.
    icactiver: [u32; 32],
    /// Interrupt priority registers.
    ipriorityr: [u8; 1024],
    /// Interrupt processor targets registers.
    itargetsr: [u32; 256],
    /// Interrupt configuration registers.
    icfgr: [u32; 64],
    /// Interrupt group modifier registers.
    igrpmodr: [u32; 32],
    _reserved5: [u32; 32],
    /// Non-secure access control registers.
    nsacr: [u32; 64],
    /// Software generated interrupt register.
    sigr: u32,
    _reserved6: [u32; 3],
    /// SGI clear-pending registers.
    cpendsgir: [u32; 4],
    /// SGI set-pending registers.
    spendsgir: [u32; 4],
    _reserved7: [u32; 20],
    /// Non-maskable interrupt registers.
    inmir: [u32; 32],
    /// Interrupt group registers for extended SPI range.
    igroupr_e: [u32; 32],
    _reserved8: [u32; 96],
    /// Interrupt set-enable registers for extended SPI range.
    isenabler_e: [u32; 32],
    _reserved9: [u32; 96],
    /// Interrupt clear-enable registers for extended SPI range.
    icenabler_e: [u32; 32],
    _reserved10: [u32; 96],
    /// Interrupt set-pending registers for extended SPI range.
    ispendr_e: [u32; 32],
    _reserved11: [u32; 96],
    /// Interrupt clear-pending registers for extended SPI range.
    icpendr_e: [u32; 32],
    _reserved12: [u32; 96],
    /// Interrupt set-active registers for extended SPI range.
    isactive_e: [u32; 32],
    _reserved13: [u32; 96],
    /// Interrupt clear-active registers for extended SPI range.
    icactive_e: [u32; 32],
    _reserved14: [u32; 224],
    /// Interrupt priority registers for extended SPI range.
    ipriorityr_e: [u8; 1024],
    _reserved15: [u32; 768],
    /// Extended SPI configuration registers.
    icfgr_e: [u32; 64],
    _reserved16: [u32; 192],
    /// Interrupt group modifier registers for extended SPI range.
    igrpmodr_e: [u32; 32],
    _reserved17: [u32; 96],
    /// Non-secure access control registers for extended SPI range.
    nsacr_e: [u32; 32],
    _reserved18: [u32; 288],
    /// Non-maskable interrupt registers for extended SPI range.
    inmr_e: [u32; 32],
    _reserved19: [u32; 2400],
    /// Interrupt routing registers.
    irouter: [u32; 1975],
    _reserved20: [u32; 9],
    /// Interrupt routing registers for extended SPI range.
    irouter_e: [u32; 2048],
    _reserved21: [u32; 2048],
    /// Implementation defined registers.
    implementation_defined2: [u32; 4084],
    /// ID registers.
    id_registers: [u32; 12],
}

bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq)]
    struct Waker: u32 {
        const CHILDREN_ASLEEP = 1 << 2;
        const PROCESSOR_SLEEP = 1 << 1;
    }
}

/// GIC Redistributor registers.
#[repr(C, align(8))]
struct GICR {
    /// Redistributor control register.
    ctlr: u32,
    /// Implementer identification register.
    iidr: u32,
    /// Redistributor type register.
    typer: u64,
    /// Error reporting status register.
    statusr: u32,
    /// Redistributor wake register.
    waker: Waker,
    /// Report maximum PARTID and PMG register.
    mpamidr: u32,
    /// Set PARTID and PMG register.
    partidr: u32,
    /// Implementation defined registers.
    implementation_defined1: [u32; 8],
    /// Set LPI pending register.
    setlprir: u64,
    /// Clear LPI pending register.
    clrlpir: u64,
    _reserved0: [u32; 8],
    /// Redistributor properties base address register.
    propbaser: u64,
    /// Redistributor LPI pending table base address register.
    pendbaser: u64,
    _reserved1: [u32; 8],
    /// Redistributor invalidate LPI register.
    invlpir: u64,
    _reserved2: u64,
    /// Redistributor invalidate all register.
    invallr: u64,
    _reserved3: u64,
    /// Redistributor synchronize register.
    syncr: u32,
    _reserved4: [u32; 15],
    /// Implementation defined registers.
    implementation_defined2: u64,
    _reserved5: u64,
    /// Implementation defined registers.
    implementation_defined3: u64,
    _reserved6: [u32; 12218],
    /// Implementation defined registers.
    implementation_defined4: [u32; 4084],
    /// ID registers.
    id_registers: [u32; 12],
}

/// GIC Redistributor SGI and PPI registers.
#[repr(C, align(8))]
struct SGI {
    _reserved0: [u32; 32],
    /// Interrupt group register 0.
    igroupr0: u32,
    /// Interrupt group registers for extended PPI range.
    igroupr_e: [u32; 2],
    _reserved1: [u32; 29],
    /// Interrupt set-enable register 0.
    isenabler0: u32,
    /// Interrupt set-enable registers for extended PPI range.
    isenabler_e: [u32; 2],
    _reserved2: [u32; 29],
    /// Interrupt clear-enable register 0.
    icenabler0: u32,
    /// Interrupt clear-enable registers for extended PPI range.
    icenabler_e: [u32; 2],
    _reserved3: [u32; 29],
    /// Interrupt set-pending register 0.
    ispendr0: u32,
    /// Interrupt set-pending registers for extended PPI range.
    ispendr_e: [u32; 2],
    _reserved4: [u32; 29],
    /// Interrupt clear-pending register 0.
    icpendr0: u32,
    /// Interrupt clear-pending registers for extended PPI range.
    icpendr_e: [u32; 2],
    _reserved5: [u32; 29],
    /// Interrupt set-active register 0.
    isactiver0: u32,
    /// Interrupt set-active registers for extended PPI range.
    isactive_e: [u32; 2],
    _reserved6: [u32; 29],
    /// Interrupt clear-active register 0.
    icactiver0: u32,
    /// Interrupt clear-active registers for extended PPI range.
    icactive_e: [u32; 2],
    _reserved7: [u32; 29],
    /// Interrupt priority registers.
    ipriorityr: [u8; 32],
    /// Interrupt priority registers for extended PPI range.
    ipriorityr_e: [u8; 64],
    _reserved8: [u32; 488],
    /// SGI configuration register, PPI configuration register and extended PPI
    /// configuration registers.
    icfgr: [u32; 6],
    _reserved9: [u32; 58],
    /// Interrupt group modifier register 0.
    igrpmodr0: u32,
    /// Interrupt group modifier registers for extended PPI range.
    igrpmodr_e: [u32; 2],
    _reserved10: [u32; 61],
    /// Non-secure access control register.
    nsacr: u32,
    _reserved11: [u32; 95],
    /// Non-maskable interrupt register for PPIs.
    inmir0: u32,
    /// Non-maskable interrupt register for extended PPIs.
    inmir_e: [u32; 31],
    _reserved12: [u32; 11264],
    /// Implementation defined registers.
    implementation_defined: [u32; 4084],
    _reserved13: [u32; 12],
}

/// Driver for an Arm Generic Interrupt Controller version 3 (or 4).
#[derive(Debug)]
pub struct GicV3 {
    gicd: *mut GICD,
    gicr: *mut GICR,
    sgi: *mut SGI,
}

impl GicV3 {
    /// Constructs a new instance of the driver for a GIC with the given
    /// distributor and redistributor base addresses.
    ///
    /// # Safety
    ///
    /// The given base addresses must point to the GIC distributor and
    /// redistributor registers respectively. These regions must be mapped into
    /// the address space of the process as device memory, and not have any
    /// other aliases, either via another instance of this driver or otherwise.
    pub unsafe fn new(gicd: *mut u64, gicr: *mut u64) -> Self {
        Self {
            gicd: gicd as _,
            gicr: gicr as _,
            sgi: gicr.wrapping_add(SGI_OFFSET / size_of::<u64>()) as _,
        }
    }

    /// Initialises the GIC.
    pub fn setup(&mut self) {
        // Safe because writing to this system register doesn't access memory in
        // any way.
        unsafe {
            // Enable system register access.
            write_sysreg!(icc_sre_el1, 0x01);
        }

        // Safe because we know that `self.gicr` is a valid and unique pointer
        // to the registers of a GIC redistributor interface.
        unsafe {
            // Mark this CPU core as awake, and wait until the GIC wakes up
            // before continuing.
            let mut waker = addr_of!((*self.gicr).waker).read_volatile();
            trace!("WAKER: {:?}", waker);
            waker -= Waker::PROCESSOR_SLEEP;
            addr_of_mut!((*self.gicr).waker).write_volatile(waker);

            while addr_of!((*self.gicr).waker)
                .read_volatile()
                .contains(Waker::CHILDREN_ASLEEP)
            {
                spin_loop();
            }
        }

        // Safe because accessing this system register doesn't access memory in
        // any way.
        unsafe {
            trace!("ICC_CTLR_EL1={:#x}", read_sysreg!(icc_ctlr_el1));
            // Disable use of `ICC_PMR_EL1` as a hint for interrupt
            // distribution, configure a write to an EOI register to also
            // deactivate the interrupt, and configure preemption groups for
            // group 0 and group 1 interrupts separately.
            write_sysreg!(icc_ctlr_el1, 0);
            trace!("ICC_CTLR_EL1={:#x}", read_sysreg!(icc_ctlr_el1));
        }

        // Safe because we know that `self.gicd` is a valid and unique pointer
        // to the registers of a GIC distributor interface.
        unsafe {
            // Enable affinity routing and non-secure group 1 interrupts.
            addr_of_mut!((*self.gicd).ctlr)
                .write_volatile(GicdCtlr::ARE_S | GicdCtlr::EnableGrp1NS);
        }

        // Safe because we know that `self.gicd` is a valid and unique pointer
        // to the registers of a GIC distributor interface, and `self.sgi` to
        // the SGI and PPI registers of a GIC redistributor interface.
        unsafe {
            // Put all SGIs and PPIs into non-secure group 1.
            addr_of_mut!((*self.sgi).igroupr0).write_volatile(0xffffffff);
            // Put all SPIs into non-secure group 1.
            for i in 0..32 {
                addr_of_mut!((*self.gicd).igroupr[i]).write_volatile(0xffffffff);
            }
        }

        // Safe because writing to this system register doesn't access memory in
        // any way.
        unsafe {
            // Enable non-secure group 1.
            write_sysreg!(icc_igrpen1_el1, 0x00000001);
        }
    }

    pub fn gicd_pending(&self, index: usize) -> u32 {
        unsafe { addr_of!((*self.gicd).ispendr[index]).read_volatile() }
    }

    pub fn gicr_pending(&self) -> u32 {
        unsafe { addr_of!((*self.sgi).ispendr0).read_volatile() }
    }

    pub fn gicd_active(&self, index: usize) -> u32 {
        unsafe { addr_of!((*self.gicd).isactiver[index]).read_volatile() }
    }

    pub fn gicr_active(&self) -> u32 {
        unsafe { addr_of!((*self.sgi).isactiver0).read_volatile() }
    }

    /// Enables or disables the interrupt with the given ID.
    pub fn enable_interrupt(&mut self, intid: IntId, enable: bool) {
        let index = (intid.0 / 32) as usize;
        let bit = 1 << (intid.0 % 32);

        // Safe because we know that `self.gicd` is a valid and unique pointer
        // to the registers of a GIC distributor interface, and `self.sgi` to
        // the SGI and PPI registers of a GIC redistributor interface.
        unsafe {
            if enable {
                addr_of_mut!((*self.gicd).isenabler[index]).write_volatile(bit);
                if intid.is_private() {
                    addr_of_mut!((*self.sgi).isenabler0).write_volatile(bit);
                }
            } else {
                addr_of_mut!((*self.gicd).icenabler[index]).write_volatile(bit);
                if intid.is_private() {
                    addr_of_mut!((*self.sgi).icenabler0).write_volatile(bit);
                }
            }
        }
    }

    /// Enables all interrupts.
    pub fn enable_all_interrupts(&mut self, enable: bool) {
        for i in 0..32 {
            // Safe because we know that `self.gicd` is a valid and unique
            // pointer to the registers of a GIC distributor interface.
            unsafe {
                if enable {
                    addr_of_mut!((*self.gicd).isenabler[i]).write_volatile(0xffffffff);
                } else {
                    addr_of_mut!((*self.gicd).icenabler[i]).write_volatile(0xffffffff);
                }
            }
        }
        // Safe because we know that `self.sgi` is a valid and unique pointer
        // to the SGI and PPI registers of a GIC redistributor interface.
        unsafe {
            if enable {
                addr_of_mut!((*self.sgi).isenabler0).write_volatile(0xffffffff);
            } else {
                addr_of_mut!((*self.sgi).icenabler0).write_volatile(0xffffffff);
            }
        }
    }

    /// Sets the priority mask for the current CPU core.
    ///
    /// Only interrupts with a higher priority (numerically lower) will be
    /// signalled.
    pub fn set_priority_mask(min_priority: u8) {
        // Safe because writing to this system register doesn't access memory in
        // any way.
        unsafe {
            write_sysreg!(icc_pmr_el1, min_priority.into());
        }
    }

    /// Sets the priority of the interrupt with the given ID.
    ///
    /// Note that lower numbers correspond to higher priorities; i.e. 0 is the
    /// highest priority, and 255 is the lowest.
    pub fn set_interrupt_priority(&mut self, intid: IntId, priority: u8) {
        // Safe because we know that `self.gicd` is a valid and unique pointer
        // to the registers of a GIC distributor interface, and `self.sgi` to
        // the SGI and PPI registers of a GIC redistributor interface.
        unsafe {
            // Affinity routing is enabled, so use the GICR for SGIs and PPIs.
            if intid.is_private() {
                addr_of_mut!((*self.sgi).ipriorityr[intid.0 as usize])
                    .write_volatile(priority);
            } else {
                addr_of_mut!((*self.gicd).ipriorityr[intid.0 as usize])
                    .write_volatile(priority);
            }
        }
    }

    /// Configures the trigger type for the interrupt with the given ID.
    pub fn set_trigger(&mut self, intid: IntId, trigger: Trigger) {
        let index = (intid.0 / 16) as usize;
        let bit = 1 << (((intid.0 % 16) * 2) + 1);

        // Safe because we know that `self.gicd` is a valid and unique pointer
        // to the registers of a GIC distributor interface, and `self.sgi` to
        // the SGI and PPI registers of a GIC redistributor interface.
        unsafe {
            // Affinity routing is enabled, so use the GICR for SGIs and PPIs.
            let register = if intid.is_private() {
                addr_of_mut!((*self.sgi).icfgr[index])
            } else {
                addr_of_mut!((*self.gicd).icfgr[index])
            };
            let v = register.read_volatile();
            register.write_volatile(match trigger {
                Trigger::Edge => v | bit,
                Trigger::Level => v & !bit,
            });
        }
    }

    /// Sends a software-generated interrupt (SGI) to the given cores.
    pub fn send_sgi(intid: IntId, target: SgiTarget) {
        assert!(intid.is_sgi());

        let sgi_value = match target {
            SgiTarget::All => {
                let irm = 0b1;
                (u64::from(intid.0 & 0x0f) << 24) | (irm << 40)
            }
            SgiTarget::List {
                affinity3,
                affinity2,
                affinity1,
                target_list,
            } => {
                let irm = 0b0;
                u64::from(target_list)
                    | (u64::from(affinity1) << 16)
                    | (u64::from(intid.0 & 0x0f) << 24)
                    | (u64::from(affinity2) << 32)
                    | (irm << 40)
                    | (u64::from(affinity3) << 48)
            }
        };

        // Safe because writing to this system register doesn't access memory in
        // any way.
        unsafe {
            write_sysreg!(icc_sgi1r_el1, sgi_value);
        }
    }

    /// Gets the ID of the highest priority signalled interrupt, and
    /// acknowledges it.
    ///
    /// Returns `None` if there is no pending interrupt of sufficient priority.
    pub fn get_and_acknowledge_interrupt() -> Option<IntId> {
        // Safe because reading this system register doesn't access memory in
        // any way.
        let intid = unsafe { read_sysreg!(icc_iar1_el1) } as u32;
        if intid == IntId::SPECIAL_START {
            None
        } else {
            Some(IntId(intid))
        }
    }

    /// Informs the interrupt controller that the CPU has completed processing
    /// the given interrupt. This drops the interrupt priority and deactivates
    /// the interrupt.
    pub fn end_interrupt(intid: IntId) {
        // Safe because writing to this system register doesn't access memory in
        // any way.
        unsafe { write_sysreg!(icc_eoir1_el1, intid.0.into()) }
    }
}

/// The trigger configuration for an interrupt.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Trigger {
    /// The interrupt is edge triggered.
    Edge,
    /// The interrupt is level triggered.
    Level,
}

/// The target specification for a software-generated interrupt.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum SgiTarget {
    /// The SGI is routed to all CPU cores except the current one.
    All,
    /// The SGI is routed to the CPU cores matching the given affinities and list.
    List {
        affinity3: u8,
        affinity2: u8,
        affinity1: u8,
        target_list: u16,
    },
}

/// Disables debug, SError, IRQ and FIQ exceptions.
pub fn irq_disable() {
    // Safe because writing to this system register doesn't access memory in any
    // way.
    unsafe {
        asm!("msr DAIFSet, #0xf", options(nomem, nostack));
    }
}

/// Enables debug, SError, IRQ and FIQ exceptions.
pub fn irq_enable() {
    // Safe because writing to this system register doesn't access memory in any
    // way.
    unsafe {
        asm!("msr DAIFClr, #0xf", options(nomem, nostack));
    }
}

/// Waits for an interrupt.
pub fn wfi() {
    // Safe because this doesn't access memory in any way.
    unsafe {
        asm!("wfi", options(nomem, nostack));
    }
}
