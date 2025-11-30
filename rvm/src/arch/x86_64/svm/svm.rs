use core::arch::asm;

pub unsafe fn vmrun(vmcb_pa: usize) -> ! {
    asm!(
        "vmrun rax",
        in("rax") vmcb_pa,
        options(noreturn)
    );
}

pub unsafe fn vmload(vmcb_pa: usize) {
    asm!(
        "vmload rax",
        in("rax") vmcb_pa
    );
}

pub unsafe fn vmsave(vmcb_pa: usize) {
    asm!(
        "vmsave rax",
        in("rax") vmcb_pa
    );
}
