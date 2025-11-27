mod structs;

use raw_cpuid::CpuId;
use x86::vmx::VmFail;
use x86_64::registers::control::{Efer, EferFlags};

use self::structs::SvmRegion;
use crate::arch::x86_64::svm::structs::{FeatureControl, FeatureControlFlags};
use crate::error::RvmResult;
use crate::hal::RvmHal;

pub use self::SvmPerCpuState as ArchPerCpuState;

pub fn has_hardware_support() -> bool {
    if let Some(feature) = CpuId::new().get_svm_info() {
        true
    } else {
        false
    }
}

pub struct SvmPerCpuState<H: RvmHal> {
    svm_region: SvmRegion<H>,
}

impl<H: RvmHal> SvmPerCpuState<H> {
    pub const fn new() -> Self {
        Self {
            svm_region: unsafe { SvmRegion::uninit() },
        }
    }

    pub fn is_enabled(&self) -> bool {
        Efer::read().contains(EferFlags::SECURE_VIRTUAL_MACHINE_ENABLE)
    }

    pub fn hardware_enable(&mut self) -> RvmResult {
        if !has_hardware_support() {
            return rvm_err!(Unsupported, "CPU does not support feature SVM");
        }
        if self.is_enabled() {
            return rvm_err!(ResourceBusy, "SVM is already turned on");
        }

        let vm_cr = FeatureControl::read();
        let disabled = vm_cr.contains(FeatureControlFlags::SVMDIS);
        if (disabled) {
            if let Some(feature) = CpuId::new().get_svm_info() {
                if !feature.has_svm_lock() {
                    return rvm_err!(Unsupported, "SVM disabled by BIOS and not unlockable");
                } else {
                    return rvm_err!(Unsupported, "SVM diabled by BIOS and locked, platform key required");
                }
            } else {
                return rvm_err!(Unsupported, "SVM disabled by BIOS");
            }
        }

        self.svm_region = SvmRegion::new()?;

        unsafe {
            // Enable SVM by setting the SVME bit.
            Efer::write(Efer::read() | EferFlags::SECURE_VIRTUAL_MACHINE_ENABLE);
            // We do not need the process `VMXON` in VMX.
        }
        info!("[RVM] successed to turn on SVM.");

        Ok(())
    }

    pub fn hardware_disable(&mut self) -> RvmResult {
        if !self.is_enabled() {
            return rvm_err!(BadState, "SVM is not enabled");
        }

        unsafe {
            // Disable SVM by resetting the SVME bit.
            Efer::update(|efer| efer.remove(EferFlags::SECURE_VIRTUAL_MACHINE_ENABLE));
        };
        info!("[RVM] successed to turn off SVM.");

        self.svm_region = unsafe { SvmRegion::uninit() };
        Ok(())
    }
}
