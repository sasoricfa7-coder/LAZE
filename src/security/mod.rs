use uefi::prelude::*;
use crate::drivers::screen::flash_security_alert;

pub fn run_shell(system_table: &mut SystemTable<Boot>) {
    // 1. On lit la touche sur stdin
    let stdin = system_table.stdin();
    let mut trigger_alert = false;

    if let Ok(Some(key)) = stdin.read_key() {
        match key {
            uefi::proto::console::text::Key::Printable(ch) => {
                if char::from(ch) == 'r' {
                    trigger_alert = true;
                }
            }
            _ => {}
        }
    }

    // 2. L'emprunt de stdin est terminé ici. On peut appeler librement les boot_services
    let boot_services = system_table.boot_services();
    
    if trigger_alert {
        flash_security_alert(boot_services);
    }

    // Pause de 10ms pour le processeur
    boot_services.stall(10_000);
}
