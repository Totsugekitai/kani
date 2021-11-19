use crate::arch::task::ContextX64;
use crossbeam::queue::ArrayQueue;
use lazy_static::lazy_static;
use spin::mutex::Mutex;

#[derive(Debug, Clone, Copy)]
#[repr(C, align(16))]
pub struct Task {
    pub ctx: ContextX64,
    pub f: fn(),
}

const NUM_TASKS: usize = 100;

lazy_static! {
    static ref TASK_QUEUE: ArrayQueue<Task> = ArrayQueue::new(NUM_TASKS);
    static ref CURRENT_TASK: Mutex<Task> = Mutex::new(Task::empty());
}

impl Task {
    pub fn new(f: fn(), stack: u64) -> Self {
        Self {
            ctx: ContextX64::new(stack),
            f,
        }
    }

    pub fn empty() -> Self {
        Self {
            ctx: ContextX64::new(0),
            f: Task::empty_fn,
        }
    }

    fn empty_fn() {}

    pub fn start(&self) {
        (self.f)();
    }

    pub fn register(self) {
        push_task_queue(self);
    }
}

fn push_task_queue(task: Task) {
    TASK_QUEUE.push(task);
}

fn select_next_task() -> Option<Task> {
    TASK_QUEUE.pop()
}

fn switch_context(current_ctx: &ContextX64, next_ctx: &ContextX64) {}

pub fn switch_task() {
    let current_task = *CURRENT_TASK.lock();
    let next_task = if let Some(t) = select_next_task() {
        t
    } else {
        current_task
    };
    push_task_queue(current_task);

    switch_context(&current_task.ctx, &next_task.ctx);
}
