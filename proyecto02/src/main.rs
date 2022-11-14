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


use crate::save_disk::write_pixels;


use std::env;
use std::ffi::OsStr;
use image;
use quircs;




fn main() {
    let caca: Vec<u8> = vec![1,0,1,1,0,1,0,1,1,0,1,0,0,1,0,1,0,1,0,1];



    write_pixels(5,4,caca);
}
