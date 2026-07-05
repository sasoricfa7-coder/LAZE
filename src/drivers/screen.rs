use uefi::prelude::*;
use uefi::proto::console::gop::{GraphicsOutput, BltOp, BltPixel};

pub fn init_laze_screen(boot_services: &BootServices) {
    if let Ok(gop_handle) = boot_services.get_handle_for_protocol::<GraphicsOutput>() {
        if let Ok(mut gop) = boot_services.open_protocol_exclusive::<GraphicsOutput>(gop_handle) {
            
            // Fond d'écran bleu LAZE
            let _ = gop.blt(BltOp::VideoFill {
                color: BltPixel::new(15, 23, 42),
                dest: (0, 0),
                dims: (1024, 768),
            });

            // Notre zone blanche centrale (le terminal)
            let _ = gop.blt(BltOp::VideoFill {
                color: BltPixel::new(241, 245, 249),
                dest: (150, 150),
                dims: (300, 250),
            });
        }
    }
}

pub fn flash_security_alert(boot_services: &BootServices) {
    if let Ok(gop_handle) = boot_services.get_handle_for_protocol::<GraphicsOutput>() {
        if let Ok(mut gop) = boot_services.open_protocol_exclusive::<GraphicsOutput>(gop_handle) {
            let _ = gop.blt(BltOp::VideoFill {
                color: BltPixel::new(239, 68, 68), // Rouge alerte
                dest: (150, 150),
                dims: (300, 250),
            });
        }
    }
}
