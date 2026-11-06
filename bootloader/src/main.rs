#![no_main]
#![no_std]

use uefi::{
    Result,
    boot::{MemoryType, PAGE_SIZE},
    mem::memory_map::MemoryMap,
    prelude::*,
    println,
    proto::console::text::{Input, Key, ScanCode},
};

fn wait_esc(input: &mut Input) -> Result {
    let events = [input.wait_for_key_event().unwrap()];
    loop {
        boot::wait_for_event(&events).discard_errdata()?;
        if let Some(Key::Special(ScanCode::ESCAPE)) = input.read_key()? {
            break;
        }
    }

    Ok(())
}

#[entry]
fn main() -> Status {
    uefi::helpers::init().unwrap();

    let mem_map = boot::memory_map(MemoryType::LOADER_DATA).expect("Unable to get memmap");
    for entry in mem_map.entries() {
        println!(
            "type: {:?}\n    start: {:#X}\n    size: {} bytes ({} pages)",
            entry.ty,
            entry.phys_start,
            entry.page_count * PAGE_SIZE as u64,
            entry.page_count
        );
    }

    system::with_stdin(wait_esc).expect("Unable to read the keyboard");

    Status::SUCCESS
}
