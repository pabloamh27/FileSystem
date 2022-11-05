use fuse::{Filesystem, Request, ReplyCreate, ReplyEmpty, ReplyAttr, ReplyEntry, ReplyOpen, ReplyStatfs,ReplyData, ReplyDirectory, ReplyWrite, FileType, FileAttr};
use libc::{ENOSYS, ENOENT, EIO, EISDIR, ENOSPC};
use std::ffi::OsStr;
use std::mem;
use crate::mkfs;
use serde::{Serialize, Deserialize};
use crate::ses_infor::FileAttrDef;
use qrcode::QrCode;
use image::Luma;
use crate::Disk::*;
use crate::SaveDisk::*;
use crate::Inode::*;

#[path = "src/fsstructure/Inode.rs"] use Inode;
#[path = "src/fsstructure/Disk.rs"] use Disk;
#[path = "src/fsstructure/SaveDisk.rs"] use SaveDisk;

//Nuestro fs tiene un disco
pub struct Rb_fs {
    disk : Disk
}
impl Rb_fs {
    pub fn new(root_path:String, disk_path:String, path_save:String) -> Self{
        //Falta verificar si hay que agregar crear un nuevo disco o cargarlo, las funciones ya estan
        let new_disk = Disk::new(root_path.to_string(), disk_path, path_save);
        Rb_fs {
            disk : new_disk
        }
    }

    pub fn get_disk(&self) -> &Disk {
        return &self.disk;
    }

    pub fn set_disk(&mut self,new_disk:Disk) {
        self.disk = new_disk;
    }

    pub fn save_fs(&self){
        let encode_fs = encode(&self.disk);
        save_to_qr(self.disk.path_save.clone(),encode_fs);
    }

}

impl Drop for Rb_fs {
    fn drop(&mut self) {
        &self.save_fs();
        println!("---RB-FS SAVED---!");
    }
}

impl Filesystem for Rb_fs {

    //Mira dentro de un directorio por su nombre y obtiene sus atributos
    fn lookup(&mut self, _req: &Request, parent: u64, name: &OsStr, reply: ReplyEntry) {

        let fila_name = name.to_str().unwrap();
        let inode = self.disk.find_inode_in_references_by_name(parent, fila_name);
        match inode {
            Some(inode) => {
                let ttl = time::now().to_timespec();
                reply.entry(&ttl, &inode.attributes, 0);
                println!("----RB-FS: LOOKUP----");
            },
            None => {
                reply.error(ENOENT);
            }
        }
    }
    //Crea un archivo en la padre pasado poor parametro
    fn create(&mut self, _req: &Request, parent: u64, name: &OsStr, mode: u32, flags: u32, reply: ReplyCreate) {

        let ino_available = self.disk.new_ino();
        let MemoryBlock = MemoryBlock {
            ino_ref : ino_available,
            data : Vec::new()
        };

        let ts = time::now().to_timespec();

        let attr = FileAttr {
            ino: ino_available,
            size: 0,
            blocks: 1,
            atime: ts,
            mtime: ts,
            ctime: ts,
            crtime: ts,
            kind: FileType::RegularFile,
            perm: 0o755,
            nlink: 0,
            uid: 0,
            gid: 0,
            rdev: 0,
            flags,
        };

        let name = name.to_str().unwrap();

        let mut inode = Inode {
            name: name.to_string(),
            attributes: attr,
            references: Vec::new()
        };

        inode.references.push(MemoryBlock.ino_ref as usize);

        self.disk.write_ino(inode);

        self.disk.add_reference(parent, ino_available as usize);
        self.disk.memory_block.push(MemoryBlock);
        println!("----RB-FS: CREATED----");

        reply.created(&ts, &attr, 1, ino_available, flags)
    }

    //Escribe dentro de un archivo en base al ino pasado
    fn write(&mut self, _req: &Request, ino: u64, _fh: u64, offset: i64, data: &[u8], _flags: u32, reply: ReplyWrite) {

        let inode = self.disk.get_mut_inode(ino);
        let content: Vec<u8> = data.to_vec();

        match inode {
            Some(inode) => {
                inode.attributes.size = data.len() as u64;
                self.disk.write_content(ino, content);
                println!("----RB-FS: WRITE----");

                reply.written(data.len() as u32);
            },
            None => {
                reply.error(ENOENT);
            }
        }
    }
    //Busca el bloque de memoria asignado al ino y muestra su contenido
    fn read(&mut self, _req: &Request, ino: u64, fh: u64, offset: i64, size: u32, reply: ReplyData) {
        let memory_block = self.disk.get_bytes_content(ino);
        match memory_block {
            Some(memory_block) => {reply.data(memory_block);
                println!("----RB-FS: READ----");

            },
            None => {reply.error(EIO);}
        }
    }
    //Funcion para cambiar de nombre un archivo mediante el padre
    fn rename(&mut self, _req:&Request, parent:u64, name:&OsStr, _newparent: u64, newname:&OsStr, reply:ReplyEmpty) {
        let name = name.to_str().unwrap();
        let inode :Option<&Inode> = self.disk.find_inode_in_references_by_name(parent, name);
        match inode {
            Some(inode) => {
                let ino = inode.attributes.ino;
                let child = self.disk.get_mut_inode(ino);
                match child {
                    Some(child) => {
                        println!("----RB-FS: RENAME----");
                        child.name = newname.to_str().unwrap().to_string();
                        reply.ok()
                    },
                    None => {println!("-------RENAME ERROR-------------");}
                }
            },
            None =>{reply.error(ENOENT);}
        }
    }
    //Busca el inode asignado al ino y devuelve sus atributos
    fn getattr(&mut self,_req: &Request, ino: u64, reply: ReplyAttr) {
        let inode = self.disk.get_inode(ino);
        match inode {
            Some(inode) => {
                let ttl = time::now().to_timespec();
                println!("----RB-FS: GETATTR----");

                reply.attr(&ttl, &inode.attributes);
            },
            None => reply.error(ENOENT)
        }
    }
    //Literalmente, lee un directorio
    fn readdir(&mut self, _req: &Request, ino: u64, fh: u64, offset: i64, mut reply: ReplyDirectory) {
        println!("----RB-FS: READDIR----");

        if ino == 1 {
            if offset == 0 {
                reply.add(1, 0, FileType::Directory, ".");
                reply.add(1, 1, FileType::Directory, "..");

            }
        }

        let inode: Option<&Inode> = self.disk.get_inode(ino);
        if mem::size_of_val(&inode) == offset as usize {
            reply.ok();
            return;
        }

        match inode {
            Some(inode) => {
                let references = &inode.references;

                for ino in references {

                    if let ino = ino {
                        let inode = self.disk.get_inode(*ino as u64);

                        if let Some(inode_data) = inode {
                            if inode_data.attributes.ino == 1 {
                                continue;
                            }

                            let name = &inode_data.name;
                            let offset = mem::size_of_val(&inode) as i64;
                            reply.add(inode_data.attributes.ino, offset, inode_data.attributes.kind, name);
                        }
                    }
                }

                reply.ok()
            },
            None => { println!("ERROR ino={:?}", ino); reply.error(ENOENT) }
        }
    }

    //Crea un directorio y asigna un nuevo ino
    fn mkdir(&mut self, _req: &Request, parent: u64, name: &OsStr, _mode: u32, reply: ReplyEntry) {
        println!("----RB-FS: MKDIR----");

        let ino = self.disk.new_ino();
        let ts = time::now().to_timespec();
        let attr = FileAttr {
            ino: ino as u64,
            size: 0,
            blocks: 1,
            atime: ts,
            mtime: ts,
            ctime: ts,
            crtime: ts,
            kind: FileType::Directory,
            perm: 0o755,
            nlink: 0,
            uid: 0,
            gid: 0,
            rdev: 0,
            flags: 0,
        };

        let name = name.to_str().unwrap().to_string();


        let inode = Inode {
            name: name,
            attributes: attr,
            references: Vec::new()
        };

        self.disk.write_ino(inode);
        self.disk.add_reference(parent,ino as usize);

        reply.entry(&ts, &attr, 0);
    }
    //Elimina un directorio en base al nombre
    fn rmdir(&mut self,_req: &Request, parent: u64, name: &OsStr, reply: ReplyEmpty) {
        println!("----RB-FS: RMDIR----");

        let name = name.to_str().unwrap();
        let inode = self.disk.find_inode_in_references_by_name(parent, name);

        match inode {
            Some(inode) => {
                let ino = inode.attributes.ino;
                self.disk.clear_reference(parent, ino as usize);
                self.disk.remove_inode(ino);

                reply.ok();
            },
            None => reply.error(EIO)
        }
    }
    //Devuelve las estadistcas del filesystem *no funciona bien XD
    fn statfs(&mut self, _req: &Request, _ino: u64, reply: ReplyStatfs) {
        println!("----RB-FS: STATFS----");

        let mut blocks:u64 =  (self.disk.super_block.len() +self.disk.memory_block.len()) as u64;
        let mut bfree:u64 = blocks - self.disk.memory_block.len() as u64;
        let mut bavail:u64 = bfree;
        let mut files:u64 = self.disk.memory_block.len().try_into().unwrap();
        let mut ffree:u64 = 1024 as u64;
        let mut bsize:u32 = (mem::size_of::<Vec<Inode>>() as u32 +mem::size_of::<Inode>() as u32)*1024;
        let mut namelen:u32 = 77;
        let mut frsize:u32 = 1;

        reply.statfs(blocks,
                     bfree,
                     bavail,
                     files,
                     ffree,
                     bsize,
                     namelen,
                     frsize);
    }

    //Si datasync != 0, solo se deben vaciar los datos del usuario, no los metadatos.
    fn fsync(&mut self, _req: &Request, ino: u64, fh: u64, datasync: bool, reply: ReplyEmpty) {
        println!("----RB-FS: FSYNC----");
        reply.error(ENOSYS);
    }
    //Abre un directorio
    fn opendir(&mut self, _req: &Request, _ino: u64, _flags: u32, reply: ReplyOpen) {
        let dir = self.disk.get_inode(_ino);
        match dir {
            Some(dir) => {
                println!("----RB-FS OPENDIR----");
                reply.opened(dir.attributes.ino, 1 as u32);
            },
            None => {println!("-------CANT OPEN-------")}
        }

    }

    //Revisa el acceso de los permisos
    fn access(&mut self, _req: &Request, _ino: u64, _mask: u32, reply: ReplyEmpty) {
        println!("----RB-FS: ACCESS----");
        reply.ok();
    }



}
