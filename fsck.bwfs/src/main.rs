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
    println!("Cargando Disco Encontrado");
    let mut sys = System::new_all();
    sys.refresh_all();

    println!("==================================================================");
    println!("BWFS en nuestro sistema");
    println!("==================================================================");
    println!("Memoria total del sistema: {} KB", sys.total_memory());
    println!("Memoria utilizada en el sistema: {} KB", sys.used_memory());
    println!("Espacio usado por el BWFS: {} KB", mem::size_of_val(&disk));
    println!("==================================================================\n\n");

    println!("==================================================================");
    println!("BWFS en el disco");
    println!("==================================================================");
    println!("Espacio usado en memory block en el BWFS: {} KB", mem::size_of_val(&disk.inodes_block)*&disk.inodes_block.len());
    println!("Espacio usado en el superbloque en el BWFS: {} KB", mem::size_of_val(&disk.memory_block)*&disk.memory_block.len());
    println!("Espacio disponible: {} KB", sys.total_memory()-sys.used_memory());
    println!("==================================================================");

}
