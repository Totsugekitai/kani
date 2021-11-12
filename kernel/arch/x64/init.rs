//#![feature(llvm_asm)]

use crate::arch::x64::uart;

#[no_mangle]
#[warn(dead_code)]
unsafe extern "C" fn init_x86() {
    //llvm_asm!("mov dword ptr [0xb8000], 0x2f4b2f4f" :::: "intel");
    //loop {}
    let com1 = uart::COM1;
    let com2 = uart::COM2;
    let com3 = uart::COM3;
    let com4 = uart::COM4;
    let serial = |com| {
        if let Ok(()) = uart::init(com) {
            for _ in 0..5 {
                let mut c = b'A';
                for _ in 0..26 {
                    uart::putc(com, c);
                    c += 1;
                }
                uart::putc(com, b'\r');
                uart::putc(com, b'\n');
            }
            Ok(())
        } else {
            Err(())
        }
    };
    let r = serial(com1);
    match r {
        Ok(()) => (),
        Err(()) => {
            return;
        }
    }
    loop {}
}
