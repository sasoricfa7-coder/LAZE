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

    let boot_services = system_table.boot_services();

    // 1. Initialise l'affichage passif
    drivers::screen::init_laze_screen(boot_services);

    // 2. Découpe la RAM en zones étanches et fixes (Stade 4)
    drivers::memory::map_laze_memory(boot_services);

    // 3. Boucle d'exécution du Micro-noyau
    loop {
        security::run_shell(boot_services, &system_table);
    }
}
