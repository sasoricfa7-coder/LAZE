use uefi::prelude::*;
use uefi::proto::media::fs::SimpleFileSystem;

pub fn list_root_directory(system_table: &mut SystemTable<Boot>) {
    let mut file_names = [[0u16; 32]; 10];
    let mut file_count = 0;

    {
        let boot_services = system_table.boot_services();

        if let Ok(fs_handle) = boot_services.get_handle_for_protocol::<SimpleFileSystem>() {
            if let Ok(mut fs) = boot_services.open_protocol_exclusive::<SimpleFileSystem>(fs_handle) {
                if let Ok(mut root) = fs.open_volume() {
                    let mut buffer = [0u8; 512];
                    
                    while file_count < 10 {
                        match root.read_entry(&mut buffer) {
                            Ok(Some(file_info)) => {
                                let name = file_info.file_name();
                                let mut i = 0;
                                for c in name.iter() {
                                    if i < 31 {
                                        // Correction du type ici : conversion de Char16 en u16
                                        file_names[file_count][i] = u16::from(*c);
                                        i += 1;
                                    }
                                }
                                file_names[file_count][i] = 0;
                                file_count += 1;
                            }
                            _ => break,
                        }
                    }
                }
            }
        }
    }

    let stdout = system_table.stdout();

    if file_count == 0 {
        let _ = stdout.output_string(uefi::cstr16!("Erreur : Impossible de lire le disque ou repertoire vide.\r\n"));
        return;
    }

    for i in 0..file_count {
        if let Ok(cstr) = uefi::CStr16::from_u16_with_nul(&file_names[i]) {
            let _ = stdout.output_string(cstr);
            let _ = stdout.output_string(uefi::cstr16!("   "));
        }
    }
    let _ = stdout.output_string(uefi::cstr16!("\r\n"));
}
