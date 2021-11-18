#[derive(Debug, Clone, Copy)]
#[repr(C, align(16))]
pub struct Task<T: Context> {
    pub ctx: T,
}

impl<T: Context> Task<T> {
    pub fn switch_task(&self) {
        let next_task = select_next_task(self);
        self.ctx.switch_context(&next_task.ctx);
    }

    pub fn new() -> Self {
        Self {
            ctx: Context::new(),
        }
    }
}

pub trait Context<T = Self> {
    fn switch_context(&self, next_ctx: &T);
    fn new() -> Self;
}

fn select_next_task<T: Context<T>>(current: &Task<T>) -> &Task<T> {
    current
}
