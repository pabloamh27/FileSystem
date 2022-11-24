use fuse::{Filesystem, Request, ReplyCreate, ReplyEmpty, ReplyAttr, ReplyEntry, ReplyOpen, ReplyStatfs,ReplyData, ReplyDirectory, ReplyWrite, FileType, FileAttr};
use libc::{ENOSYS, ENOENT, EIO};
use std::ffi::OsStr;
use std::mem;
use crate::Disk::*;
use crate::save_disk::*;
use crate::Inode::*;
use crate::memory_block::*;


#[path = "src/fsstructure/Inode.rs"] use Inode;
#[path = "src/fsstructure/Disk.rs"] use Disk;

//Nuestro fs tiene un disco
/*
Descripción: Define la estructura del FS, básicamente que disco va a usar.
Entradas: No tiene entradas.
Salidas: No tiene salidas.
*/
pub struct BWFS {
    pub(crate) disk : Disk
}
impl BWFS {
    /*
Descripción: Crea un nuevo FS basado en un disco en especifico.
Entradas: Ruta del punto de montaje, ruta del disco, ruta del disco a guardar
Salidas: A si mismo
*/
pub fn new(disk_path:String) -> Self{
        //Falta verificar si hay que agregar crear un nuevo disco o cargarlo, las funciones ya estan
        let new_disk = Disk::new(disk_path);
        BWFS {
            disk : new_disk
        }
}
    /*
Descripción: Carga un nuevo disco desde el binario obtenido de las imagenes.
Entradas: binario del disco, el filesystem vacío.
Salidas: el filesystem ingresado con los datos cargados
*/
pub fn load(disk:Disk,mut fs:BWFS) -> BWFS{
    fs.disk = disk;
    return fs
}


    /*
Descripción: Guarda el FileSystem en una imagen blanco y negro.
Entradas: El mismo
Salidas: No hay salidas.
*/
pub fn save_fs(&self){
        let encode_fs = encode(&self.disk);
        write_pixels(1000,1000,encode_fs,&self.disk.save_path, 0, 0)
    }

}

impl Drop for BWFS {
        /*
    Descripción: Apaga el FS guardandolo de manera persistente en imagenes.
    Entradas: El mismo
    Salidas: No hay salidas.
    */
    fn drop(&mut self) {
        let _ = &self.save_fs();
        println!("---BWFS--SAVED---!");
    }
}

impl Filesystem for BWFS {
    //Busca el inode asignado al ino y devuelve sus atributos
    /*
Descripción: Obtiene los atributos de un archivo existente en el FS.
Entradas: El mismo, el request, el inodo, el reply
Salidas: No hay salidas.
*/
fn getattr(&mut self,_req: &Request, inodo: u64, reply: ReplyAttr) {
        let inode = self.disk.get_inode(inodo);
        match inode {
            Some(inode) => {
                let ttl = time::now().to_timespec();
                println!("----BWFS--GETATTR----");

                reply.attr(&ttl, &inode.attributes);
            },
            None => reply.error(ENOENT)
        }
    }

    //Crea un archivo en la padre pasado por parametro
    /*
Descripción: Crea un archivo nuevo en el FS.
Entradas: El mismo, el request, el id del inodo de la carpeta padre, el nombre del archivo, el modo, banderas y el reply o respuesta
Salidas: No hay salidas.
*/
fn create(&mut self, _req: &Request, parent: u64, name: &OsStr, mode: u32, flags: u32, reply: ReplyCreate) {

        let ino_available = self.disk.get_next_available_inode();
        let memory_block = MemoryBlock {
            ino_ref : self.disk.get_next_available_inode(),
            data : Vec::new()
        };

        let ts = time::now().to_timespec();

        let attr = FileAttr {
            ino: ino_available.clone(),
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

        inode.references.push(memory_block.ino_ref as usize);

        self.disk.write_inode(inode);

        self.disk.add_reference(parent, ino_available.clone() as usize);
        self.disk.memory_block.push(memory_block.clone());
        println!("----BWFS--CREATED----");

        reply.created(&ts, &attr, 1, ino_available.clone(), flags.clone())
    }



    /*
Descripción: Abre un archivo existente en el FS.
Entradas: El mismo, el request, el id del inodo de la carpeta padre, el nombre del archivo, el modo, banderas y el reply o respuesta
Salidas: No hay salidas.
*/
fn open(&mut self, _req: &Request, _ino: u64, _flags: u32, reply: ReplyOpen) {
        let memory_block = self.disk.get_bytes_content(_ino);
        match memory_block {
            Some(memory_block) => {
                reply.opened(1, 0);
                print!("----BWFS--OPEN----\n");
            },
            None => reply.error(ENOENT)
        }
    }


    //Busca el bloque de memoria asignado al ino y muestra su contenido
    /*
Descripción: Lee un archivo existente en el FS.
Entradas: El mismo, el request, el id del inodo, el offset, el tamaño del buffer, el reply o respuesta
Salidas: No hay salidas.
*/
fn read(&mut self, _req: &Request, ino: u64, fh: u64, offset: i64, size: u32, reply: ReplyData) {
        let memory_block = self.disk.get_bytes_content(ino);
        match memory_block {
            Some(memory_block) => {reply.data(memory_block);
                println!("----BWFS--READ----");

            },
            None => {reply.error(EIO);}
        }
    }

    //Escribe dentro de un archivo en base al ino pasado
    /*
Descripción: Escribe sobre un archivo ya existente en el FS.
Entradas: El mismo, el request, el id del inodo, el offset, los datos, banderas y el reply o respuesta
Salidas: No hay salidas.
*/
fn write(&mut self, _req: &Request, ino: u64, _fh: u64, offset: i64, data: &[u8], _flags: u32, reply: ReplyWrite) {

        let inode = self.disk.get_mut_inode(ino);
        let content: Vec<u8> = data.to_vec();

        match inode {
            Some(inode) => {
                inode.attributes.size = data.len() as u64;
                self.disk.write_content(ino.clone(), content);
                println!("----BWFS--WRITE----");

                reply.written(data.len() as u32);
            },
            None => {
                reply.error(ENOENT);
            }
        }
    }


    //Funcion para cambiar de nombre un archivo mediante el padre
    /*
Descripción: Renombra el archivo existente en el FS. 
Entradas: El mismo, el request, el id del inodo de la carpeta padre, el nombre del archivo, el nuevo padre, el nuevo nombre y el reply o respuesta
Salidas: No hay salidas.
*/
fn rename(&mut self, _req:&Request, parent:u64, name:&OsStr, _newparent: u64, newname:&OsStr, reply:ReplyEmpty) {
        let name = name.to_str().unwrap();
        let inode :Option<&Inode> = self.disk.find_inode_in_references_by_name(parent, name);
        match inode {
            Some(inode) => {
                let ino = inode.attributes.ino;
                let child = self.disk.get_mut_inode(ino);
                match child {
                    Some(child) => {
                        println!("----BWFS--RENAME----");
                        child.name = newname.to_str().unwrap().to_string();
                        reply.ok()
                    },
                    None => {println!("-------RENAME ERROR-------------");}
                }
            },
            None =>{reply.error(ENOENT);}
        }
    }

    //Crea un directorio y asigna un nuevo ino
    /*
Descripción: Crea un directorio nuevo en el FS en el que se pueden guardar archivos.
Entradas: El mismo, el request, el id del inodo de la carpeta padre, el nombre del archivo, el modo y el reply o respuesta
Salidas: No hay salidas. 
*/
fn mkdir(&mut self, _req: &Request, parent: u64, name: &OsStr, _mode: u32, reply: ReplyEntry) {
        println!("----BWFS--MKDIR----");

        let ino = self.disk.get_next_available_inode();
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
            name,
            attributes: attr,
            references: Vec::new()
        };

        self.disk.write_inode(inode);
        self.disk.add_reference(parent,ino.clone() as usize);

        reply.entry(&ts, &attr, 0);
    }

    //Literalmente, lee un directorio
    /*
Descripción: Lee el directorio que se le pase como parámetro, este debe existir en el FS.
Entradas: El mismo, el request, el id del inodo, el fh, el offset y el reply o respuesta
Salidas: No hay salidas.
*/
fn readdir(&mut self, _req: &Request, ino: u64, fh: u64, offset: i64, mut reply: ReplyDirectory) {
        println!("----BWFS--READDIR----");

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

                reply.ok()
            },
            None => { println!("ERROR ino={:?}", ino.clone()); reply.error(ENOENT) }
        }
    }

    //Abre un directorio
    /*
Descripción: Abre un directorio existente en el FS.
Entradas: El mismo, el request, el id del inodo, banderas y el reply o respuesta
Salidas: No hay salidas.
*/
fn opendir(&mut self, _req: &Request, _ino: u64, _flags: u32, reply: ReplyOpen) {
        let dir = self.disk.get_inode(_ino);
        match dir {
            Some(dir) => {
                println!("----BWFS--OPENDIR----");
                reply.opened(dir.attributes.ino, 1 as u32);
            },
            None => {println!("-------CANT OPEN-------")}
        }

    }

    //Elimina un directorio en base al nombre
    /*
Descripción: Elimina o remueve un directorio existente en el FS.
Entradas: El mismo, el request, el id del inodo de la carpeta padre, el nombre del archivo y el reply o respuesta
Salidas: No hay salidas.
*/
fn rmdir(&mut self,_req: &Request, parent: u64, name: &OsStr, reply: ReplyEmpty) {
        println!("----BWFS--RMDIR----");

        let name = name.to_str().unwrap();
        let inode = self.disk.find_inode_in_references_by_name(parent, name);

        match inode {
            Some(inode) => {
                let ino = inode.attributes.ino;
                self.disk.clear_reference(parent.clone(), ino as usize);
                self.disk.remove_inode(ino);

                reply.ok();
            },
            None => reply.error(EIO)
        }
    }

    //Devuelve las estadistcas del filesystem *no funciona bien XD
    /*
Descripción: Muestra las estadísticas básicas del FS, como cantidad de inodos o bloques de memoria.
Entradas: El mismo, el request, el id del inodo, el reply o respuesta
Salidas: No hay salidas.
*/
fn statfs(&mut self, _req: &Request, _ino: u64, reply: ReplyStatfs) {
        println!("----BWFS--STATFS----");

        let blocks:u64 =  (self.disk.inodes_block.len() +self.disk.memory_block.len()) as u64;
        let bfree:u64 = blocks - self.disk.memory_block.len() as u64;
        let bavail:u64 = bfree;
        let files:u64 = self.disk.memory_block.len().try_into().unwrap();
        let ffree:u64 = 1024 as u64;
        let bsize:u32 = (mem::size_of::<Vec<Inode>>() as u32 +mem::size_of::<Inode>() as u32)*1024;
        let namelen:u32 = 77;
        let frsize:u32 = 1;

        reply.statfs(blocks.clone(),
                     bfree.clone(),
                     bavail,
                     files,
                     ffree,
                     bsize,
                     namelen,
                     frsize);
    }

    //Si datasync != 0, solo se deben vaciar los datos del usuario, no los metadatos.
    /*
Descripción: Sincroniza los contenidos de los archivos, si es diferente a 0 no borra los metadatos pero si los datos del usuario.
Entradas: El mismo, el request, el id del inodo, el fh, booleano sobre si la data esta sincronizada y el reply o respuesta
Salidas: No hay salidas. 
*/
fn fsync(&mut self, _req: &Request, ino: u64, fh: u64, datasync: bool, reply: ReplyEmpty) {
        println!("----BWFS--FSYNC----");
        reply.error(ENOSYS);
    }


    //Revisa el acceso de los permisos
    /*
Descripción: Revisa si puede acceder a un archivo ya existente en el FS.
Entradas: El mismo, el request, el id del inodo, una mascara y el reply o respuesta
Salidas: No hay salidas.
*/
fn access(&mut self, _req: &Request, _ino: u64, _mask: u32, reply: ReplyEmpty) {
        println!("----BWFS--ACCESS----");
        reply.ok();
    }

    //---------------------------------------------------------------------------------------
    // NO SE SI SIRVE
    //---------------------------------------------------------------------------------------
    /*
Descripción: Desvincula un archivo, ya sea vinculo normal o vínculo simbólico.
Entradas: El mismo, el request, el id del inodo padre, el nombre y el reply o respuesta
Salidas: No hay salidas. 
*/
fn unlink(&mut self, _req: &Request, _parent: u64, _name: &OsStr, reply: ReplyEmpty) {
        println!("----BWFS--UNLINK----");
        reply.error(ENOSYS);
    }

    /*
Descripción: Trata de eliminar o *flushear* los datos del caché.
Entradas: El mismo, el request, el id del inodo, el fh, el lock owner y el reply o respuesta
Salidas: No hay salidas.
*/
fn flush(&mut self, _req: &Request, _ino: u64, _fh: u64, _lock_owner: u64, reply: ReplyEmpty) {
        println!("----BWFS--FLUSH----\n");
        reply.error(ENOSYS);
    }


/*
    /*
Descripción: Encuentra el primer hueco de datos en un offset especifico.
Entradas: El mismo, el request, el id del inodo, el fh, el offset, el whence y el reply o respuesta
Salidas: No hay salidas.
*/
fn lseek(&mut self, _req: &Request, _ino: u64, _fh: u64, _offset: i64, _whence: u32, reply: ReplyEmpty) {
        println!("----BWFS--LSEEK----");
        reply.error(ENOSYS);
    }


*/
    //Revissa dentro de un directorio por su nombre y extrae los atributos
    /*
Descripción: /////////
Entradas: El mismo, el request, el id del inodo padre, el nombre y el reply o respuesta
Salidas: No hay salidas.
*/

    fn lookup(&mut self, _req: &Request, parent: u64, name: &OsStr, reply: ReplyEntry) {

        let fila_name = name.to_str().unwrap();
        let inode = self.disk.find_inode_in_references_by_name(parent, fila_name);
        match inode {
            Some(inode) => {
                let ttl = time::now().to_timespec();
                reply.entry(&ttl, &inode.attributes, 0);
                println!("----BWFS--LOOKUP----");
            },
            None => {
                println!("ERROR EN LOOKUP");
                reply.error(ENOENT);
            }
        }
    }
        
}
