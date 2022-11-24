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


use crate::save_disk::{load_disk, validate_fs_path};
use std::env;
use std::mem;
use sysinfo::{System, SystemExt};



fn main() {
    let save_path = env::args().nth(1).unwrap();
    if validate_fs_path(save_path.clone()) == false {
        println!("Direccion no valida!");
        return;
    }
    let disk = load_disk(save_path.clone()).unwrap();
    println!("---------------------------------CHARGING OLD DISK---------------------------------");
    let mut sys = System::new_all();
    sys.refresh_all();
    println!("=> System:");
    println!("total memory: {} KB", sys.total_memory());
    println!("used memory : {} KB", sys.used_memory());
    println!("BWFS SPACE USED : {} KB", mem::size_of_val(&disk));
    println!("BWFS::MEMORY BLOCK SPACE USED : {} KB", mem::size_of_val(&disk.inodes_block)*&disk.inodes_block.len());
    println!("BWFS::SUPER BLOCK SPACE USED : {} KB", mem::size_of_val(&disk.memory_block)*&disk.memory_block.len());
    println!("BWFS SPACE AVAILABLE : {} KB", sys.total_memory()-sys.used_memory());

}
