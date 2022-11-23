use fuse::{Filesystem, Request, ReplyCreate, ReplyEmpty, ReplyAttr, ReplyEntry, ReplyOpen, ReplyStatfs,ReplyData, ReplyDirectory, ReplyWrite, FileType, FileAttr};
use libc::{ENOSYS, ENOENT, EIO, EISDIR, ENOSPC};
use std::ffi::OsStr;
use std::mem;
use serde::{Serialize, Deserialize};
use image::Luma;
use crate::fileAttribute::FileAttrDef;


//Los Inodes son la unidad que movera nuestro fs

#[derive(Serialize, Deserialize)]
/*
Descripción: Define los elementos que debe contener un I-node como lo son el nombre, sus atributos y referencias a bloques de memoria.
Entradas: No tiene entradas.
Salidas: No tiene salidas.
*/
pub struct Inode {
    pub name: String,
    #[serde(with = "FileAttrDef")]
    pub attributes : FileAttr,
    pub references: Vec<usize>
}

impl Inode {
    //Agrega una referencia a si mismo
    /*
Descripción: Agrega una referencia a un bloque de memoria a un I-node.
Entradas: El mismo I-node y la referencia a agregar.
Salidas: No tiene salidas.
*/
pub fn add_reference(&mut self,ref_value: usize) {
        self.references.push(ref_value);
    }

    //Elimina una referencia a si mismo
    /*
Descripción: Elimina una referencia a un bloque de memoria a un I-node.
Entradas: El mismo I-node y la referencia a eliminar.
Salidas: No tiene salidas.
*/

pub fn delete_reference(&mut self,ref_value: usize) {
        self.references.retain(|i| *i != ref_value);
    }

    //Cambia el nombre del Inode
    /*
Descripción: Cambia el nombre de un I-node.
Entradas: El mismo I-node y el nuevo nombre.
Salidas: No tiene salidas.
*/
pub fn change_name(&mut self,value: String) {
        self.name = value;
    }
}