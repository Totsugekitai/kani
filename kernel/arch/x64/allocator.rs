extern "C" {
    static __kernel_heap: u8;
    static __kernel_heap_end: u8;
}

pub unsafe fn init() -> (usize, usize) {
    let kernel_heap = &__kernel_heap as *const u8 as usize;
    let kernel_heap_end = &__kernel_heap_end as *const u8 as usize;
    let length = kernel_heap_end - kernel_heap;
    (kernel_heap, length)
}
