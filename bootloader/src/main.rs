#![no_main]
#![no_std]

use uefi::{
    CStr16, Result,
    boot::{MemoryType, PAGE_SIZE, ScopedProtocol},
    mem::memory_map::MemoryMap,
    prelude::*,
    println,
    proto::{
        console::text::{Input, Key, ScanCode},
        media::{
            file::{File, FileAttribute, FileInfo, FileMode::Read},
            fs::SimpleFileSystem,
        },
    },
};

const MIB: u64 = 1024 * 1024;

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

fn type_info(ty: MemoryType) -> (&'static str, &'static str) {
    match ty {
        MemoryType::CONVENTIONAL => ("conventional", "FREE"),
        MemoryType::BOOT_SERVICES_CODE => ("boot_code", "RECLAIM"),
        MemoryType::BOOT_SERVICES_DATA => ("boot_data", "RECLAIM"),
        MemoryType::LOADER_CODE => ("loader_code", "LOADER"),
        MemoryType::LOADER_DATA => ("loader_data", "LOADER"),
        MemoryType::RUNTIME_SERVICES_CODE => ("runtime_code", "FIRMWARE"),
        MemoryType::RUNTIME_SERVICES_DATA => ("runtime_data", "FIRMWARE"),
        MemoryType::ACPI_RECLAIM => ("acpi_reclaim", "ACPI"),
        MemoryType::ACPI_NON_VOLATILE => ("acpi_nvs", "ACPI"),
        MemoryType::MMIO => ("mmio", "RESERVED"),
        MemoryType::MMIO_PORT_SPACE => ("mmio_port", "RESERVED"),
        MemoryType::UNUSABLE => ("unusable", "RESERVED"),
        MemoryType::RESERVED => ("reserved", "RESERVED"),
        _ => ("unknown", "RESERVED"),
    }
}

fn disp_memmap() {
    println!("Memory map:");
    println!(
        "{:>3} {:<14} {:<10} {:<16} {}",
        "Idx", "Name", "Size", "Start", "Type"
    );

    let mem_map = boot::memory_map(MemoryType::LOADER_DATA).expect("Unable to get memmap");
    let mut total_free = 0;
    for (idx, entry) in mem_map.entries().enumerate() {
        let (name, kind) = type_info(entry.ty);
        let size = entry.page_count * PAGE_SIZE as u64;
        if matches!(
            entry.ty,
            MemoryType::CONVENTIONAL
                | MemoryType::BOOT_SERVICES_CODE
                | MemoryType::BOOT_SERVICES_DATA
        ) {
            total_free += size;
        }

        println!(
            "{:>3} {:<14} {:010x} {:016x} {}",
            idx, name, size, entry.phys_start, kind
        );
    }

    println!();
    println!("Total free mem: {} MiB", total_free / MIB);
}

const MAX_FAT32_FILENAME_LENGTH: usize = 255;

#[repr(align(8))]
struct InfoWrapper {
    buffer: [u8; 1024],
}

fn read_file(file_name: &str, buffer: &mut [u8]) -> Result<usize> {
    /* +1 for the NULL terminator */
    let mut file_name_buffer = [0; MAX_FAT32_FILENAME_LENGTH + 1];
    let mut info_buffer = InfoWrapper { buffer: [0; 1024] };

    let handle = boot::image_handle();
    let mut sfs: ScopedProtocol<SimpleFileSystem> = boot::get_image_file_system(handle)?;

    let mut volume = sfs.open_volume()?;

    let name = CStr16::from_str_with_buf(file_name, &mut file_name_buffer)
        .map_err(|_| Status::INVALID_PARAMETER)?;
    let mut file = volume
        .open(name, Read, FileAttribute::empty())?
        .into_regular_file()
        .ok_or(Status::UNSUPPORTED)?;
    let file_len = file
        .get_info::<FileInfo>(&mut info_buffer.buffer).discard_errdata()?
        .file_size() as usize;

    let read_bytes = file.read(&mut *buffer)?;

    if read_bytes != file_len {
        return Err(Status::BUFFER_TOO_SMALL.into())
    }
    assert_eq!(read_bytes, file_len);

    Ok(read_bytes)
}

#[entry]
fn main() -> Status {
    uefi::helpers::init().unwrap();
    println!("Welcome to LisbOS bootloader");
    println!();

    disp_memmap();
    let mut file_buffer = [0; 1024];
    let filename = "test.txt";
    let file_len = match read_file(filename, &mut file_buffer) {
        Ok(x) => x,
        Err(x) => {
            println!("{}", x);
            return x.status()    
        }
    };

    println!(
        "{}: {}",
        filename,
        str::from_utf8(&file_buffer[..file_len]).unwrap()
    );

    system::with_stdin(wait_esc).expect("Unable to read the keyboard");

    Status::SUCCESS
}
