#![allow(dead_code)]
#![deny(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(clippy::upper_case_acronyms)]

use crate::{arch::msr::Msr, RvmResult};

/// Adapted from QEMU: target/i386/svm.h
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct Vmcb {
    pub control: VmcbControlArea,
    pub save: VmcbSaveArea,
}

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct VmcbControlArea {
    pub intercept_cr_read: u16,
    pub intercept_cr_write: u16,
    pub intercept_dr_read: u16,
    pub intercept_dr_write: u16,
    pub intercept_exceptions: u32,
    pub intercept: u64,
    pub reserved_1: [u8; 44],
    pub iopm_base_pa: u64,
    pub msrpm_base_pa: u64,
    pub tsc_offset: u64,
    pub asid: u32,
    pub tlb_ctl: u8,
    pub reserved_2: [u8; 3],
    pub int_ctl: u32,
    pub int_vector: u32,
    pub int_state: u32,
    pub reserved_3: [u8; 4],
    pub exit_code: u64,
    pub exit_info_1: u64,
    pub exit_info_2: u64,
    pub exit_int_info: u32,
    pub exit_int_info_err: u32,
    pub nested_ctl: u64,
    pub reserved_4: [u8; 16],
    pub event_inj: u32,
    pub event_inj_err: u32,
    pub nested_cr3: u64,
    pub lbr_ctl: u64,
    pub reserved_5: [u8; 832],
}

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct VmcbSeg {
    pub selector: u16,
    pub attrib: u16,
    pub limit: u32,
    pub base: u64,
}

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct VmcbSaveArea {
    pub es: VmcbSeg,
    pub cs: VmcbSeg,
    pub ss: VmcbSeg,
    pub ds: VmcbSeg,
    pub fs: VmcbSeg,
    pub gs: VmcbSeg,
    pub gdtr: VmcbSeg,
    pub ldtr: VmcbSeg,
    pub idtr: VmcbSeg,
    pub tr: VmcbSeg,
    pub reserved_1: [u8; 43],
    pub cpl: u8,
    pub reserved_2: [u8; 4],
    pub efer: u64,
    pub reserved_3: [u8; 112],
    pub cr4: u64,
    pub cr3: u64,
    pub cr0: u64,
    pub dr7: u64,
    pub dr6: u64,
    pub rflags: u64,
    pub rip: u64,
    pub reserved_4: [u8; 88],
    pub rsp: u64,
    pub reserved_5: [u8; 24],
    pub rax: u64,
    pub star: u64,
    pub lstar: u64,
    pub cstar: u64,
    pub sfmask: u64,
    pub kernel_gs_base: u64,
    pub sysenter_cs: u64,
    pub sysenter_esp: u64,
    pub sysenter_eip: u64,
    pub cr2: u64,
    pub reserved_6: [u8; 32],
    pub g_pat: u64,
    pub dbgctl: u64,
    pub br_from: u64,
    pub br_to: u64,
    pub last_excp_from: u64,
    pub last_excp_to: u64,
}