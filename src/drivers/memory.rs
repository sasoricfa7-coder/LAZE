use uefi::prelude::*;
use uefi::table::boot::MemoryType;
use uefi::mem::memory_map::MemoryMap; // Requis pour utiliser .entries()

pub fn map_laze_memory(boot_services: &BootServices) {
    // On spécifie le type de mémoire pour l'allocation de la carte
    if let Ok(memory_map) = boot_services.memory_map(MemoryType::LOADER_DATA) {
        for descriptor in memory_map.entries() {
            if descriptor.ty == MemoryType::CONVENTIONAL {
                let _start_addr = descriptor.phys_start;
                let _page_count = descriptor.page_count;
                break;
            }
        }
    }
}
