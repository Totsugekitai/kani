use crate::arch::x64::ioapic;
use x86_64::instructions::port::{PortGeneric, ReadOnlyAccess, WriteOnlyAccess};

const COM1: u16 = 0x3f8;
const IRQ_COM1: u32 = 4;

pub unsafe fn init() -> Result<(), ()> {
    PortGeneric::<u8, WriteOnlyAccess>::new(COM1 + 2).write(0);
    PortGeneric::<u8, WriteOnlyAccess>::new(COM1 + 3).write(0x80);
    PortGeneric::<u8, WriteOnlyAccess>::new(COM1 + 0).write(12); // 115200 / 9600
    PortGeneric::<u8, WriteOnlyAccess>::new(COM1 + 1).write(0);
    PortGeneric::<u8, WriteOnlyAccess>::new(COM1 + 3).write(0x03);
    PortGeneric::<u8, WriteOnlyAccess>::new(COM1 + 4).write(0);
    PortGeneric::<u8, WriteOnlyAccess>::new(COM1 + 1).write(0x01);

    if PortGeneric::<u16, ReadOnlyAccess>::new(COM1 + 5).read() == 0xff {
        return Err(());
    }

    PortGeneric::<u16, ReadOnlyAccess>::new(COM1 + 2).read();
    PortGeneric::<u16, ReadOnlyAccess>::new(COM1 + 0).read();

    ioapic::enable(IRQ_COM1, 0);
    Ok(())
}

pub unsafe fn putc(c: u8) {
    for _ in 0..128 {
        asm!("nop");
    }
    PortGeneric::<u8, WriteOnlyAccess>::new(COM1 + 0).write(c);
}
