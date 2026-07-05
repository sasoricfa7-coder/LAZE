use uefi::prelude::*;
use uefi::cstr16;

pub fn init_laze_screen(system_table: &mut SystemTable<Boot>) {
    let stdout = system_table.stdout();
    let _ = stdout.clear();
    let _ = stdout.set_color(
        uefi::proto::console::text::Color::LightGreen,
        uefi::proto::console::text::Color::Black,
    );

    // Affichage de la bannière de bienvenue de LAZE
    let _ = stdout.output_string(cstr16!("Welcome to LAZE Microkernel (Axis Architecture)\r\n"));
    let _ = stdout.output_string(cstr16!("Copyright (C) 2026 Sasori. All rights reserved.\r\n\n"));
    
    // Premier prompt de commande style Linux
    let _ = stdout.output_string(cstr16!("sasori@laze:~$ "));
}
