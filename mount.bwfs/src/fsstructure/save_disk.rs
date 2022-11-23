use fuse::{Filesystem, Request, ReplyCreate, ReplyEmpty, ReplyAttr, ReplyEntry, ReplyOpen, ReplyStatfs,ReplyData, ReplyDirectory, ReplyWrite, FileType, FileAttr};
use libc::{ENOSYS, ENOENT, EIO, EISDIR, ENOSPC, clone};
use std::ffi::OsStr;
use std::io::Cursor;
use std::mem;
use serde::{Serialize, Deserialize};
use crate::fileAttribute;
use image::{Pixel, Luma, GenericImageView, GenericImage, DynamicImage, ImageBuffer, Rgb, Rgba, RgbaImage, RgbImage, GrayImage, GrayAlphaImage, ImageOutputFormat};
use crate::Disk::*;
use image::io::Reader as ImageReader;

use std::path::Path;
use std::fs::File;
use std::io::BufWriter;


#[path = "src/fsstructure/Disk.rs"] use Disk;


//Transforma el disco a bits

/*
Descripción: Codifica un disco y lo serializa a binario.
Entradas: El disco a codificar.
Salidas: Un vector de bits.
*/
pub fn encode(object: &Disk) -> Vec<u8> {
    let enc = bincode::serialize(object).unwrap();
    //println!("{:?}", enc);
    return enc;
}
//Decodifica un arreglo de bits y devuelve un Disk
/*
Descripción: De-codifica un disco y lo de-serializa de binario.
Entradas: Un vector de bits. 
Salidas: El disco decodificado.
*/
pub fn decode(object: Vec<u8>) -> Disk {
    let decoded: Disk = bincode::deserialize(&object[..]).unwrap();
    return decoded;
}


// =================================================================================================
// https://docs.rs/image/latest/image/
// https://blog.logrocket.com/decoding-encoding-images-rust-using-image-crate/?ref=morioh.com&utm_source=morioh.com
// https://morioh.com/p/a3e5136ef8db
// =================================================================================================
//Escribe pixeles en una imagen
/*
Descripción: Se encarga de escribir los datos serializados en binario de un disco a una imagen de blanco y negro.
Entradas: El ancho, alto, datos, ruta de guardado, contador de bloques y la posición del bloque a escribir.
Salidas: No tiene salidas.
*/
pub fn write_pixels(width: u32, height: u32,mut data: Vec<u8>, mut save_path: &str, file_counter: u32, mut data_position: usize) {
    if data.len() < ((width * height) * (file_counter + 1)) as usize {
        //rellena con ceros hasta el tamaño de la imagen
        while data.len() < ((width * height) * (file_counter + 1)) as usize {
            data.push(0);
        }
    }

    if data_position >= data.len() {
        return;
    }

    let final_path = format!("{}{}{}{}", save_path, "/file", file_counter, ".png");

    println!("final_path: {}", final_path);

    let path = Path::new(final_path.as_str());
    let file = File::create(path).unwrap();
    let mut w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, width, height);


    let mut counter = 0;

    let mut pixels_colors = Vec::new();

    for i in data_position..data.len() + 1 {
        if counter == (width * height) as usize || i == data.len() {
            data_position = i;
            break;
        }
        else {
            pixels_colors.push(data[i]);
            counter = counter + 1;
        }
    }
    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(&pixels_colors).unwrap();
    if data_position >= data.len() {
        return;
    }
    write_pixels(width, height, data.clone(), save_path, file_counter + 1, data_position);
}

/*
Descripción: Valida que la ruta de la para cargar el fs tenga imagenes.
Entradas: la direccion a verificar.
Salidas: Un booleano que indica si la ruta tiene contenido .
*/
pub fn validate_fs_path(mut path:String) -> bool{
    let mut counter = 0;
    let mut final_path = String::new();
    loop {
        final_path = format!("{}{}{}{}", path, "/file", counter, ".png");
        if Path::new(final_path.as_str()).exists() {
            counter = counter + 1;
        }
        else {
            break;
        }
    }
    if counter == 0 {
        return false;
    }
    return true;
}



/*
Descripción: Valida que la ruta de la imagen a escribir exista.
Entradas: La ruta de la imagen a escribir.
Salidas: Un booleano que indica si la ruta existe o no.
*/
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

/*
HAY QUE HACER LA VERSION DE ESTA FUNCION QUE RECIBA UNA IMAGEN Y LA GUARDE EN EL DISCO
 */
/*
Descripción: Carga un sistema de archivos usando la ruta de este.
Entradas: La ruta del sistema de archivos a cargar.
Salidas: El sistema de archivos cargado o un error.
*/
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

/*
Descripción: Carga un disco usando la ruta de este.
Entradas: La ruta del disco a cargar.
Salidas: El disco cargado o un error.
*/
pub fn load_disk(path: String) -> Option<Disk> {
    //por cada imagen en la ruta
    let mut data: Vec<u8> = Vec::new();
    let mut file_counter = 0;
    let mut final_path = format!("{}{}{}{}", path, "/file", file_counter, ".png");
    while validate_path(final_path.clone()){
        println!("hola");
        println!("Contador: {}", file_counter);
        file_counter += 1;
        let img = image::open(final_path.clone()).unwrap();
        for pixel in img.pixels() {
            let pixel = pixel.2;
            data.push(pixel[0]);
        }
        final_path = format!("{}{}{}{}", path, "/file", file_counter, ".png");
    }
    println!("data: {:?}", data.len());
    let disk_to_load = decode(data);
    //Aca se carga el disc al fs
    println!("----BWFS--DISCO CARGADO---------");
    return Some(disk_to_load);

}