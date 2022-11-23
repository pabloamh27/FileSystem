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




fn main() {

/*
    let caca: Vec<u8> = vec![1,0,1,1,0,1,0,1,1,1,1,1,0,0,1,0,1,1,0,1,0,1,0,1,1,1,1,1,1,1];
    write_pixels(2, 2, caca, "/home/estudiante/Escritorio/S.O/proyecto2/FileSystem/proyecto02/src/output", 0, 0);
 */
    let disk_direction = env::args().nth(1).unwrap();
    let mountpoint = env::args().nth(2).unwrap();
    let mut  fs = filesystem_management::BWFS::new(mountpoint.clone(), disk_direction.clone());
    if validate_fs_path(disk_direction.clone()) {
        let mut disk = load_disk(disk_direction.clone());
        fs = filesystem_management::BWFS::load(disk.unwrap(), fs);
        println!("---------------------------------CHARGING OLD DISK---------------------------------");
    } else {
        println!("---------------------------------CREATING NEW DISK---------------------------------");

    }
    let options = ["-o", "nonempty"].iter().map(|o| o.as_ref()).collect::<Vec<&OsStr>>();
    println!("BWFS started!");
    fuse::mount(fs, &mountpoint, &options).unwrap();
}