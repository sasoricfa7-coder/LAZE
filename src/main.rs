/*
 * LAZE / AXIS Microkernel
 * Copyright (C) 2026  Sasori
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 */ 

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
