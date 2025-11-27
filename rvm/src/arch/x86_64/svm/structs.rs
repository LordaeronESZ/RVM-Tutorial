use bit_field::BitField;
use bitflags::bitflags;

use crate::arch::msr::{Msr, MsrReadWrite};
use crate::mm::PhysFrame;
use crate::{HostPhysAddr, RvmHal, RvmResult};

/// VMCB region in 4K size.
#[derive(Debug)]
pub struct SvmRegion<H: RvmHal> {
    frame: PhysFrame<H>,
}

impl<H: RvmHal> SvmRegion<H> {
    pub const unsafe fn uninit() -> Self {
        Self {
            frame: PhysFrame::uninit(),
        }
    }

    pub fn new() -> RvmResult<Self> {
        let frame = PhysFrame::alloc_zero()?;
        Ok(Self { frame })
    }

    pub fn phys_addr(&self) -> HostPhysAddr {
        self.frame.start_paddr()
    }
}

bitflags! {
    /// VM_CR flags (comment from APM Vol2 15.30 SVM Related MSRs).
    pub struct FeatureControlFlags: u64 {
        /// If set, disables the external hardware debug port and certain internal debug features.
        const DPD = 1 << 0;
        /// If set, non-intercepted INIT signals are converted into an #SX exception.
        const R_INIT = 1 << 1;
        /// If set, disables A20 masking.
        const DIS_A20M = 1 << 3;
        /// When this bit is set, writes to LOCK and SVMDIS are silently ignored.
        /// When this bit is clear, VM_CR bits 3 and 4 can be written. Once set,
        /// LOCK can only be cleared using the SVM_KEY MSR (See APM Section 15.31.)
        /// This bit is not affected by INIT or SKINIT.
        const LOCK = 1 << 4;
        /// When this bit is set, writes to EFER treat the SVME bit as MBZ.
        /// When this bit is clear, EFER.SVME can be written normally.
        /// This bit does not prevent CPUID from reporting that SVM is available.
        /// Setting SVMDIS while EFER.SVME is 1 generates a #GP fault,
        /// regardless of the current state of VM_CR.LOCK. This bit is not affected by SKINIT.
        /// It is cleared by INIT when LOCK is cleared to 0; otherwise, it is not affected.
        const SVMDIS = 1 << 5;
   }
}

/// Control Features in AMD 64 Processor.
pub struct FeatureControl;

impl MsrReadWrite for FeatureControl {
    const MSR: Msr = Msr::VM_CR;
}

impl FeatureControl {
    /// Read the current VM_CR flags.
    pub fn read() -> FeatureControlFlags {
        FeatureControlFlags::from_bits_truncate(Self::read_raw())
    }

    /// Write VM_CR flags, preserving reserved values.
    pub fn write(flags: FeatureControlFlags) {
        let old_value = Self::read_raw();
        let reserved = old_value & !(FeatureControlFlags::all().bits());
        let new_value = reserved | flags.bits();
        unsafe { Self::write_raw(new_value) };
    }
}