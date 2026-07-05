use uefi::prelude::*;
use uefi::proto::console::gop::GraphicsOutput;

pub fn init_laze_screen(boot_services: &BootServices) {
    if let Ok(gop_handle) = boot_services.get_handle_for_protocol::<GraphicsOutput>() {
        if let Ok(mut gop) = boot_services.open_protocol_exclusive::<GraphicsOutput>(gop_handle) {
            let mode_info = gop.current_mode_info();
            let width = mode_info.resolution().0;
            let height = mode_info.resolution().1;
            
            let mut framebuffer = gop.frame_buffer();
            let fb_ptr = framebuffer.as_mut_ptr();
            
            // Fond Bleu Nuit Paresseux
            for y in 0..height {
                for x in 0..width {
                    let pixel_offset = ((y * width + x) * 4) as isize;
                    unsafe {
                        *fb_ptr.offset(pixel_offset)     = 0x24;
                        *fb_ptr.offset(pixel_offset + 1) = 0x16;
                        *fb_ptr.offset(pixel_offset + 2) = 0x12;
                    }
                }
            }

            // Zone applicative isolée grise
            for y in 150..450 {
                for x in 200..600 {
                    let pixel_offset = ((y * width + x) * 4) as isize;
                    unsafe {
                        *fb_ptr.offset(pixel_offset)     = 0xEE;
                        *fb_ptr.offset(pixel_offset + 1) = 0xEE;
                        *fb_ptr.offset(pixel_offset + 2) = 0xEE;
                    }
                }
            }
        }
    }
}
