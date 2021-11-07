use crate::arch::x64::uart;

#[no_mangle]
#[warn(dead_code)]
unsafe extern "C" fn init_x86() {
    let _ = uart::init();
    for _ in 0..5 {
        let mut c = b'A';
        for _ in 0..26 {
            uart::putc(c);
            c += 1;
        }
        uart::putc(b'\n');
    }
    loop {}
}
