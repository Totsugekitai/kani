use super::{gdt, interrupts, lapic, multiboot2, uart};
use crate::{
    allocator, logger, println,
    task::{executor::Executor, Task},
};
use log::{debug, info};
use x86_64::structures::paging::Translate;

const INIT_PAGING_PHYS_MEM_OFFSET: u64 = 0;

#[no_mangle]
pub unsafe extern "C" fn init_x86(multiboot2_magic: u32, multiboot2_info: usize) {
    allocator::init();
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

    info!("boot ok.");
    println!("Hello, kani!");

    let mut executor = Executor::new();
    executor.spawn(Task::new(sample_task()));
    executor.spawn(Task::new(crate::task::uart::print_keypresses()));
    executor.run();
}

async fn sample() -> usize {
    42
}

async fn sample_task() {
    let num = sample().await;
    println!("num {}", num);
}
