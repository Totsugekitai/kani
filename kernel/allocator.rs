use linked_list_allocator::LockedHeap;

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

extern "C" {
    static __kernel_heap: u8;
}

const HEAP_SIZE: usize = 0x1000 * 0x1000;

pub unsafe fn init() {
    ALLOCATOR
        .lock()
        .init(&__kernel_heap as *const u8 as usize, HEAP_SIZE);
}
