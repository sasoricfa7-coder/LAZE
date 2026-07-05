use uefi::prelude::*;
use uefi::proto::console::text::Input;

pub fn run_shell(boot_services: &BootServices, system_table: &SystemTable<Boot>) {
    let stdin = system_table.stdin();
    
    // Le strict minimum pour écouter une commande sans se fatiguer
    if let Ok(Some(key)) = stdin.read_key() {
        match key {
            uefi::proto::console::text::Key::Printable(ch) => {
                let char_val = char::from(ch);
                
                // Exemple de trigger de sécurité paresseux : si l'utilisateur tape 'r' (Reset)
                if char_val == 'r' {
                    // Ici on simulera la coupure de courant / reset de la zone active
                    // Pour l'instant, on se contente de savoir qu'on a intercepté l'ordre
                }
            }
            _ => {}
        }
    }
}
