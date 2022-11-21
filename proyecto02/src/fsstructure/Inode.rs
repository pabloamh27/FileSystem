use fuse::{Filesystem, Request, ReplyCreate, ReplyEmpty, ReplyAttr, ReplyEntry, ReplyOpen, ReplyStatfs,ReplyData, ReplyDirectory, ReplyWrite, FileType, FileAttr};
use libc::{ENOSYS, ENOENT, EIO, EISDIR, ENOSPC};
use std::ffi::OsStr;
use std::mem;
use serde::{Serialize, Deserialize};
use image::Luma;
use crate::fileAttribute::FileAttrDef;


//Los Inodes son la unidad que movera nuestro fs

#[derive(Serialize, Deserialize)]
pub struct Inode {
    pub name: String,
    #[serde(with = "FileAttrDef")]
    pub attributes : FileAttr,
    pub references: Vec<usize>
}

impl Inode {
    //Agrega una referencia a si mismo
    pub fn add_reference(&mut self,ref_value: usize) {
        self.references.push(ref_value);
    }

    //Elimina una referencia a si mismo
    pub fn delete_reference(&mut self,ref_value: usize) {
        self.references.retain(|i| *i != ref_value);
    }

    //Cambia el nombre del Inode
    pub fn change_name(&mut self,value: String) {
        self.name = value;
    }
}