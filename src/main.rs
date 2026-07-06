#![no_std]
#![no_main]

use core::panic::PanicInfo;
use uefi::prelude::*;

mod drivers;
mod security;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[entry]
fn main(_image_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    uefi::helpers::init().unwrap();

    drivers::screen::init_laze_screen(&mut system_table);
    drivers::memory::map_laze_memory(system_table.boot_services());

    // Initialisation sécurisée sans bloc unsafe
    drivers::fs::FS_MANAGER.get_mut().init(system_table.boot_services());

    loop {
        security::run_shell(&mut system_table);
    }
}
