use fuse::{Filesystem, Request, ReplyCreate, ReplyEmpty, ReplyAttr, ReplyEntry, ReplyOpen, ReplyStatfs,ReplyData, ReplyDirectory, ReplyWrite, FileType, FileAttr};
use libc::{ENOSYS, ENOENT, EIO, EISDIR, ENOSPC};
use std::ffi::OsStr;
use std::mem;
use serde::{Serialize, Deserialize};
use crate::fileAttribute;
use image::Luma;
use crate::Disk::*;


#[path = "src/fsstructure/Disk.rs"] use Disk;


//Transforma el disco a bits

pub fn encode(object: &Disk) -> Vec<u8> {
    let enc = bincode::serialize(object).unwrap();
    //println!("{:?}", enc);
    return enc;
}
//Decodifica un arreglo de bits y devuelve un Disk
pub fn decode(object: Vec<u8>) -> Disk {
    let decoded: Disk = bincode::deserialize(&object[..]).unwrap();
    return decoded;
}

pub fn validate_path(path:String) -> bool{
    let img = image::open(path);
    match img {
        Ok(img) => {
            return true;
        },
        Err(img) => {
            return false;
        }
    }
}


pub fn load_fs(path : String) -> Option<Disk>{
    // Carga la base pasada por parametro
    let img = image::open(path).unwrap();
    let img_gray = img.into_luma(); //La pasa a grises

    //Crea el decodificador
    let mut decoder = quircs::Quirc::default();

    // Busca todos los codigos qr
    let codes = decoder.identify(img_gray.width() as usize, img_gray.height() as usize, &img_gray);
    let mut vec_decode: Option<Vec<u8>> = None;
    for code in codes {
        let code = code.expect("----RB-FS ERROR AL EXTRAER QR-----------");
        let decoded = code.decode().expect("----RB-FS ERROR AL DECODIFICAR-------");
        vec_decode = Some(decoded.payload);
    }
    match vec_decode {
        Some(vec_decode) => {
            let disk_to_load:Disk = decode(vec_decode);
            //Aca se carga el disc al fs
            println!("----RB-FS DISCO CARGADO---------");
            return Some(disk_to_load);
        },
        None => {
            println!("------- ERROR AL CARGAR EL DISCO --------");
            return None;
        }
    }
}