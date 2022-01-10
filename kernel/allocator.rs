use crate::arch;
use linked_list_allocator::LockedHeap;

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

#[cfg(target_arch = "x86_64")]
pub unsafe fn init() {
    let (heap_bottom, heap_size) = arch::x64::allocator::init();
    ALLOCATOR.lock().init(heap_bottom, heap_size);
}
