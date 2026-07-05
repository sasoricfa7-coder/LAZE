use uefi::prelude::*;
use uefi::table::boot::{MemoryDescriptor, MemoryType};

pub fn map_laze_memory(boot_services: &BootServices) {
    // 1. Allouer un espace temporaire pour stocker la carte mémoire de l'UEFI
    // On prend une taille généreuse pour être tranquille (Paresseux !)
    let mut buffer = [0u8; 8192];
    
    // 2. Récupérer la carte mémoire
    if let Ok((_key, desc_iter)) = boot_services.memory_map(&mut buffer) {
        // 3. Parcourir les blocs de RAM disponibles sur ton Celeron
        for descriptor in desc_iter {
            // On cherche la mémoire conventionnelle (RAM libre utilisable)
            if descriptor.ty == MemoryType::CONVENTIONAL {
                let _start_addr = descriptor.phys_start;
                let _page_count = descriptor.page_count;
                
                // C'est ici que LAZE va marquer de grands blocs fixes :
                // ex: ZONE_CODE_APPLICATIONS = start_addr
                // Pas de fragmentation, on prend tout le bloc d'un coup !
                break; // On prend le premier gros bloc libre pour rester paresseux
            }
        }
    }
}
