use uefi::prelude::*;
use uefi::proto::console::text::Key;
use uefi::CStr16;
use uefi::cstr16;
use uefi::table::runtime::ResetType;
use crate::drivers::fs::list_root_directory;

static mut CMD_BUFFER: [char; 64] = ['\0'; 64];
static mut CMD_LEN: usize = 0;

pub fn run_shell(system_table: &mut SystemTable<Boot>) {
    let stdin = system_table.stdin();
    let mut key_event: Option<Key> = None;

    if let Ok(Some(key)) = stdin.read_key() {
        key_event = Some(key);
    }

    if let Some(key) = key_event {
        match key {
            Key::Printable(ch) => {
                let c = char::from(ch);

                if c == '\r' || c == '\n' {
                    let _ = system_table.stdout().output_string(cstr16!("\r\n"));
                    unsafe {
                        process_command(system_table);
                    }
                    let _ = system_table.stdout().output_string(cstr16!("sasori@laze:~$ "));
                } 
                else if c == '\x08' {
                    unsafe {
                        if CMD_LEN > 0 {
                            CMD_LEN -= 1;
                            CMD_BUFFER[CMD_LEN] = '\0';
                            let _ = system_table.stdout().output_string(cstr16!("\x08 \x08"));
                        }
                    }
                } 
                else {
                    unsafe {
                        if CMD_LEN < 63 {
                            CMD_BUFFER[CMD_LEN] = c;
                            CMD_LEN += 1;
                            
                            let mut buf = [0u16; 3];
                            let len = c.encode_utf16(&mut buf[0..2]).len();
                            buf[len] = 0;
                            if let Ok(cstr) = CStr16::from_u16_with_nul(&buf[0..=len]) {
                                let _ = system_table.stdout().output_string(cstr);
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }

    system_table.boot_services().stall(10_000);
}

unsafe fn process_command(system_table: &mut SystemTable<Boot>) {
    let stdout = system_table.stdout();

    if matches_cmd("clear") {
        let _ = stdout.clear();
    } 
    else if matches_cmd("help") {
        let _ = stdout.output_string(cstr16!("Commands: help, clear, uname, pwd, whoami, reboot, poweroff, ls\r\n"));
    } 
    else if matches_cmd("uname") {
        let _ = stdout.output_string(cstr16!("Linux laze 6.2026-axis #1 SMP PREEMPT Sasori x86_64 GNU/Linux\r\n"));
    } 
    else if matches_cmd("pwd") {
        let _ = stdout.output_string(cstr16!("/home/sasori\r\n"));
    } 
    else if matches_cmd("whoami") {
        let _ = stdout.output_string(cstr16!("sasori\r\n"));
    } 
    // VRAI ACCÈS MATÉRIEL AU DISQUE ICI
    else if matches_cmd("ls") {
        list_root_directory(system_table);
    }
    else if matches_cmd("reboot") {
        let _ = stdout.output_string(cstr16!("Rebooting system via UEFI Runtime Services...\r\n"));
        system_table.runtime_services().reset(ResetType::COLD, Status::SUCCESS, None);
    } 
    else if matches_cmd("poweroff") || matches_cmd("shutdown") {
        let _ = stdout.output_string(cstr16!("Shutting down ACPI power state...\r\n"));
        system_table.runtime_services().reset(ResetType::SHUTDOWN, Status::SUCCESS, None);
    }
    else if CMD_LEN > 0 {
        let _ = stdout.output_string(cstr16!("laze: command not found\r\n"));
    }

    CMD_LEN = 0;
    CMD_BUFFER = ['\0'; 64];
}

unsafe fn matches_cmd(expected: &str) -> bool {
    if expected.len() != CMD_LEN {
        return false;
    }
    let mut i = 0;
    for c in expected.chars() {
        if CMD_BUFFER[i] != c {
            return false;
        }
        i += 1;
    }
    true
}
