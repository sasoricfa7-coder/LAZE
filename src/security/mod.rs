use uefi::prelude::*;
use crate::drivers::screen::draw_char;

pub fn run_shell(system_table: &mut SystemTable<Boot>) {
    let stdin = system_table.stdin();
    let mut key_to_print: Option<char> = None;

    if let Ok(Some(key)) = stdin.read_key() {
        match key {
            uefi::proto::console::text::Key::Printable(ch) => {
                key_to_print = Some(char::from(ch));
            }
            _ => {}
        }
    }

    let boot_services = system_table.boot_services();
    
    if let Some(c) = key_to_print {
        // On dessine le caractère saisi à la position (200, 200) dans notre terminal
        draw_char(boot_services, c, 200, 200);
    }

    // Temporisation de 10ms pour préserver l'exécution
    boot_services.stall(10_000);
}
