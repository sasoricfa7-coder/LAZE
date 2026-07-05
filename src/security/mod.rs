use uefi::prelude::*;
use uefi::proto::console::text::Key;
use uefi::CStr16;
use uefi::cstr16;

// Tampon global fixe pour stocker les caractères de la commande en cours (max 64 caractères)
static mut CMD_BUFFER: [char; 64] = ['\0'; 64] ;
static mut CMD_LEN: usize = 0;

pub fn run_shell(system_table: &mut SystemTable<Boot>) {
    let stdin = system_table.stdin();
    let mut key_event: Option<Key> = None;

    if let Ok(Some(key)) = stdin.read_key() {
        key_event = Some(key);
    }

    if let Some(key) = key_event {
        let stdout = system_table.stdout();

        match key {
            Key::Printable(ch) => {
                let c = char::from(ch);

                // Si c'est Entrée (\r ou \n selon l'émulateur/clavier)
                if c == '\r' || c == '\n' {
                    let _ = stdout.output_string(cstr16!("\r\n"));
                    
                    // Traitement de la commande
                    unsafe {
                        process_command(stdout);
                    }
                    
                    // Réaffichage du prompt
                    let _ = stdout.output_string(cstr16!("sasori@laze:~$ "));
                } 
                // Si c'est Retour arrière (Backspace)
                else if c == '\x08' {
                    unsafe {
                        if CMD_LEN > 0 {
                            CMD_LEN -= 1;
                            CMD_BUFFER[CMD_LEN] = '\0';
                            // Recule le curseur, écrit un espace pour effacer, et recule à nouveau
                            let _ = stdout.output_string(cstr16!("\x08 \x08"));
                        }
                    }
                } 
                // Caractère standard : on l'ajoute au buffer si on a de la place
                else {
                    unsafe {
                        if CMD_LEN < 63 {
                            CMD_BUFFER[CMD_LEN] = c;
                            CMD_LEN += 1;
                            
                            // Affichage immédiat à l'écran
                            let mut buf = [0u16; 3];
                            let len = c.encode_utf16(&mut buf[0..2]).len();
                            buf[len] = 0;
                            if let Ok(cstr) = CStr16::from_u16_with_nul(&buf[0..=len]) {
                                let _ = stdout.output_string(cstr);
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

// Fonction de traitement des mots-clés de commandes Linux
unsafe fn process_command(stdout: &mut uefi::proto::console::text::Output) {
    // Vérification de la commande "clear"
    if matches_cmd("clear") {
        let _ = stdout.clear();
    } 
    // Vérification de la commande "help"
    else if matches_cmd("help") {
        let _ = stdout.output_string(cstr16!("Available LAZE commands: help, clear, uname, exit\r\n"));
    } 
    // Vérification de la commande "uname"
    else if matches_cmd("uname") {
        let _ = stdout.output_string(cstr16!("LAZE microkernel 0.1.0-axis x86_64 uefi-mode\r\n"));
    } 
    // Commande inconnue (sauf si vide)
    else if CMD_LEN > 0 {
        let _ = stdout.output_string(cstr16!("laze: command not found\r\n"));
    }

    // Réinitialisation complète du buffer pour la commande suivante
    CMD_LEN = 0;
    CMD_BUFFER = ['\0'; 64];
}

// Helper pour comparer le buffer statique brut avec une chaîne de caractères classique
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
