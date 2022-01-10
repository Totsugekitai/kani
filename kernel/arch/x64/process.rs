use core::cell::UnsafeCell;
use crossbeam::atomic::AtomicCell;

#[repr(C, packed)]
pub struct Process {
    rsp: UnsafeCell<u64>,
    pub fsbase: AtomicCell<u64>,
    pub xsave: Option<u64>,
}
