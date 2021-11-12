use crate::arch::x64::ioapic;
use x86_64::instructions::port::{PortGeneric, ReadOnlyAccess, WriteOnlyAccess};

pub const COM1: u16 = 0x3f8;
pub const COM2: u16 = 0x2f8;
pub const COM3: u16 = 0x3e8;
pub const COM4: u16 = 0x2e8;

const IRQ_COM1: u32 = 4;
const IRQ_COM2: u32 = 3;

pub unsafe fn init(com: u16) -> Result<(), ()> {
    PortGeneric::<u8, WriteOnlyAccess>::new(com + 1).write(0);
    PortGeneric::<u8, WriteOnlyAccess>::new(com + 3).write(0x80);
    PortGeneric::<u8, WriteOnlyAccess>::new(com + 0).write(1); // 115200 / 115200
    PortGeneric::<u8, WriteOnlyAccess>::new(com + 1).write(0);
    PortGeneric::<u8, WriteOnlyAccess>::new(com + 3).write(0x03);
    PortGeneric::<u8, WriteOnlyAccess>::new(com + 4).write(0x0b);
    PortGeneric::<u8, WriteOnlyAccess>::new(com + 1).write(0x01);

    if PortGeneric::<u16, ReadOnlyAccess>::new(com + 5).read() == 0xff {
        return Err(());
    }

    PortGeneric::<u16, ReadOnlyAccess>::new(com + 2).read();
    PortGeneric::<u16, ReadOnlyAccess>::new(com + 0).read();

    if com == COM1 || com == COM3 {
        ioapic::enable(IRQ_COM1, 0);
    } else if com == COM2 || com == COM4 {
        ioapic::enable(IRQ_COM2, 0);
    } else {
        return Err(());
    }
    Ok(())
}

pub unsafe fn putc(com: u16, c: u8) {
    for _ in 0..1280000 {
        asm!("nop");
    }
    PortGeneric::<u8, WriteOnlyAccess>::new(com + 0).write(c);
}
