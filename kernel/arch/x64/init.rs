use super::gdt::gdt_init;
use super::interrupts::idt_init;
use super::lapic::lapic_init;
use super::multiboot2;
use super::uart::uart_init;
use crate::logger;
use crate::println;
use log::{debug, info};
use x86_64::structures::paging::Translate;

const INIT_PAGING_PHYS_MEM_OFFSET: u64 = 0;

#[no_mangle]
#[warn(dead_code)]
pub unsafe extern "C" fn init_x86(multiboot2_magic: u32, multiboot2_info: usize) {
    let _ = logger::init();
    uart_init();
    gdt_init();
    idt_init();
    lapic_init();

    if !multiboot2::is_magic_correct(multiboot2_magic) {
        panic!("multiboot2 magic is incorrect.");
    }
    let _boot_info = multiboot2::process_info(multiboot2_info);

    use super::paging;
    use x86_64::VirtAddr;
    let phys_mem_offset = VirtAddr::new(INIT_PAGING_PHYS_MEM_OFFSET);
    let mapper = paging::init(phys_mem_offset);

    // 物理アドレスとリニアアドレス変換の確認用
    {
        let addresses = [
            0x0,
            0xFEE0_00F0,
            0x201008,
            0xFFFF_FFFF,
            0x1_0000_0000,
            0x0100_0020_1a10,
        ];
        for &address in &addresses {
            let virt = VirtAddr::new(address);
            let phys = mapper.translate_addr(virt);
            debug!("{:?} -> {:?}", virt, phys);
        }
    }

    info!("boot ok.");
    println!("Hello, kani!");

    loop {
        x86_64::instructions::hlt();
    }
}
