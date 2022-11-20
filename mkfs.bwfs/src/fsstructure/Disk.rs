use fuse::{Filesystem, Request, ReplyCreate, ReplyEmpty, ReplyAttr, ReplyEntry, ReplyOpen, ReplyStatfs,ReplyData, ReplyDirectory, ReplyWrite, FileType, FileAttr};
use libc::{ENOSYS, ENOENT, EIO, EISDIR, ENOSPC};
use std::ffi::OsStr;
use std::mem;
use serde::{Serialize, Deserialize};
use image::Luma;
use crate::fileAttribute::*;
use crate::Inode::*;
use crate::memory_block::*;
use crate::save_disk::*;


//Creamos una estructura para guardar nuestros archivos Inodes
#[derive(Serialize, Deserialize)]
pub struct Disk {
    pub inodes_block: Vec<Inode>,
    pub memory_block : Vec<MemoryBlock>,
    pub root_path: String,
    pub path_save: String
}
impl Disk {
    //Crea un nuevo disco y crea el inode raiz
    pub fn new(path:String, disk_path:String, path_to_save:String) -> Disk{

        println!("-----CREATING DISK------");
        unsafe{
            let mut memory_block = Vec::new();
            let mut blocks = Vec::new(); //Aca guardamos los inodes
            let time = time::now().to_timespec();
            let attributes = FileAttr {
                ino: 1,
                size: 0,
                blocks: 0,
                atime: time,
                mtime: time,

                ctime: time,
                crtime: time,
                kind: FileType::Directory,
                perm: 0o755,
                nlink: 0,
                uid: 0,
                gid: 0,
                rdev: 0,
                flags: 0,
            };
            let name = "Empty";
            let first_node = Inode {
                name : name.to_string(),
                attributes,
                references: Vec::new()
            };

            blocks.push(first_node);

            let new_disk = Disk { inodes_block: blocks, memory_block,root_path :  path, path_save:path_to_save};
            if validate_path(disk_path.clone()) {
                println!("------WE FOUND A DISK TO LOAD------");
                let disk_to_load = load_fs(disk_path);
                match disk_to_load {
                    Some(disk_to_load) => {
                        return disk_to_load;
                    },
                    None => {
                        println!("------CRETING NEW DISK---------");
                        return new_disk;
                    }
                }
            }
            println!("-----CRETING NEW DISK--------");
            return new_disk;


        }
    }

    pub fn get_next_ino(&mut self) -> u64 {
        return (self.inodes_block.len() + 1) as u64;
    }


    //Agrega el inode al super bloque
    pub fn write_ino(&mut self, inode:Inode) {
        self.inodes_block.push(inode);
    }

    //Elimina el inode disponible
    pub fn remove_inode(&mut self, inode:u64) {
        self.inodes_block.retain(|i| i.attributes.ino != inode);
    }

    //Elimina una referencia de un respectivo inode
    pub fn clear_reference(&mut self, ino: u64, ref_value: usize) {
        for i in 0..self.inodes_block.len() {
            if self.inodes_block[i].attributes.ino == ino {
                self.inodes_block[i.clone()].delete_reference(ref_value.clone());
            }
        }
    }

    //Agrega una respectiva referencia a un inode
    pub fn add_reference(&mut self, ino: u64, ref_value: usize) {
        for i in 0..self.inodes_block.len() {
            if self.inodes_block[i].attributes.ino == ino {
                self.inodes_block[i.clone()].add_reference(ref_value.clone());
            }
        }
    }

    //Obtiene un Inode o nada
    pub fn get_inode(&self, ino: u64) -> Option<&Inode> {
        for i in 0..self.inodes_block.len() {
            if self.inodes_block[i].attributes.ino == ino {
                return Some(&self.inodes_block[i.clone()]);
            }
        }
        return None;
    }

    //Obtiene un Inode mutable o nada
    pub fn get_mut_inode(&mut self, ino: u64) -> Option<&mut Inode> {
        for i in 0..self.inodes_block.len() {
            if self.inodes_block[i].attributes.ino == ino {
                return Some(&mut self.inodes_block[i.clone()]);
            }
        }
        return None;
    }

    //Busca en base a la carpeta del padre el hijo que tenga el nombre por parametro
    pub fn find_inode_in_references_by_name(&self, parent_inode_ino: u64, name: &str) -> Option<&Inode> {
        for i in 0..self.inodes_block.len() {
            if self.inodes_block[i].attributes.ino == parent_inode_ino {
                let parent =  &self.inodes_block[i.clone()];
                for j in 0..parent.references.len() {
                    for k in 0..self.inodes_block.len() {
                        if self.inodes_block[k].attributes.ino == parent.references[j.clone()].try_into().unwrap() {
                            let child =  &self.inodes_block[k.clone()];
                            if child.name == name {
                                return Some(child);
                            }
                        }
                    }
                }
            }
        }
        return None;
    }

    //Agrega data al bloque de memoria asociado al ino
    pub fn add_data_to_inode(&mut self, ino:u64,data:u8) {
        for i in 0..self.memory_block.len() {
            if self.memory_block[i].ino_ref == ino {
                self.memory_block[i.clone()].add_data(data.clone()) ;
            }
        }
    }

    //Elimina la data el bloque de memoria asociado al ino
    pub fn delete_data_to_inode(&mut self, ino:u64,data: u8) {
        for i in 0..self.memory_block.len() {
            if self.memory_block[i].ino_ref == ino {
                self.memory_block[i.clone()].delete_data(data.clone());
            }
        }
    }

    //Escribe un arreglo de bites dentro de un inode
    pub fn write_content(&mut self, ino_ref: u64, content: Vec<u8>) {
        for i in 0..content.len(){
            self.add_data_to_inode(ino_ref.clone(), content[i].clone());

        }
    }

    //Obtiene el contenido de un arreglo
    pub fn get_bytes_content(&self, ino: u64) -> Option<&[u8]> {
        for i in 0..self.memory_block.len() {
            if self.memory_block[i].ino_ref == ino {
                let bytes = &self.memory_block[i.clone()].data[..];
                return Some(bytes);
            }
        }
        return None;
    }
}
