use alloc::sync::Arc;
use arrayvec::ArrayVec;
use bitflags::bitflags;
use lazy_static::lazy_static;
use spin::Mutex;

bitflags! {
    struct PageFrameMapEntry: u8 {
        const present = 0b00000001;
        const kernel_used = 0b10000000;
    }
}

impl PageFrameMapEntry {
    fn clear(&mut self) {
        self.bits = 0;
    }

    fn is_present(&self) -> bool {
        (self.bits() & 0b00000001) == 0b00000001
    }

    fn is_kernel_used(&self) -> bool {
        (self.bits() & 0b10000000) == 0b10000000
    }
}

#[derive(Debug, Clone)]
pub struct PageFrameMap {
    // エントリが何個あればいいのかわからないので、とりあえずメモリ16GBを最大に設定
    // ページは2MB単位として計算
    // 17179869184 / 2097152 = 8192 (16GB / 2MB)
    map: ArrayVec<PageFrameMapEntry, 8192>,
}

impl PageFrameMap {
    pub fn new() -> Self {
        let mut page_frame_map = Self {
            map: ArrayVec::new(),
        };
        for entry in &mut page_frame_map.map {
            entry.clear();
        }
        page_frame_map
    }
}

lazy_static! {
    pub static ref PAGE_FRAME_MAP: Arc<Mutex<PageFrameMap>> =
        Arc::new(Mutex::new(PageFrameMap::new()));
}
