use crate::task::Context;

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
    fn x64_switch_context(prev_ctx: *const ContextX64, next_ctx: *const ContextX64);
}

impl Context<Self> for ContextX64 {
    fn new() -> Self {
        Self {
            cr3: 0,
            rip: 0,
            rflags: 0,
            reserved: 0,
            cs: 0,
            ss: 0,
            fs: 0,
            gs: 0,
            rax: 0,
            rbx: 0,
            rcx: 0,
            rdx: 0,
            rdi: 0,
            rsi: 0,
            rsp: 0,
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
        }
    }
    fn switch_context(&self, next_ctx: &Self) {
        unsafe { x64_switch_context(self as *const ContextX64, next_ctx as *const ContextX64) }
    }
}
