use super::interrupts::InterruptIndex;
use log::debug;
use spin::mutex::Mutex;
use x86_64::structures::idt::InterruptStackFrame;

const SVR: *mut u32 = 0xFEE0_00F0 as *mut u32;
const LVT_TIMER: *mut u32 = 0xFEE0_0320 as *mut u32;
const INITIAL_COUNT: *mut u32 = 0xFEE0_0380 as *mut u32;
//const CURRENT_COUNT: *mut usize = 0xFEE0_0390 as *mut usize;
const DIVIDE_CONFIG: *mut u32 = 0xFEE0_03E0 as *mut u32;

pub const TIMER_INTERVAL: u32 = 10000000;

pub unsafe fn lapic_init() {
    let mut ia32_apic_base = x86_64::registers::model_specific::Msr::new(0x1b);
    let ia32_apic_base_value = ia32_apic_base.read();
    let ia32_apic_base_value = ia32_apic_base_value | (1 << 11);
    ia32_apic_base.write(ia32_apic_base_value);

    let svr_value = core::ptr::read_volatile(SVR);
    core::ptr::write_volatile(SVR, svr_value | (1 << 8));

    core::ptr::write_volatile(DIVIDE_CONFIG, 0b1011);
    core::ptr::write_volatile(
        LVT_TIMER,
        (0b10 << 16) | InterruptIndex::Lapic.as_usize() as u32,
    );
    core::ptr::write_volatile(INITIAL_COUNT, TIMER_INTERVAL);
}

static TICK: Mutex<usize> = Mutex::new(0);

pub mod tick {
    pub fn get() -> usize {
        use super::TICK;
        use x86_64::instructions::interrupts;
        let mut tick = 0;
        interrupts::without_interrupts(|| tick = *TICK.lock());
        tick
    }
}

pub extern "x86-interrupt" fn lapic_handler(_: InterruptStackFrame) {
    x86_64::instructions::interrupts::disable();
    {
        let mut tick = TICK.lock(); // must be included in this block!
        *tick += 1;
    }
    unsafe {
        super::interrupts::notify_end_of_interrupt();
    }
    x86_64::instructions::interrupts::enable();
    debug!("LAPIC interrupt. Tick: {}", tick::get());
}