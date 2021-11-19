use crate::arch::task::ContextX64;
use crossbeam::queue::ArrayQueue;
use lazy_static::lazy_static;
use spin::mutex::Mutex;

#[derive(Debug, Clone, Copy)]
#[repr(C, align(16))]
pub struct Task {
    pub ctx: ContextX64,
}

const NUM_TASKS: usize = 100;

lazy_static! {
    static ref TASK_QUEUE: ArrayQueue<Task> = ArrayQueue::new(NUM_TASKS);
    static ref CURRENT_TASK: Mutex<Task> = Mutex::new(Task::empty());
}

impl Task {
    pub fn new(f: u64, stack_bottom: u64) -> Self {
        Self {
            ctx: ContextX64::new(f, stack_bottom),
        }
    }

    pub fn empty() -> Self {
        Self {
            ctx: ContextX64::new(Task::empty_fn as u64, 0),
        }
    }

    fn empty_fn() {}

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

pub fn switch_task(current_task: &Task, next_task: &Task) {
    // let current_task = *CURRENT_TASK.lock();
    // let next_task = if let Some(t) = select_next_task() {
    //     t
    // } else {
    //     current_task
    // };
    // push_task_queue(current_task);

    unsafe {
        crate::arch::x64::task::switch_context(&current_task.ctx, &next_task.ctx);
    }
}
