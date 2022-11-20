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

/*
    let caca: Vec<u8> = vec![1,0,1,1,0,1,0,1,1,1,1,1,0,0,1,0,1,1,0,1,0,1,0,1,1,1,1,1,1,1];
    write_pixels(2, 2, caca, "/home/estudiante/Escritorio/S.O/proyecto2/FileSystem/proyecto02/src/output", 0, 0);
 */
    let disk_direction = env::args().nth(1).unwrap();
    let mountpoint = env::args().nth(2).unwrap();
    let disk_to_save = env::args().nth(3).unwrap();
    let fs = filesystem_management::Rb_fs::new(mountpoint.clone(), disk_direction.clone(), disk_to_save.clone());
    //fsck::check_consistens(&fs);
    let options = ["-o", "nonempty"].iter().map(|o| o.as_ref()).collect::<Vec<&OsStr>>();
    println!("RB-FS started!");
    fuse::mount(fs, &mountpoint, &options).unwrap();

}
