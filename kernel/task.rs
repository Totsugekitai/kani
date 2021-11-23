use crate::arch::task::ContextX64;
use alloc::{
    boxed::Box,
    collections::{BTreeMap, VecDeque},
};
use core::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use lazy_static::lazy_static;
use log::info;
use spin::mutex::Mutex;

pub const TASK_INTERVAL: usize = 100;

type Tid = u64;

#[cfg(target_arch = "x86_64")]
#[derive(Debug, Clone, Copy)]
#[repr(C, align(16))]
pub struct Task {
    pub ctx: ContextX64,
    tid: Tid,
}

impl Task {
    pub fn new(f: u64, stack_bottom: u64) -> Self {
        let total = TOTAL_TID.fetch_add(1, Ordering::SeqCst);
        Self {
            ctx: ContextX64::new(f, stack_bottom),
            tid: total,
        }
    }

    pub fn make_idle_task() -> Self {
        Self {
            ctx: ContextX64::new(idle_fn as u64, IDLE_FN_STACK.as_ptr() as u64 + 0x100),
            tid: 0,
        }
    }

    pub fn register(self) {
        let tid = self.tid;
        TASK_QUEUE.lock().insert(tid, self);
        TID_VEC.lock().push_back(tid);
        info!("register {}", tid);
    }
}

fn idle_fn() {
    loop {
        x86_64::instructions::hlt();
    }
}

lazy_static! {
    static ref TOTAL_TID: AtomicU64 = AtomicU64::new(0);
    static ref IDLE_TASK: Task = Task::make_idle_task();
    static ref TID_VEC: Mutex<VecDeque<Tid>> = Mutex::new(VecDeque::new());
    static ref CURRENT_TID: AtomicU64 = AtomicU64::new(0);
    static ref TASK_QUEUE: Mutex<BTreeMap<Tid, Task>> = Mutex::new(BTreeMap::new());
    static ref LOCK_FLAG: AtomicBool = AtomicBool::new(false);
    static ref IDLE_FN_STACK: Box<[u8; 0x100]> = Box::new([0u8; 0x100]);
}

fn select_next_task() -> Tid {
    match TID_VEC.lock().pop_front() {
        Some(tid) => tid,
        None => 0,
    }
}

pub fn switch_next_task() {
    let next_tid = select_next_task();
    let current_tid = CURRENT_TID.swap(next_tid, Ordering::SeqCst);
    let task_queue = TASK_QUEUE.lock();
    info!("{} -> {}", current_tid, next_tid);
    switch_task(
        task_queue.get(&current_tid).unwrap(),
        task_queue.get(&next_tid).unwrap(),
    );
    core::mem::forget(task_queue);
}

fn switch_task(current_task: &Task, next_task: &Task) {
    unsafe {
        crate::arch::x64::task::switch_context(&current_task.ctx, &next_task.ctx);
    }
}
