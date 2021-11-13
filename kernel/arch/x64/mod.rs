global_asm!(include_str!("boot.S"));

pub mod gdt;
pub mod init;
pub mod interrupts;
pub mod ioapic;
pub mod uart;
