use uefi::prelude::*;
use uefi::proto::console::gop::{GraphicsOutput, BltOp, BltPixel};

// Une police système 8x8 très basique pour quelques caractères (A-Z, 0-9, Espace, Entrée)
// Chaque octet représente une ligne de 8 pixels (1 = pixel allumé, 0 = éteint)
const FONT_8X8: [[u8; 8]; 4] = [
    [0x3C, 0x66, 0x66, 0x7E, 0x66, 0x66, 0x66, 0x00], // 'A' (Index 0)
    [0x7C, 0x66, 0x66, 0x7C, 0x66, 0x66, 0x7C, 0x00], // 'B' (Index 1)
    [0x3C, 0x66, 0x06, 0x06, 0x06, 0x66, 0x3C, 0x00], // 'C' (Index 2)
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], // ' ' (Espace - Index 3)
];

pub fn init_laze_screen(boot_services: &BootServices) {
    if let Ok(gop_handle) = boot_services.get_handle_for_protocol::<GraphicsOutput>() {
        if let Ok(mut gop) = boot_services.open_protocol_exclusive::<GraphicsOutput>(gop_handle) {
            
            // Fond sombre
            let _ = gop.blt(BltOp::VideoFill {
                color: BltPixel::new(15, 23, 42),
                dest: (0, 0),
                dims: (1024, 768),
            });

            // Terminal blanc
            let _ = gop.blt(BltOp::VideoFill {
                color: BltPixel::new(241, 245, 249),
                dest: (150, 150),
                dims: (400, 300),
            });
        }
    }
}

// Dessine un caractère pixel par pixel sur l'écran
pub fn draw_char(boot_services: &BootServices, c: char, x: usize, y: usize) {
    let font_index = match c {
        'A' | 'a' => 0,
        'B' | 'b' => 1,
        'C' | 'c' => 2,
        _ => 3, // Espace par défaut pour le test
    };

    if let Ok(gop_handle) = boot_services.get_handle_for_protocol::<GraphicsOutput>() {
        if let Ok(mut gop) = boot_services.open_protocol_exclusive::<GraphicsOutput>(gop_handle) {
            let glyph = FONT_8X8[font_index];
            
            for row in 0..8 {
                let bits = glyph[row];
                for col in 0..8 {
                    // Si le bit est à 1, on dessine un pixel noir, sinon on laisse le fond blanc
                    let color = if (bits & (0x80 >> col)) != 0 {
                        BltPixel::new(0, 0, 0) // Noir
                    } else {
                        BltPixel::new(241, 245, 249) // Couleur du terminal
                    };

                    let _ = gop.blt(BltOp::VideoFill {
                        color,
                        dest: (x + col, y + row),
                        dims: (1, 1),
                    });
                }
            }
        }
    }
}
