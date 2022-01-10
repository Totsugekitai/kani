use crate::arch;
use alloc::collections::BTreeMap;

#[derive(Debug)]
pub struct Process {
    ctx: arch::x64::process::Process,
}
