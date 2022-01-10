use crossbeam::atomic::AtomicCell;

#[repr(C)]
#[derive(Debug)]
pub struct Process {
    rsp: AtomicCell<u64>,
    cr3: AtomicCell<u64>,
}

impl Process {
    pub fn new(stack_top: u64, cr3: u64) -> Self {
        Self {
            rsp: AtomicCell::new(stack_top),
            cr3: AtomicCell::new(cr3),
        }
    }

    pub fn push(&self, value: u64) {
        unsafe {
            self.rsp.fetch_sub(8);
            *self.rsp.as_ptr() = value;
        }
    }

    pub fn pop(&self) -> u64 {
        let value = unsafe { *(self.rsp.as_ptr()) };
        self.rsp.fetch_add(8);
        value
    }
}
