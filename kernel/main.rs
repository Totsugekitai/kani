#![no_std]
#![no_main]
#![feature(asm)]
#![feature(global_asm)]

pub mod arch;
pub mod println;

use core::panic::PanicInfo;

#[cfg(target_arch = "x86_64")]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("===== panic! =====");
    println!("{:?}", info);
    loop {
        x86_64::instructions::hlt();
    }
}

#[cfg(not(target_arch = "x86_64"))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("===== panic! =====");
    println!("{:?}", info);
    loop {}
}
