use x86_64::instructions::segmentation::Segment;

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct ContextX64 {
    pub cr3: u64,
    pub rip: u64,
    pub rflags: u64,
    reserved: u64,
    pub cs: u64,
    pub ss: u64,
    pub fs: u64,
    pub gs: u64,
    pub rax: u64,
    pub rbx: u64,
    pub rcx: u64,
    pub rdx: u64,
    pub rdi: u64,
    pub rsi: u64,
    pub rsp: u64,
    pub rbp: u64,
    pub r8: u64,
    pub r9: u64,
    pub r10: u64,
    pub r11: u64,
    pub r12: u64,
    pub r13: u64,
    pub r14: u64,
    pub r15: u64,
    pub fxsave: [u8; 512],
}

extern "C" {
    fn x64_switch_context(next_ctx: *const ContextX64, prev_ctx: *const ContextX64);
}

impl ContextX64 {
    pub fn new(f: u64, stack_bottom: u64) -> Self {
        let mut ctx = Self {
            cr3: x86_64::registers::control::Cr3::read()
                .0
                .start_address()
                .as_u64(),
            rip: f,
            rflags: 0x202, // from osbook_day13
            reserved: 0,
            cs: x86_64::registers::segmentation::CS::get_reg().0 as u64,
            ss: x86_64::registers::segmentation::SS::get_reg().0 as u64,
            fs: 0,
            gs: 0,
            rax: 0,
            rbx: 0,
            rcx: 0,
            rdx: 0,
            rdi: 0,
            rsi: 0,
            rsp: (stack_bottom & 0xffff_ffff_ffff_fff0) - 8,
            rbp: 0,
            r8: 0,
            r9: 0,
            r10: 0,
            r11: 0,
            r12: 0,
            r13: 0,
            r14: 0,
            r15: 0,
            fxsave: [0; 512],
        };
        ctx.fxsave[24] = 0x80;
        ctx.fxsave[24] = 0x1f;
        ctx
    }
}

pub unsafe fn switch_context(current_ctx: &ContextX64, next_ctx: &ContextX64) {
    x64_switch_context(
        next_ctx as *const ContextX64,
        current_ctx as *const ContextX64,
    )
}
