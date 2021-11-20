use log::info;
use x86_64::{
    structures::paging::{Mapper, OffsetPageTable, Page, PageTable, PhysFrame, Size4KiB},
    PhysAddr, VirtAddr,
};

/// 有効なレベル4テーブルへの可変参照を返す
/// `&mut`参照が複数の名称を持つこと(mutable aliasing、未定義動作)につながるため、この関数は一度しか呼び出してはならない
unsafe fn active_level_4_table(physical_memory_offset: VirtAddr) -> &'static mut PageTable {
    use x86_64::registers::control::Cr3;

    let (level_4_table_frame, _) = Cr3::read();

    let phys = level_4_table_frame.start_address();
    let virt = physical_memory_offset + phys.as_u64();
    let page_table_ptr: *mut PageTable = virt.as_mut_ptr();

    &mut *page_table_ptr // unsafe
}

pub unsafe fn init(physical_memory_offset: VirtAddr) -> OffsetPageTable<'static> {
    let level_4_table = active_level_4_table(physical_memory_offset);
    OffsetPageTable::new(level_4_table, physical_memory_offset)
}

extern "C" {
    static __kernel_image: u8;
    static __kernel_image_end: u8;
}

// #[derive(Debug)]
// pub struct FrameAllocator {
//     next: usize,
// }

// impl FrameAllocator {
//     pub fn init() -> Self {
//         FrameAllocator { next: 0 }
//     }
// }

// unsafe impl x86_64::structures::paging::FrameAllocator<Size4KiB> for FrameAllocator {
//     fn allocate_frame(&mut self) -> Option<PhysFrame<Size4KiB>> {
//         None
//     }
// }

/// 与えられたページを`0xb8000`が含まれているフレームにマップする
pub fn create_example_mapping(
    page: Page<Size4KiB>,
    mapper: &mut OffsetPageTable,
    frame_allocator: &mut impl x86_64::structures::paging::FrameAllocator<Size4KiB>,
) {
    use x86_64::structures::paging::PageTableFlags as Flags;
    unsafe {
        info!(
            "__kernel_image: 0x{:x}",
            &__kernel_image as *const u8 as usize
        );
        info!(
            "__kernel_image_end: 0x{:x}",
            &__kernel_image_end as *const u8 as usize
        );
    }
    let frame = PhysFrame::containing_address(PhysAddr::new(0xb8000));
    let flags = Flags::PRESENT | Flags::WRITABLE;

    let map_to_result = unsafe { mapper.map_to(page, frame, flags, frame_allocator) };
    map_to_result.expect("map_to failed").flush();
}

pub struct EmptyFrameAllocator;

unsafe impl x86_64::structures::paging::FrameAllocator<Size4KiB> for EmptyFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame> {
        None
    }
}
