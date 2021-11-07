global_asm!(include_str!("boot.S"));

mod init;
mod ioapic;
mod uart;
