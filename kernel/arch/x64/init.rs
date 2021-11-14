use super::gdt::gdt_init;
use super::interrupts::idt_init;
use super::lapic::lapic_init;
use super::uart::uart_init;
use crate::logger;
use crate::println;

use log::info;

#[no_mangle]
#[warn(dead_code)]
pub unsafe extern "C" fn init_x86() {
    uart_init();
    let _ = logger::init();
    gdt_init();
    idt_init();
    lapic_init();
    println!("\r\nHello, kani!");
    info!("boot ok.");
    loop {}
}
