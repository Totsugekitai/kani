use super::arch;
use linked_list_allocator::LockedHeap;

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

const HEAP_SIZE: usize = 0x1000 * 0x1000;

#[cfg(target_arch = "x86_64")]
pub unsafe fn init() {
    let heap_bottom = arch::x64::allocator::init();
    ALLOCATOR.lock().init(heap_bottom, HEAP_SIZE);
}
