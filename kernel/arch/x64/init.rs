use super::gdt::gdt_init;
use super::interrupts::idt_init;
use super::lapic::lapic_init;
use super::multiboot2;
use super::uart::uart_init;
use crate::logger;
use crate::println;
use log::info;
use x86_64::structures::paging::Translate;

const INIT_PAGING_PHYS_MEM_OFFSET: u64 = 0;

#[no_mangle]
#[warn(dead_code)]
pub unsafe extern "C" fn init_x86(magic: u32, bootinfo: usize) {
    uart_init();
    let _ = logger::init();
    gdt_init();
    idt_init();
    lapic_init();

    use super::paging;
    use x86_64::VirtAddr;
    let phys_mem_offset = VirtAddr::new(INIT_PAGING_PHYS_MEM_OFFSET);
    let mapper = paging::init(phys_mem_offset);

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
        println!("{:?} -> {:?}", virt, phys);
    }

    core::ptr::write_volatile(0xB_8000 as *mut u64, 0x_f021_f077_f065_f04e);

    if !multiboot2::is_magic_correct(magic) {
        panic!("multiboot2 magic is incorrect.");
    }
    multiboot2::process_info(bootinfo);

    println!("Hello, kani!");
    info!("boot ok.");

    loop {
        x86_64::instructions::hlt();
    }
}
