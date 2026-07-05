use uefi::prelude::*;

pub fn init_laze_screen(system_table: &mut SystemTable<Boot>) {
    let stdout = system_table.stdout();
    // On nettoie l'écran textuel natif
    let _ = stdout.clear();
    // On met le texte en vert clair sur fond noir (style terminal classique / rétro)
    let _ = stdout.set_color(
        uefi::proto::console::text::Color::LightGreen,
        uefi::proto::console::text::Color::Black,
    );
}
