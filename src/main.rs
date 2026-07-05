#![no_std]
#![no_main]

use core::fmt::Write;
use core::panic::PanicInfo;
use uefi::prelude::*;
use uefi::proto::console::text::Key;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[entry]
fn main(_image_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    uefi::helpers::init().unwrap();

    // On efface l'écran directement
    system_table.stdout().clear().unwrap();

    writeln!(system_table.stdout(), "LAZE > OS initialise en UEFI 64 bits natif !").unwrap();
    write!(system_table.stdout(), "LAZE > ").unwrap();

    loop {
        // On effectue un emprunt court de stdin juste pour lire la touche
        if let Ok(Some(key)) = system_table.stdin().read_key() {
            match key {
                Key::Printable(character) => {
                    let c = u16::from(character) as u8 as char;
                    if c == '\r' {
                        writeln!(system_table.stdout()).unwrap();
                        write!(system_table.stdout(), "LAZE > ").unwrap();
                    } else {
                        // L'emprunt précédent est fini, on peut réutiliser stdout ici
                        write!(system_table.stdout(), "{}", c).unwrap();
                    }
                }
                _ => {}
            }
        }
    }
}
