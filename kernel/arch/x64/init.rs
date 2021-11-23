use super::{gdt, interrupts, lapic, multiboot2, uart};
use crate::{allocator, logger, println, task::Task};
use alloc::boxed::Box;
use lazy_static::lazy_static;
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

    let mut task_a_stack: Box<[u8; 0x1000]> = Box::new([0u8; 0x1000]);
    let mut task_b_stack: Box<[u8; 0x1000]> = Box::new([0u8; 0x1000]);
    let task_a = Task::new(task_a_fn as u64, task_a_stack.as_mut_ptr() as u64 + 0x1000);
    let task_b = Task::new(task_b_fn as u64, task_b_stack.as_mut_ptr() as u64 + 0x1000);

    task_a.register();
    task_b.register();

    loop {
        x86_64::instructions::hlt();
    }
}

lazy_static! {}

fn task_a_fn() {
    loop {
        //print!("A");
    }
}

fn task_b_fn() {
    loop {
        //print!("B");
    }
}
