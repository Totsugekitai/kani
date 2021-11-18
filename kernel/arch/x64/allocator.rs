extern "C" {
    static __kernel_heap: u8;
}

pub unsafe fn init() -> usize {
    &__kernel_heap as *const u8 as usize
}
