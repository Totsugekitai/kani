use crate::arch::task::ContextX64;
use core::sync::atomic::AtomicU64;
use lazy_static::lazy_static;

#[cfg(target_arch = "x86_64")]
#[derive(Debug, Clone, Copy)]
#[repr(C, align(16))]
pub struct Task {
    pub ctx: ContextX64,
    idx: u64,
}

impl Task {
    pub fn new(f: u64, stack_bottom: u64) -> Self {
        Self {
            ctx: ContextX64::new(f, stack_bottom),
            idx: 0,
        }
    }
}

const NUM_TASKS: usize = 0x100;
static TASK_ARRAY: [Option<Task>; NUM_TASKS] = [None; NUM_TASKS];

lazy_static! {
    static ref CURRENT_TASK_INDEX: AtomicU64 = AtomicU64::new(0);
}

pub fn switch_task(current_task: &Task, next_task: &Task) {
    unsafe {
        crate::arch::x64::task::switch_context(&current_task.ctx, &next_task.ctx);
    }
}
