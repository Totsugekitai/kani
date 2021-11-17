use arrayvec::ArrayVec;
use log::debug;

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
#[non_exhaustive]
#[repr(u32)]
enum InfoType {
    Terminate = 0,
    BootCommandLine = 1,
    BootLoaderName = 2,
    Modules = 3,
    BasicMemoryInfo = 4,
    BiosBootDevice = 5,
    MemoryMap = 6,
    VbeInfo = 7,
    FramebufferInfo = 8,
    ElfSymbols = 9,
    ApmTable = 10,
    Efi32bitSystemTablePointer = 11,
    Efi64bitSystemTablePointer = 12,
    SmbiosTables = 13,
    AcpiOldRsdp = 14,
    AcpiNewRsdp = 15,
    NetworkingInfo = 16,
    EfiMemoryMap = 17,
    EfiBootServicesNotTerminated = 18,
    Efi32bitImageHandlePointer = 19,
    Efi64bitImageHandlePointer = 20,
    ImageLoadBasePhysicalAddress = 21,
}

#[allow(unreachable_patterns)]
impl core::fmt::Display for InfoType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Terminate => f.write_fmt(format_args!("Terminate")),
            Self::BootCommandLine => f.write_fmt(format_args!("BootCommandLine")),
            Self::BootLoaderName => f.write_fmt(format_args!("BootLoaderName")),
            Self::Modules => f.write_fmt(format_args!("Modules")),
            Self::BasicMemoryInfo => f.write_fmt(format_args!("BasicMemoryInfo")),
            Self::BiosBootDevice => f.write_fmt(format_args!("BiosBootDevice")),
            Self::MemoryMap => f.write_fmt(format_args!("MemoryMap")),
            Self::VbeInfo => f.write_fmt(format_args!("VbeInfo")),
            Self::FramebufferInfo => f.write_fmt(format_args!("FramebufferInfo")),
            Self::ElfSymbols => f.write_fmt(format_args!("ElfSymbols")),
            Self::ApmTable => f.write_fmt(format_args!("ApmTable")),
            Self::Efi32bitSystemTablePointer => {
                f.write_fmt(format_args!("Efi32bitSystemTablePointer"))
            }
            Self::Efi64bitSystemTablePointer => {
                f.write_fmt(format_args!("Efi64bitSystemTablePointer"))
            }
            Self::SmbiosTables => f.write_fmt(format_args!("SmbiosTables")),
            Self::AcpiOldRsdp => f.write_fmt(format_args!("AcpiOldRsdp")),
            Self::AcpiNewRsdp => f.write_fmt(format_args!("AcpiNewRsdp")),
            Self::NetworkingInfo => f.write_fmt(format_args!("NetworkingInfo")),
            Self::EfiMemoryMap => f.write_fmt(format_args!("EfiMemoryMap")),
            Self::EfiBootServicesNotTerminated => {
                f.write_fmt(format_args!("EfiBootServicesNotTerminated"))
            }
            Self::Efi32bitImageHandlePointer => {
                f.write_fmt(format_args!("Efi32bitImageHandlePointer"))
            }
            Self::Efi64bitImageHandlePointer => {
                f.write_fmt(format_args!("Efi64bitImageHandlePointer"))
            }
            Self::ImageLoadBasePhysicalAddress => {
                f.write_fmt(format_args!("ImageLoadBasePhysicalAddress"))
            }
            _ => f.write_fmt(format_args!("Unknown")),
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
struct BasicHeader {
    total_size: u32,
    reserved: u32,
}

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
struct InfoTagHeader {
    info_type: InfoType,
    size: u32,
}

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
struct MemoryMapTag {
    header: InfoTagHeader,
    entry_size: u32,
    entry_version: u32,
}

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct MemoryMapEntry {
    pub base_addr: u64,
    pub length: u64,
    pub entry_type: MemoryMapType,
    reserved: u32,
}

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
#[non_exhaustive]
#[repr(u32)]
pub enum MemoryMapType {
    Available = 1,
    Acpi = 3,
    ReservedForHibernation = 4,
    DefectiveRamModules = 5,
}

#[allow(unreachable_patterns)]
impl core::fmt::Display for MemoryMapType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Available => f.write_fmt(format_args!("Available")),
            Self::Acpi => f.write_fmt(format_args!("ACPI")),
            Self::ReservedForHibernation => f.write_fmt(format_args!("ReservedForHibernation")),
            Self::DefectiveRamModules => f.write_fmt(format_args!("DefectiveRamModules")),
            _ => f.write_fmt(format_args!("Reserved")),
        }
    }
}

#[derive(Debug, Clone)]
pub struct BootInfo {
    pub memory_map: ArrayVec<MemoryMapEntry, 8>,
}

/// multiboot2のマジックが正しいか判定する
pub fn is_magic_correct(magic: u32) -> bool {
    const MULTIBOOT2_MAGIC: u32 = 0x36d76289;
    if magic == MULTIBOOT2_MAGIC {
        true
    } else {
        false
    }
}

/// multiboot2 information headerのtypeによって処理を振り分ける関数
#[allow(unaligned_references)]
pub unsafe fn process_info(addr: usize) -> BootInfo {
    let mut boot_info = BootInfo {
        memory_map: ArrayVec::new(),
    };

    let base_addr = addr as *const BasicHeader;
    let mut total_size = (*base_addr).total_size;
    let mut tag_ptr =
        (base_addr as usize + core::mem::size_of::<BasicHeader>()) as *const InfoTagHeader;
    while total_size > 0 {
        let tag = tag_ptr.as_ref().unwrap();
        debug!(
            "tag_base: 0x{:x}, tag type: {}, tag size: 0x{:x}",
            tag_ptr as usize, tag.info_type, tag.size
        );
        match tag.info_type {
            InfoType::MemoryMap => {
                let mmap_tag_ptr = tag_ptr as *const MemoryMapTag;
                let mmap_tag = mmap_tag_ptr.as_ref().unwrap();
                let entry_ptr = (mmap_tag_ptr as usize + core::mem::size_of::<MemoryMapTag>())
                    as *const MemoryMapEntry;
                let num_entries =
                    (tag.size - core::mem::size_of::<MemoryMapTag>() as u32) / mmap_tag.entry_size;
                boot_info = parse_memory_map(entry_ptr, num_entries);
            }
            InfoType::Terminate => {
                debug!(
                    "Terminate tag found: info_type={}, size=0x{:x}",
                    tag.info_type, tag.size
                );
                break;
            }
            _ => {
                if tag.info_type as u32 > 21 {
                    panic!("unknown type of multiboot2 information: {}", tag.info_type);
                } else {
                    debug!("unknown type of multiboot2 information: {}", tag.info_type);
                }
            }
        }
        let tag_size = if tag.size % 8 != 0 {
            tag.size + (8 - tag.size % 8)
        } else {
            tag.size
        };
        total_size -= tag_size;
        tag_ptr = (tag_ptr as usize + tag_size as usize) as *const InfoTagHeader;
    }
    boot_info
}

/// multibootから渡されてきたメモリマップをパースする関数
#[allow(unaligned_references)]
unsafe fn parse_memory_map(ptr: *const MemoryMapEntry, n: u32) -> BootInfo {
    let mut boot_info = BootInfo {
        memory_map: ArrayVec::new(),
    };

    let mut entry_ptr = ptr;
    for i in 0..n {
        let entry = entry_ptr.as_ref().unwrap();
        debug!(
            "Memory Map {}: base_addr=0x{:x}, len=0x{:x}, type={}",
            i, entry.base_addr, entry.length, entry.entry_type
        );
        if let MemoryMapType::Available = entry.entry_type {
            boot_info.memory_map.push(entry.clone());
        }

        entry_ptr =
            (entry_ptr as usize + core::mem::size_of::<MemoryMapEntry>()) as *const MemoryMapEntry;
    }
    boot_info
}
