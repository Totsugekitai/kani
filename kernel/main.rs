#![no_std]
#![no_main]
#![feature(asm)]
#![feature(global_asm)]

mod arch;

fn init_x86() {
    ()
}

use core::panic::PanicInfo;
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
 