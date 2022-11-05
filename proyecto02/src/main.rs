//Imports de la estructura de datos
#[path = "filestructure/fileAttribute.rs"] mod fileAttribute;
#[path = "filestructure/fileTimeSpec.rs"] mod fileTimeSpec;
#[path = "filestructure/fileType.rs"] mod fileType;
//Imports de la estructura de el filesystem
#[path = "fsstructure/Disk.rs"] mod Disk;
#[path = "fsstructure/Inode.rs"] mod Inode;
#[path = "fsstructure/MemoryBlock.rs"] mod MemoryBlock;
#[path = "fsstructure/FilesystemManagement.rs"] mod FilesystemManagement;
#[path = "fsstructure/SaveDisk.rs"] mod SaveDisk;


use std::env;
use std::ffi::OsStr;
use image;
use quircs;

fn main() {
    println!("Hello, world!");
}
