#![feature(llvm_asm)]

use crate::arch::x64::uart;

#[no_mangle]
#[warn(dead_code)]
unsafe extern "C" fn init_x86() {
    llvm_asm!("mov dword ptr [0xb8000], 0x2f4b2f4f" :::: "intel");
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
