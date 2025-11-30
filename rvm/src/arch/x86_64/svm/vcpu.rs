use core::fmt::{Debug, Formatter, Result};

use bit_field::BitField;
use x86::dtables::{self, DescriptorTablePointer};
use x86::segmentation::SegmentSelector;
use x86_64::registers::control::{Cr0, Cr0Flags, Cr3, Cr4, Cr4Flags};

use super::structs::SvmRegion;
use super::SvmPerCpuState;
use crate::arch::{msr::Msr, regs::GeneralRegisters};
use crate::{RvmHal, RvmResult};
use super::vmcb::Vmcb;

/// A virtual CPU within a guest.
#[repr(C)]
pub struct SvmVcpu<H: RvmHal> {
    guest_regs: GeneralRegisters,
    vmcb: SvmRegion<H>,
}

impl<H: RvmHal> SvmVcpu<H> {
    pub(crate) fn new(percpu: &SvmPerCpuState<H>) -> RvmResult<Self> {
        let mut vcpu = Self {
            guest_regs: GeneralRegisters::default(),
            vmcb: SvmRegion::new()?,
        };
        vcpu.setup_vmcb()?;
        info!("[RVM] created SvmVcpu(vmcb: {:#x})", vcpu.vmcb.phys_addr());
        Ok(vcpu)
    }

    pub fn run(&mut self) {}
}

// Implementation of private methods
impl<H: RvmHal> SvmVcpu<H> {
    fn setup_vmcb(&mut self) -> RvmResult {
        let vmcb_pa = self.vmcb.phys_addr();
        let vmcb = unsafe { &mut *(vmcb_pa as *mut Vmcb) };
        self.setup_vmcb_control_area(vmcb)?;
        self.setup_vmcb_save_area(vmcb)?;
        Ok(())
    }

    fn setup_vmcb_control_area(&mut self, vmcb: &mut Vmcb) -> RvmResult {
        // Nothing to do for now.
        Ok(())
    }

    fn setup_vmcb_save_area(&mut self, vmcb: &mut Vmcb) -> RvmResult {
        // Nothing to do for now.
        Ok(())
    }
}

impl<H: RvmHal> Drop for SvmVcpu<H> {
    fn drop(&mut self) {
        info!("[RVM] dropped SvmVcpu(vmcb: {:#x})", self.vmcb.phys_addr());
    }
}

impl<H: RvmHal> Debug for SvmVcpu<H> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let vmcb_pa = self.vmcb.phys_addr();
        let vmcb = unsafe { &mut *(vmcb_pa as *mut Vmcb) };
        let rip = vmcb.save.rip;
        let rsp = vmcb.save.rsp;
        let rflags = vmcb.save.rflags;
        let cr0 = vmcb.save.cr0;
        let cr3 = vmcb.save.cr3;
        let cr4 = vmcb.save.cr4;

        (|| -> RvmResult<Result> {
            Ok(f.debug_struct("SvmVcpu")
                .field("guest_regs", &self.guest_regs)
                .field("rip", &rip)
                .field("rsp", &rsp)
                .field("rflags", &rflags)
                .field("cr0", &cr0)
                .field("cr3", &cr3)
                .field("cr4", &cr4)
                .finish())
        })()
        .unwrap()
    }
}