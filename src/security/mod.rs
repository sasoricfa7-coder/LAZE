use uefi::prelude::*;

pub fn run_shell(system_table: &mut SystemTable<Boot>) {
    let stdin = system_table.stdin();
    
    if let Ok(Some(key)) = stdin.read_key() {
        match key {
            uefi::proto::console::text::Key::Printable(ch) => {
                let char_val = char::from(ch);
                if char_val == 'r' {
                    // La future sentence de LAZE
                }
            }
            _ => {}
        }
    }
}
