#![no_std]
#![no_main]
#![feature(asm)]
#![feature(llvm_asm)]
#![feature(global_asm)]

mod arch;

#[no_mangle]
#[warn(dead_code)]
unsafe extern "C" fn init_x86() {
    llvm_asm!("mov dword ptr [0xb8000], 0x2f4b2f4f" :::: "intel");
    loop {}
}

use core::panic::PanicInfo;
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
