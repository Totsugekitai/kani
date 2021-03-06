use crate::arch::x64::ioapic;
use crate::print;
use alloc::sync::Arc;
use core::fmt::Write;
use lazy_static::lazy_static;
use log::{debug, info};
use spin::mutex::Mutex;
use x86_64::instructions::port::{PortGeneric, ReadOnlyAccess, WriteOnlyAccess};
use x86_64::structures::idt::InterruptStackFrame;

pub const COM1: u16 = 0x3f8;
pub const COM2: u16 = 0x2f8;
pub const COM3: u16 = 0x3e8;
pub const COM4: u16 = 0x2e8;

const IRQ_COM1: u32 = 4;
const IRQ_COM2: u32 = 3;

lazy_static! {
    pub static ref UART: Arc<Mutex<Uart>> = Arc::new(Mutex::new(Uart::new(COM1)));
}

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
    pub unsafe fn init(&self) -> Result<(), UartErrorKind> {
        // 8259 PIC Disable
        PortGeneric::<u8, WriteOnlyAccess>::new(0xa1).write(0xff);
        PortGeneric::<u8, WriteOnlyAccess>::new(0x21).write(0xff);
        // 16550A UART Enable
        PortGeneric::<u8, WriteOnlyAccess>::new(self.com + 1).write(0); // disable all interrupts
        PortGeneric::<u8, WriteOnlyAccess>::new(self.com + 3).write(0x80); // DLAB set 1
        PortGeneric::<u8, WriteOnlyAccess>::new(self.com).write(1); // 115200 / 115200
        PortGeneric::<u8, WriteOnlyAccess>::new(self.com + 1).write(0); // baud rate hi bytes
        PortGeneric::<u8, WriteOnlyAccess>::new(self.com + 3).write(0x03); // DLAB set 0
        PortGeneric::<u8, WriteOnlyAccess>::new(self.com + 4).write(0x0b); // IRQ enable
        PortGeneric::<u8, WriteOnlyAccess>::new(self.com + 1).write(0x01); // interrupt enable

        if PortGeneric::<u16, ReadOnlyAccess>::new(self.com + 5).read() == 0xff {
            return Err(UartErrorKind::NotImplement);
        }

        PortGeneric::<u16, ReadOnlyAccess>::new(self.com + 2).read();
        PortGeneric::<u16, ReadOnlyAccess>::new(self.com).read();

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
            x86_64::instructions::hlt();
        }
        PortGeneric::<u8, WriteOnlyAccess>::new(self.com).write(c);
    }

    #[cfg(feature = "qemu")]
    pub unsafe fn read(self) -> u8 {
        PortGeneric::<u16, ReadOnlyAccess>::new(self.com).read() as u8
    }

    #[cfg(not(feature = "qemu"))]
    pub unsafe fn read(self) -> u8 {
        while PortGeneric::<u16, ReadOnlyAccess>::new(self.com + 5).read() & 1 != 1 {
            x86_64::instructions::hlt();
        }

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

pub unsafe fn init() {
    {
        match UART.lock().init() {
            Ok(()) => (),
            Err(e) => match e {
                UartErrorKind::InvalidParams => {
                    panic!();
                }
                UartErrorKind::NotImplement => (), // FIXME: correct error handling
            },
        }
    }
    remove_screen();
    info!("init UART");
}

pub extern "x86-interrupt" fn uart_handler(_: InterruptStackFrame) {
    x86_64::instructions::interrupts::without_interrupts(|| unsafe {
        let mut c = UART.lock().read();
        if c as char == '\r' {
            c = b'\n';
        }
        // crate::task::uart::add_ascii(c);
        print!("{}", c as char);
    });
    debug!("UART interrupt.");
}

pub fn remove_screen() {
    // let mut uart = UART.lock();
    // let _ = uart.write_str("\x1b[2J\x1b[1;1H");
    print!("\x1b[2J\x1b[1;1H");
}
