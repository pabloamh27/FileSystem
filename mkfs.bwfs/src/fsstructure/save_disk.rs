use image::GenericImageView;
use crate::Disk::*;

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
pub fn write_pixels(width: u32, height: u32,mut data: Vec<u8>, save_path: &str, file_counter: u32, mut data_position: usize) {
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
    let w = BufWriter::new(file);

    let encoder = png::Encoder::new(w, width, height);


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

