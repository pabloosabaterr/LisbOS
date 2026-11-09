use uefi::{
    boot::{self, MemoryType, PAGE_SIZE},
    mem::memory_map::MemoryMap,
    println,
};

const MIB: u64 = 1024 * 1024;

pub fn type_info(ty: MemoryType) -> (&'static str, &'static str) {
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

pub fn disp_memmap() {
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
