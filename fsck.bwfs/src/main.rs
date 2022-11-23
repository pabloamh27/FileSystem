extern crate core;

//Imports de la estructura de datos
#[path = "filestructure/fileAttribute.rs"] mod fileAttribute;
#[path = "filestructure/fileTimeSpec.rs"] mod fileTimeSpec;
#[path = "filestructure/fileType.rs"] mod fileType;
//Imports de la estructura de el filesystem
#[path = "fsstructure/Disk.rs"] mod Disk;
#[path = "fsstructure/Inode.rs"] mod Inode;
#[path = "fsstructure/memory_block.rs"] mod memory_block;
#[path = "fsstructure/filesystem_management.rs"] mod filesystem_management;
#[path = "fsstructure/save_disk.rs"] mod save_disk;


use crate::save_disk::{load_disk, validate_fs_path, write_pixels};
use std::env;
use std::ffi::OsStr;
use image;
use quircs;
use std::mem;
use log::error;
use serde::de::Unexpected::Option;
use sysinfo::{NetworkExt, NetworksExt, ProcessExt, System, SystemExt};
use crate::filesystem_management::BWFS;


fn main() {
    let disk_direction = env::args().nth(1).unwrap();
    if validate_fs_path(disk_direction.clone()) == false {
        println!("Direccion no valida!");
        return;
    }
    let mut disk = load_disk(disk_direction.clone()).unwrap();
    println!("---------------------------------CHARGING OLD DISK---------------------------------");
    let mut sys = System::new_all();
    sys.refresh_all();
    println!("=> System:");
    println!("total memory: {} KB", sys.total_memory());
    println!("used memory : {} KB", sys.used_memory());
    println!("RB-FS SPACE USED : {} KB", mem::size_of_val(&disk));
    println!("RB-FS::MEMORY BLOCK SPACE USED : {} KB", mem::size_of_val(&disk.inodes_block)*&disk.inodes_block.len());
    println!("RB-FS::SUPER BLOCK SPACE USED : {} KB", mem::size_of_val(&disk.memory_block)*&disk.memory_block.len());
    println!("RB-FS SPACE AVAILABLE : {} KB", sys.total_memory()-sys.used_memory());

}
