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


use std::env;
use std::path::Path;
use std::fs;


fn main() {

    let save_path = env::args().nth(1).unwrap();
    if Path::new(save_path.as_str()).exists() {
        fs::remove_dir_all(save_path.clone()).unwrap();
    }
    fs::create_dir_all(save_path.clone()).unwrap();
    filesystem_management::BWFS::new(save_path.clone());
    println!("A new BWFS was created!");
    /* REVERSIBLE
        let mountpoint = env::args().nth(2).unwrap();
        fuse::mount(fs, &mountpoint, &options).unwrap();
     */
}
