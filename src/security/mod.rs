use uefi::prelude::*;
use uefi::proto::console::text::Key;
use uefi::CStr16;
use uefi::cstr16;
use crate::drivers::fs::{list_root_directory, cat_target_file};

static mut CMD_BUFFER: [char; 64] = ['\0'; 64];
static mut CMD_LEN: usize = 0;

// On passe system_table à la fonction pour que ls/cat puissent l'utiliser
type CommandFn = unsafe fn(&mut SystemTable<Boot>);

const COMMANDS: &[(&str, CommandFn)] = &[
    ("clear", |st| { let _ = st.stdout().clear(); }),
    ("help",  |st| { let _ = st.stdout().output_string(cstr16!("Commands: clear, help, uname, ls, cat\r\n")); }),
    ("uname", |st| { let _ = st.stdout().output_string(cstr16!("LAZE microkernel 0.1.0-axis\r\n")); }),
    ("ls",    list_root_directory),
    ("cat",   cat_target_file),
];

pub fn run_shell(system_table: &mut SystemTable<Boot>) {
    let stdin = system_table.stdin();
    if let Ok(Some(key)) = stdin.read_key() {
        // Au lieu de garder 'stdout' en variable ici, on l'utilise à la volée
        // pour que la référence soit libérée immédiatement.
        match key {
            Key::Printable(ch) => {
                let c = char::from(ch);
                if c == '\r' || c == '\n' {
                    let _ = system_table.stdout().output_string(cstr16!("\r\n"));
                    
                    // Ici, system_table est libre car l'emprunt précédent est terminé
                    unsafe { process_command(system_table); }
                    
                    let _ = system_table.stdout().output_string(cstr16!("sasori@laze:~$ "));
                } else if c == '\x08' {
                    unsafe {
                        if CMD_LEN > 0 {
                            CMD_LEN -= 1;
                            CMD_BUFFER[CMD_LEN] = '\0';
                            let _ = system_table.stdout().output_string(cstr16!("\x08 \x08"));
                        }
                    }
                } else {
                    unsafe {
                        if CMD_LEN < 63 {
                            CMD_BUFFER[CMD_LEN] = c;
                            CMD_LEN += 1;
                            let mut buf = [0u16; 2];
                            c.encode_utf16(&mut buf);
                            if let Ok(cstr) = CStr16::from_u16_with_nul(&[buf[0], 0]) {
                                let _ = system_table.stdout().output_string(cstr);
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }
}
unsafe fn process_command(system_table: &mut SystemTable<Boot>) {
    let mut found = false;
    for (name, func) in COMMANDS {
        if matches_cmd(name) {
            func(system_table);
            found = true;
            break;
        }
    }
    if !found && CMD_LEN > 0 {
        let _ = system_table.stdout().output_string(cstr16!("laze: command not found\r\n"));
    }
    CMD_LEN = 0;
    CMD_BUFFER = ['\0'; 64];
}

unsafe fn matches_cmd(expected: &str) -> bool {
    if expected.len() != CMD_LEN { return false; }
    for (i, c) in expected.chars().enumerate() {
        if CMD_BUFFER[i] != c { return false; }
    }
    true
}
