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
fn main(_image_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    uefi::helpers::init().unwrap();

    // 1. Initialise la console texte standard
    drivers::screen::init_laze_screen(&mut system_table);

    // 2. Découpe la RAM
    drivers::memory::map_laze_memory(system_table.boot_services());

    // 3. Boucle infinie sur le Shell console autonome
    loop {
        security::run_shell(&mut system_table);
    }
}
