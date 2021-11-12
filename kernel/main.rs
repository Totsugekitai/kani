#![no_std]
#![no_main]
#![feature(asm)]
#![feature(global_asm)]

pub mod arch;
pub mod println;

use core::panic::PanicInfo;
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
