use super::uart::uart_init;
use crate::println;

#[no_mangle]
#[warn(dead_code)]
unsafe extern "C" fn init_x86() {
    uart_init();
    println!("\r\nHello, kani!");
    println!("ok.");
    loop {}
}
