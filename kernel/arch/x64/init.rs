use super::gdt::gdt_init;
use super::interrupts::idt_init;
use super::uart::uart_init;
use crate::println;
use core::ptr::write_volatile;

#[no_mangle]
#[warn(dead_code)]
unsafe extern "C" fn init_x86() {
    uart_init();
    gdt_init();
    idt_init();
    println!("\r\nHello, kani!");
    println!("ok.");
    write_volatile(0xffffffff33333333 as *mut u8, 3);
    loop {}
}
