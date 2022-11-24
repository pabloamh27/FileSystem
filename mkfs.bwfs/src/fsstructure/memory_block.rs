use serde::{Serialize, Deserialize};


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
Descripción:  Agrega datos a un bloque de memoria
Entradas: El mismo bloque de memoria y los datos a agregar.
Salidas: No tiene salidas.
*/
    pub fn add_data(&mut self,data: u8) {
        self.data.push(data);
    }

}