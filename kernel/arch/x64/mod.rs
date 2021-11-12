global_asm!(include_str!("boot.S"));

pub mod init;
pub mod ioapic;
pub mod uart;
