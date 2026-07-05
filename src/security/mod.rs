use uefi::prelude::*;
use uefi::CStr16;

pub fn run_shell(system_table: &mut SystemTable<Boot>) {
    let stdin = system_table.stdin();
    let mut key_to_print: Option<char> = None;

    if let Ok(Some(key)) = stdin.read_key() {
        match key {
            uefi::proto::console::text::Key::Printable(ch) => {
                key_to_print = Some(char::from(ch));
            }
            _ => {}
        }
    }

    if let Some(c) = key_to_print {
        let stdout = system_table.stdout();
        
        // Initialisé à 0, donc le \0 terminal est garanti dès le départ
        let mut buf = [0u16; 3]; 
        let len = c.encode_utf16(&mut buf[0..2]).len();

        // On crée le CStr16 sur la partie écrite + le zéro qui suit
        if let Ok(cstr) = CStr16::from_u16_with_nul(&buf[0..=len]) {
            let _ = stdout.output_string(cstr);
        }
    }

    system_table.boot_services().stall(10_000);
}
