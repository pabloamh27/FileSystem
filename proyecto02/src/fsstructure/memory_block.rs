use fuse::{Filesystem, Request, ReplyCreate, ReplyEmpty, ReplyAttr, ReplyEntry, ReplyOpen, ReplyStatfs,ReplyData, ReplyDirectory, ReplyWrite, FileType, FileAttr};
use libc::{ENOSYS, ENOENT, EIO, EISDIR, ENOSPC};
use std::ffi::OsStr;
use std::mem;
use serde::{Serialize, Deserialize};
use crate::fileAttribute::*;
use image::Luma;


//Se guarda el contenido de cada iNode creado
#[derive(Serialize, Deserialize, Clone)]
/*
Descripción: Define la estructura de un bloque de memoria, que tiene una referencia a su I-node padre y los datos.
Entradas: No tiene entradas.
Salidas: No tiene salidas.
*/
pub struct MemoryBlock {
    pub ino_ref : u64,
    pub data : Vec<u8>
    //Hacer que un bloque apunte a su siguiente inmediato, ya que ahora tienen tamano estatico
    //Agregar width y height (dimension) del memory block
}

impl MemoryBlock {
    //Agrega una referencia a si mismo
    /*
Descripción:  Agrega datos a un bloque de memoria, aquí se realiza la verificación de que este no pase de 1000*1000 de tamaño.
Entradas: El mismo bloque de memoria y los datos a agregar.
Salidas: No tiene salidas.
*/
pub fn add_data(&mut self,data: u8) {
        if self.data.len() <= (1000 * 1000) {
            self.data.push(data);
        } else {
            panic!("-----NO SE PUEDE INGRESAR DATA MAS GRANDE QUE 1000x1000-----");
        }
    }
    //Elimina una referencia a si mismo
    /*
Descripción: Elimina todos los datos de un bloque de memoria.
Entradas: El mismo bloque de memoria y los datos a eliminar.
Salidas: No tiene salidas.
*/
pub fn delete_data(&mut self,data: u8) {
        self.data.retain(|i| *i != data);
    }
}