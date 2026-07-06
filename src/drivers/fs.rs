use core::cell::UnsafeCell;
use uefi::prelude::*;
use uefi::proto::media::fs::SimpleFileSystem;
use uefi::proto::media::file::{File, FileAttribute, FileMode};
use uefi::{CStr16, cstr16, Identify};
use uefi::table::boot::SearchType;

pub struct FileSystemManager {
    pub working_volume: Option<uefi::Handle>,
}

impl FileSystemManager {
    pub fn init(&mut self, boot_services: &BootServices) {
        if let Ok(handles) = boot_services.locate_handle_buffer(SearchType::ByProtocol(&SimpleFileSystem::GUID)) {
            for handle in handles.iter() {
                if let Ok(mut fs) = boot_services.open_protocol_exclusive::<SimpleFileSystem>(*handle) {
                    if let Ok(mut root) = fs.open_volume() {
                        if root.open(cstr16!("LAZE.TXT"), FileMode::Read, FileAttribute::empty()).is_ok() {
                            self.working_volume = Some(*handle);
                            break;
                        }
                    }
                }
            }
        }
    }
}

pub struct SafeFSManager(UnsafeCell<FileSystemManager>);
unsafe impl Sync for SafeFSManager {}

pub static FS_MANAGER: SafeFSManager = SafeFSManager(UnsafeCell::new(FileSystemManager {
    working_volume: None,
}));

impl SafeFSManager {
    pub fn get_mut(&self) -> &mut FileSystemManager {
        unsafe { &mut *self.0.get() }
    }
}

pub fn list_root_directory(system_table: &mut SystemTable<Boot>) {
    if let Some(handle) = FS_MANAGER.get_mut().working_volume {
        let mut fs = system_table.boot_services().open_protocol_exclusive::<SimpleFileSystem>(handle).unwrap();
        let mut root = fs.open_volume().unwrap();
        drop(fs);

        let stdout = system_table.stdout();
        let _ = stdout.output_string(cstr16!("Fichiers :\r\n"));

        let mut buf = [0u8; 256];
        loop {
            match root.read_entry(&mut buf) {
                Ok(Some(info)) => {
                    let name = info.file_name();
                    if name != cstr16!(".") && name != cstr16!("..") {
                        let _ = stdout.output_string(name);
                        let _ = stdout.output_string(cstr16!("  "));
                    }
                }
                _ => break,
            }
        }
        let _ = stdout.output_string(cstr16!("\r\n"));
    }
}

pub fn cat_target_file(system_table: &mut SystemTable<Boot>) {
    let mut buf = [0u8; 128];
    let mut read_len = 0;

    if let Some(handle) = FS_MANAGER.get_mut().working_volume {
        let mut fs = system_table.boot_services().open_protocol_exclusive::<SimpleFileSystem>(handle).unwrap();
        let mut root = fs.open_volume().unwrap();
        drop(fs);

        if let Ok(file_handle) = root.open(cstr16!("LAZE.TXT"), FileMode::Read, FileAttribute::empty()) {
            if let Some(mut file) = file_handle.into_regular_file() {
                read_len = file.read(&mut buf).unwrap_or(0);
            }
        }
    }

    let stdout = system_table.stdout();
    for i in 0..read_len {
        let cstr = [buf[i] as u16, 0];
        let _ = stdout.output_string(CStr16::from_u16_with_nul(&cstr).unwrap());
    }
    let _ = stdout.output_string(cstr16!("\r\n"));
}
