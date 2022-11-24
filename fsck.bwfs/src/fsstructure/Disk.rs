use fuse::{FileType, FileAttr};
use serde::{Serialize, Deserialize};
use crate::Inode::*;
use crate::memory_block::*;


//Creamos una estructura para guardar nuestros archivos Inodes
#[derive(Serialize, Deserialize)]
/*
Descripción: Estructura que define los atributos del disco en el que se va a guardar los datos del FS.
Entradas: No tiene entradas.
Salidas: No tiene salidas.
*/
pub struct Disk {
    pub inodes_block: Vec<Inode>,
    pub memory_block : Vec<MemoryBlock>,
    pub save_path: String
}
impl Disk {

/*Crea un nuevo disco y crea el inode raiz

Descripción: Crea un nuevo Disk, crea un nuevo superbloque de memory blocks, asigna los tiempos iniciales y le da atributos.
Entradas: La ruta del disco, la ruta donde se va a guardar el disco.
Salidas: El nuevo Disk.
*/

pub fn new(save_path:String) -> Disk{

    let memory_block = Vec::new();
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
    let new_disk = Disk { inodes_block: blocks, memory_block, save_path};
    println!("-----CREATING NEW DISK--------");
    return new_disk;
}


    /*
Descripción: Obtiene el siguiente inode que este disponible en el Disk.
Entradas: A si mismo, osea no tiene entradas.
Salidas: El siguiente inode disponible.
*/
pub fn get_next_available_inode(&mut self) -> u64 {
        return (self.inodes_block.len() + 1) as u64;
    }


    //Agrega el inode al super bloque
    /*
Descripción: Escribe el inode ingresado al superbloque de inodes.
Entradas: A si mismo, el inode que se va a escribir.
Salidas: No tiene salidas.
*/
pub fn write_inode(&mut self, inode:Inode) {
        self.inodes_block.push(inode);
    }


    //Elimina el inode disponible
    /*
Descripción: Elimina el inode ingresado del superbloque de inodes.
Entradas: A si mismo, el inode que se va a eliminar.
Salidas: No tiene salidas.
*/
pub fn remove_inode(&mut self, inode:u64) {
        self.inodes_block.retain(|i| i.attributes.ino != inode);
    }

    //Elimina una referencia de un respectivo inode
    /*
Descripción: Elimina la referencia a un bloque de memoria de un inode.
Entradas: A si mismo, el inode al que se le va a eliminar la referencia, el bloque de memoria que se va a eliminar.
Salidas: No tiene salidas.
*/
pub fn clear_reference(&mut self, ino: u64, ref_value: usize) {
        for i in 0..self.inodes_block.len() {
            if self.inodes_block[i].attributes.ino == ino {
                self.inodes_block[i.clone()].delete_reference(ref_value.clone());
            }
        }
    }

    //Agrega una respectiva referencia a un inode
    /*
Descripción: Agrega la referencia a un bloque de memoria de un inode.
Entradas: A si mismo, el inode al que se le va a agregar la referencia, el bloque de memoria que se va a agregar.
Salidas: No tiene salidas.
*/
pub fn add_reference(&mut self, ino: u64, ref_value: usize) {
        for i in 0..self.inodes_block.len() {
            if self.inodes_block[i].attributes.ino == ino {
                self.inodes_block[i.clone()].add_reference(ref_value.clone());
            }
        }
    }

    //Obtiene un Inode o nada
    /*
Descripción: Obtiene un inode por medio del Id, si no encuentra nada no devuelve nada.
Entradas: A si mismo, el Id del inode que se va a buscar.
Salidas: El inode que se buscaba o Nada.
*/
pub fn get_inode(&self, ino: u64) -> Option<&Inode> {
        for i in 0..self.inodes_block.len() {
            if self.inodes_block[i].attributes.ino == ino {
                return Some(&self.inodes_block[i.clone()]);
            }
        }
        return None;
    }

    //Obtiene un Inode mutable o nada
    /*
Descripción: Obtiene un inode mutable por medio del Id, si no encuentra nada no devuelve nada.
Entradas: A si mismo, el Id del inode que se va a buscar.
Salidas: El inode mutable que se buscaba o Nada.
*/
pub fn get_mut_inode(&mut self, ino: u64) -> Option<&mut Inode> {
        for i in 0..self.inodes_block.len() {
            if self.inodes_block[i].attributes.ino == ino {
                return Some(&mut self.inodes_block[i.clone()]);
            }
        }
        return None;
    }

    //Busca en base a la carpeta del padre el hijo que tenga el nombre por parametro
    /*
Descripción: Obtiene un Inode por medio del nombre, si no encuentra nada no devuelve nada.
Entradas: A si mismo, el inode del padre, el nombre del hijo que se va a buscar.
Salidas: El inode que se buscaba o Nada.
*/
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
    /*
Descripción: Agrega datos a un bloque de memoria asociado a un Inode buscado por medio del Id.
Entradas: A si mismo, el Id del inode al que se le va a agregar datos, los datos que se van a agregar.
Salidas: No tiene salidas.
*/
pub fn add_data_to_memory_block(&mut self, ino:u64,data:u8) {
        for i in 0..self.memory_block.len() {
            if self.memory_block[i].ino_ref == ino {
                self.memory_block[i.clone()].add_data(data.clone()) ;
            }
        }
    }

    //Escribe un arreglo de bites dentro de un inode
    /*
Descripción: Escribe los datos en el memory block asociado a un Inode por medio del Id.
Entradas: A si mismo, el Id del inode al que se le va a escribir datos, los datos que se van a escribir.
Salidas: No tiene salidas.
*/

pub fn write_content(&mut self, ino_ref: u64, content: Vec<u8>) {
        for i in 0..content.len(){
            self.add_data_to_memory_block(ino_ref.clone(), content[i].clone());

        }
    }

    //Obtiene el contenido de un arreglo
    /*
Descripción: Obtiene los datos en el memory block asociado a un inode por medio del Id.
Entradas: A si mismo, el Id del inode al que se le va a obtener datos.
Salidas: Los datos que se buscaban o Nada.
*/
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
