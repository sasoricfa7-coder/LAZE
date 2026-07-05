#![no_std]
#![no_main]

use core::panic::PanicInfo;
use uefi::prelude::*;

mod drivers;
mod security;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[entry]
fn main(_image_handle: Handle, system_table: SystemTable<Boot>) -> Status {
    uefi::helpers::init().unwrap();

    // 1. Initialise l'affichage passif (L'écran bleu et la zone blanche)
    drivers::screen::init_laze_screen(system_table.boot_services());

    // 2. Boucle d'exécution du Micro-noyau (Surveillance constante)
    loop {
        // Écoute le matériel et gère les zones de manière isolée
        security::run_shell(system_table.boot_services(), &system_table);
    }
}
