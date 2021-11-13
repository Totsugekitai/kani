use crate::arch::x64::ioapic;
use core::fmt::Write;
use spin::mutex::Mutex;
use x86_64::instructions::port::{PortGeneric, ReadOnlyAccess, WriteOnlyAccess};

pub const COM1: u16 = 0x3f8;
pub const COM2: u16 = 0x2f8;
pub const COM3: u16 = 0x3e8;
pub const COM4: u16 = 0x2e8;

const IRQ_COM1: u32 = 4;
const IRQ_COM2: u32 = 3;

pub static UART: Mutex<Uart> = Mutex::new(Uart::new(COM1));

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Uart {
    com: u16,
}

pub enum UartErrorKind {
    NotImplement,
    InvalidParams,
}

impl Uart {
    pub const fn new(com: u16) -> Self {
        Uart { com }
    }
    pub unsafe fn init(self) -> Result<(), UartErrorKind> {
        // 8259 PIC Disable
        PortGeneric::<u8, WriteOnlyAccess>::new(0xa1).write(0xff);
        PortGeneric::<u8, WriteOnlyAccess>::new(0x21).write(0xff);
        // 16550A UART Enable
        PortGeneric::<u8, WriteOnlyAccess>::new(self.com + 1).write(0);
        PortGeneric::<u8, WriteOnlyAccess>::new(self.com + 3).write(0x80);
        PortGeneric::<u8, WriteOnlyAccess>::new(self.com + 0).write(1); // 115200 / 115200
        PortGeneric::<u8, WriteOnlyAccess>::new(self.com + 1).write(0);
        PortGeneric::<u8, WriteOnlyAccess>::new(self.com + 3).write(0x03);
        PortGeneric::<u8, WriteOnlyAccess>::new(self.com + 4).write(0x0b);
        PortGeneric::<u8, WriteOnlyAccess>::new(self.com + 1).write(0x01);

        if PortGeneric::<u16, ReadOnlyAccess>::new(self.com + 5).read() == 0xff {
            return Err(UartErrorKind::NotImplement);
        }

        PortGeneric::<u16, ReadOnlyAccess>::new(self.com + 2).read();
        PortGeneric::<u16, ReadOnlyAccess>::new(self.com + 0).read();

        if self.com == COM1 || self.com == COM3 {
            ioapic::enable(IRQ_COM1, 0);
        } else if self.com == COM2 || self.com == COM4 {
            ioapic::enable(IRQ_COM2, 0);
        } else {
            return Err(UartErrorKind::InvalidParams);
        }
        x86_64::instructions::interrupts::enable();
        Ok(())
    }

    #[cfg(feature = "qemu")]
    pub unsafe fn write(self, c: u8) {
        PortGeneric::<u8, WriteOnlyAccess>::new(self.com + 0).write(c);
    }

    #[cfg(not(feature = "qemu"))]
    pub unsafe fn write(self, c: u8) {
        while PortGeneric::<u16, ReadOnlyAccess>::new(self.com + 5).read() & 0x20 != 0x20 {
            asm!("nop");
        }
        PortGeneric::<u8, WriteOnlyAccess>::new(self.com + 0).write(c);
    }

    pub unsafe fn read(self) -> u8 {
        /*         while PortGeneric::<u16, ReadOnlyAccess>::new(self.com + 5).read() & 1 != 1 {
                   asm!("nop");
               }
        */
        PortGeneric::<u16, ReadOnlyAccess>::new(self.com).read() as u8
    }
}

impl Write for Uart {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for c in s.bytes() {
            unsafe { self.write(c) }
        }
        Ok(())
    }
}

pub unsafe fn uart_init() {
    match UART.lock().init() {
        Ok(()) => (),
        Err(e) => match e {
            UartErrorKind::InvalidParams => {
                panic!();
            }
            UartErrorKind::NotImplement => (),
        },
    }
}
