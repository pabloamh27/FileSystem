use serde::{Serialize, Deserialize};



/*
Descripción: Define la estructura de un bloque de memoria, que tiene una referencia a su I-node padre y los datos.
Entradas: No tiene entradas.
Salidas: No tiene salidas.
*/
#[derive(Serialize, Deserialize, Clone)]
pub struct MemoryBlock {
    pub ino_ref : u64,
    pub data : Vec<u8>
}

impl MemoryBlock {

    /*
Descripción:  Agrega datos a un bloque de memoria
Entradas: El mismo bloque de memoria y los datos a agregar.
Salidas: No tiene salidas.
    */
    pub fn add_data(&mut self,data: u8) {
        self.data.push(data);
    }
}