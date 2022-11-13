use fuse::{Filesystem, Request, ReplyCreate, ReplyEmpty, ReplyAttr, ReplyEntry, ReplyOpen, ReplyStatfs,ReplyData, ReplyDirectory, ReplyWrite, FileType, FileAttr};
use libc::{ENOSYS, ENOENT, EIO, EISDIR, ENOSPC};
use std::ffi::OsStr;
use std::io::Cursor;
use std::mem;
use serde::{Serialize, Deserialize};
use crate::fileAttribute;
use image::{Pixel, Luma, GenericImageView, GenericImage, DynamicImage, ImageBuffer, Rgb, Rgba, RgbaImage, RgbImage, GrayImage, GrayAlphaImage, ImageOutputFormat};
use crate::Disk::*;
use image::io::Reader as ImageReader;
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

// =================================================================================================
// No se descarta, pueden salir ideas de acá
// =================================================================================================
/*
//Escribe pixeles en una imagen
pub fn write_pixels(height: u32, width: u32 , data: Vec<u8>) {
    let mut imgbuf = image::ImageBuffer::new(width as usize as u32, height as u32);
    let mut i = 0;
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        if data[i] == 0 {
            *pixel = image::Luma([0]);
            i = i + 1;
        } else {
            *pixel = image::Luma([255]);
            i = i + 1;
        }
    }
    imgbuf.save("test.png").unwrap();
}
*/



// =================================================================================================
// falta, crear la imagen de una vez con el tamaño correcto *
// probar en el main ***
// En teoría funciona el for y el if, ya más de 2 fuentes lo hacen parecido
// https://docs.rs/image/latest/image/
// https://blog.logrocket.com/decoding-encoding-images-rust-using-image-crate/?ref=morioh.com&utm_source=morioh.com
// https://morioh.com/p/a3e5136ef8db
// =================================================================================================
//Escribe pixeles en una imagen
pub fn write_pixels(height: u32, width: u32, data: Vec<u8>) {

    let mut img = image::open("image.png").unwrap();
    //let (width, height) = img.dimensions();

    let mut output = ImageBuffer::new(width, height);
    let (width, height) = output.dimensions();

    let mut i = 0;
    for (x, y, pixel) in img.pixels() {
        if data[i] == 0 {
            output.put_pixel(x, y,
                             // 0 is black, 255 is white
                             pixel.map(|p| p.saturating_sub(0))
            );
            i = i + 1;
        } else {
            output.put_pixel(x, y,
                                // 0 is black, 255 is white
                             pixel.map(|p| p.saturating_sub(255))
            );
            i = i + 1;
        }

    }

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