use super::{gdt, interrupts, lapic, multiboot2, uart};
use crate::{allocator, arch::task::ContextX64, logger, println, task};
use alloc::boxed::Box;
use log::{debug, info};
use x86_64::structures::paging::Translate;

const INIT_PAGING_PHYS_MEM_OFFSET: u64 = 0;

#[no_mangle]
pub unsafe extern "C" fn init_x86(multiboot2_magic: u32, multiboot2_info: usize) {
    let _ = logger::init();
    uart::init();
    gdt::init();
    interrupts::init();
    lapic::init();
    multiboot2::init(multiboot2_magic, multiboot2_info);

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

    allocator::init();

    info!("boot ok.");
    println!("Hello, kani!");

    let task_a_stack = Box::new([0u8; 0x1000]);
    let task_b_stack = Box::new([0u8; 0x1000]);

    let task_a = task::Task::<ContextX64>::new();
    let task_b = task::Task::<ContextX64>::new();

    loop {
        x86_64::instructions::hlt();
    }
}

fn task_a_fn() {
    loop {
        println!("Task A is running...");
    }
}

fn task_b_fn() {
    loop {
        println!("Task B is running...");
    }
}
