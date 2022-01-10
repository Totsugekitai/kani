use core::arch::global_asm;

global_asm!(include_str!("boot.S"));
global_asm!(include_str!("task.S"));

pub mod allocator;
pub mod gdt;
pub mod init;
pub mod interrupts;
pub mod ioapic;
pub mod lapic;
pub mod multiboot2;
pub mod paging;
pub mod process;
pub mod task;
pub mod uart;
