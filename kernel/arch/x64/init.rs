use super::gdt::gdt_init;
use super::interrupts::idt_init;
use super::lapic::lapic_init;
use super::uart::uart_init;
use crate::logger;
use crate::println;
use log::info;
use x86_64::structures::paging::{Page, Size2MiB, Translate};

const INIT_PAGING_PHYS_MEM_OFFSET: u64 = 0;

#[no_mangle]
#[warn(dead_code)]
pub unsafe extern "C" fn init_x86() {
    uart_init();
    let _ = logger::init();
    gdt_init();
    idt_init();
    lapic_init();

    use super::paging;
    use x86_64::VirtAddr;
    let phys_mem_offset = VirtAddr::new(INIT_PAGING_PHYS_MEM_OFFSET);
    let mut mapper = paging::init(phys_mem_offset);
    let mut frame_allocator = paging::EmptyFrameAllocator;

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

    let unused_page = Page::<Size2MiB>::containing_address(VirtAddr::new(0x2_0000_0000));
    paging::create_example_mapping(unused_page, &mut mapper, &mut frame_allocator);
    let page_ptr: *mut u64 = unused_page.start_address().as_mut_ptr();
    page_ptr.offset(0).write_volatile(0x_f021_f077_f065_f04e);

    println!("Hello, kani!");
    info!("boot ok.");

    loop {
        x86_64::instructions::hlt();
    }
}
