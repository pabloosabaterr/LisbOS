#![no_main]
#![no_std]

use uefi::{
    boot::{MemoryType, PAGE_SIZE},
    mem::memory_map::{MemoryMap, MemoryMapOwned},
    prelude::*,
    println,
};

#[entry]
fn main() -> Status {
    uefi::helpers::init().unwrap();

    let mem_map: MemoryMapOwned = boot::memory_map(MemoryType::LOADER_DATA).unwrap();
    for n in mem_map.entries() {
        println!(
            "type: {:?}\n    start: {:#X}\n    size: {} bytes ({} pages)",
            n.ty,
            n.phys_start,
            n.page_count * PAGE_SIZE as u64,
            n.page_count
        );
    }

    loop {}
}
