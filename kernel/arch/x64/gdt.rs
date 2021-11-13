use core::ops::Deref;
use spin::mutex::Mutex;
use x86_64::instructions::segmentation::{Segment, CS};
use x86_64::instructions::tables::load_tss;
use x86_64::structures::gdt::{Descriptor, GlobalDescriptorTable, SegmentSelector};
use x86_64::structures::tss::TaskStateSegment;
use x86_64::PrivilegeLevel;
use x86_64::VirtAddr;

struct Selectors {
    code_selector: SegmentSelector,
    tss_selector: SegmentSelector,
}

pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;
const STACK_SIZE: usize = 0x1000 * 5;

static TSS: Mutex<TaskStateSegment> = Mutex::new(TaskStateSegment::new());
static GDT: Mutex<(GlobalDescriptorTable, Selectors)> = Mutex::new((
    GlobalDescriptorTable::new(),
    Selectors {
        code_selector: SegmentSelector::new(0, PrivilegeLevel::Ring0),
        tss_selector: SegmentSelector::new(0, PrivilegeLevel::Ring0),
    },
));
static STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

fn init_tss() {
    let mut tss = TSS.lock();
    tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = {
        let stack_start = VirtAddr::from_ptr(&STACK);
        let stack_end = stack_start + STACK_SIZE;
        stack_end
    }
}

fn init_gdt() {
    let mut gdt = GDT.lock();
    let tss = TSS.lock();
    let code_selector = gdt.0.add_entry(Descriptor::kernel_code_segment());
    unsafe {
        let tss_selector = gdt.0.add_entry(Descriptor::tss_segment(
            (tss.deref() as *const TaskStateSegment).as_ref().unwrap(),
        ));
        gdt.0.load_unsafe();
        gdt.1.code_selector = code_selector;
        gdt.1.tss_selector = tss_selector;
        CS::set_reg(gdt.1.code_selector);
        load_tss(gdt.1.tss_selector);
    }
}

pub fn init() {
    init_tss();
    init_gdt();
}
