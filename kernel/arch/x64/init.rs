use crate::println;

#[no_mangle]
#[warn(dead_code)]
unsafe extern "C" fn init_x86() {
    println!("\r\nHello, kani!");
    loop {}
}
