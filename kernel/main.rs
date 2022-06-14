#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![feature(alloc_error_handler)]
#![feature(linked_list_cursors)]
#![allow(clippy::missing_safety_doc)]
// #![feature(custom_test_frameworks)]
// #![test_runner(crate::test_runner)]
// #![reexport_test_harness_main = "test_main"]

extern crate alloc;

pub mod allocator;
pub mod arch;
pub mod logger;
pub mod page_frame_allocator;
pub mod println;
// pub mod process;
// pub mod task;

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

#[alloc_error_handler]
fn alloc_error_handler(layout: alloc::alloc::Layout) -> ! {
    panic!("allocation error: {:?}", layout)
}

#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
}
