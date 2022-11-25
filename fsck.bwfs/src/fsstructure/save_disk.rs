use image::GenericImageView;
use crate::Disk::*;
use std::path::Path;
use std::fs::File;
use std::io::BufWriter;


#[path = "src/fsstructure/Disk.rs"] use Disk;

/*
Descripción: Codifica un disco y lo serializa a binario.
Entradas: El disco a codificar.
Salidas: Un vector de bits.
*/
pub fn encode_disk(object: &Disk) -> Vec<u8> {
    let enc = bincode::serialize(object).unwrap();
    return enc;
}

/*
Descripción: De-codifica un disco y lo de-serializa de binario.
Entradas: Un vector de bits. 
Salidas: El disco decodificado.
*/
pub fn decode_disk(object: Vec<u8>) -> Disk {
    let decoded: Disk = bincode::deserialize(&object[..]).unwrap();
    return decoded;
}


/*
Descripción: Se encarga de escribir los datos serializados en binario de un disco a una imagen de blanco y negro.
Entradas: El ancho, alto, datos, ruta de guardado, contador de bloques y la posición del bloque a escribir.
Salidas: No tiene salidas.
*/
pub fn write_pixels(width: u32, height: u32,mut data: Vec<u8>, save_path: &str, file_counter: u32, mut data_position: usize) {
    if data.len() < ((width * height) * (file_counter + 1)) as usize {
        while data.len() < ((width * height) * (file_counter + 1)) as usize {
            data.push(0);
        }
    }

    if data_position >= data.len() {
        return;
    }

    let final_path = format!("{}{}{}{}", save_path, "/file", file_counter, ".png");

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

/*
Descripción: Valida que la ruta de la para cargar el fs tenga imagenes.
Entradas: la direccion a verificar.
Salidas: Un booleano que indica si la ruta tiene contenido .
*/
pub fn validate_fs_path(path:String) -> bool{
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
    return match img {
        Ok(_img) => {
            true
        },
        Err(_img) => {
            false
        }
    }
}


/*
Descripción: Carga un disco usando la ruta de este.
Entradas: La ruta del disco a cargar.
Salidas: El disco cargado o un error.
*/
pub fn load_disk(path: String) -> Option<Disk> {
    let mut data: Vec<u8> = Vec::new();
    let mut file_counter = 0;
    let mut final_path = format!("{}{}{}{}", path, "/file", file_counter, ".png");
    while validate_path(final_path.clone()){
        file_counter += 1;
        let img = image::open(final_path.clone()).unwrap();
        for pixel in img.pixels() {
            let pixel = pixel.2;
            data.push(pixel[0]);
        }
        final_path = format!("{}{}{}{}", path, "/file", file_counter, ".png");
    }
    let disk_to_load = decode_disk(data);
    return Some(disk_to_load);

}