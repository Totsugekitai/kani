#![no_std]
#![no_main]
#![feature(asm)]
#![feature(global_asm)]
#![feature(abi_x86_interrupt)]

pub mod arch;
pub mod logger;
pub mod println;

use core::panic::PanicInfo;
use log::error;

#[cfg(target_arch = "x86_64")]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("===== panic! =====");
    error!("{:?}", info);
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
