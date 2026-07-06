#![no_std]
#![no_main]

use core::panic::PanicInfo;
use uefi::prelude::*;
use uefi::cstr16;

mod drivers;
mod security;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! { loop {} }

#[entry]
fn main(_image_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    uefi::helpers::init().unwrap();

    drivers::screen::init_laze_screen(&mut system_table);
    drivers::memory::map_laze_memory(system_table.boot_services());
    drivers::fs::FS_MANAGER.get_mut().init(system_table.boot_services());

    // Prompt initial
    let _ = system_table.stdout().output_string(cstr16!("sasori@laze:~$ "));

    loop {
        security::run_shell(&mut system_table);
    }
}
