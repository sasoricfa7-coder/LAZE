use uefi::prelude::*;
use uefi::cstr16;
use uefi::proto::console::text::{Key};
use crate::drivers::fs::{list_root_directory, cat_target_file};

pub fn run_shell(system_table: &mut SystemTable<Boot>) {
    // 1. Affiche le prompt
    let _ = system_table.stdout().output_string(cstr16!("sasori@laze:~$ "));

    let mut input_buffer = [0u16; 64];
    let mut i = 0;

    // 2. Boucle de lecture clavier
    loop {
        let event = system_table.stdin().wait_for_key_event().expect("Stdin error");
        let _ = system_table.boot_services().wait_for_event(&mut [event]);
        
        if let Ok(Some(key)) = system_table.stdin().read_key() {
            match key {
                Key::Printable(c) => {
                    if i < 63 {
                        input_buffer[i] = u16::from(c);
                        i += 1;
                        let echo = [u16::from(c), 0];
                        let _ = system_table.stdout().output_string(uefi::CStr16::from_u16_with_nul(&echo).unwrap());
                    }
                }
                Key::Special(_) => {
                    // On valide l'entrée avec Entrée
                    let _ = system_table.stdout().output_string(cstr16!("\r\n"));
                    break;
                }
            }
        }
    }

    // 3. Exécution de la commande
    if i > 0 {
        if i == 2 && input_buffer[0] == b'l' as u16 && input_buffer[1] == b's' as u16 {
            list_root_directory(system_table);
        } else if i >= 3 && input_buffer[0] == b'c' as u16 && input_buffer[1] == b'a' as u16 && input_buffer[2] == b't' as u16 {
            cat_target_file(system_table);
        } else {
            let _ = system_table.stdout().output_string(cstr16!("laze: command not found\r\n"));
        }
    }

    // 4. Force un saut de ligne après l'exécution pour séparer les prompts
    let _ = system_table.stdout().output_string(cstr16!("\r\n"));
}
