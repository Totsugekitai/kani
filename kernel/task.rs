use crate::arch::task::ContextX64;
use core::sync::atomic::AtomicU64;
use kani_lib::linked_list::LinkedList;
use lazy_static::lazy_static;

#[cfg(target_arch = "x86_64")]
#[derive(Debug, Clone, Copy)]
#[repr(C, align(16))]
pub struct Task {
    pub ctx: ContextX64,
    id: u64,
}

impl Task {
    pub fn new(f: u64, stack_bottom: u64) -> Self {
        Self {
            ctx: ContextX64::new(f, stack_bottom),
            id: 0,
        }
    }

    pub fn make_idle_task() -> Self {
        Self {
            ctx: ContextX64::new(Task::idle_fn as u64, 0xdeadbeef),
            id: 1,
        }
    }

    fn idle_fn() {
        loop {
            x86_64::instructions::hlt();
        }
    }

    pub fn exec() {}
}

lazy_static! {
    pub static ref IDLE_TASK: Task = Task::make_idle_task();
    static ref CURRENT_TASK_INDEX: AtomicU64 = AtomicU64::new(0);
    static ref TASK_LIST: LinkedList<Task> = LinkedList::new();
}

pub fn switch_task(current_task: &Task, next_task: &Task) {
    unsafe {
        crate::arch::x64::task::switch_context(&current_task.ctx, &next_task.ctx);
    }
}
