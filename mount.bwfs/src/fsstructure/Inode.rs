use fuse::FileAttr;
use serde::{Serialize, Deserialize};
use crate::fileAttribute::FileAttrDef;




/*
Descripción: Define los elementos que debe contener un I-node como lo son el nombre, sus atributos y referencias a bloques de memoria.
Entradas: No tiene entradas.
Salidas: No tiene salidas.
*/
#[derive(Serialize, Deserialize)]
pub struct Inode {
    pub name: String,
    #[serde(with = "FileAttrDef")]
    pub attributes : FileAttr,
    pub references: Vec<usize>
}

impl Inode {

    /*
Descripción: Agrega una referencia a un bloque de memoria a un I-node.
Entradas: El mismo I-node y la referencia a agregar.
Salidas: No tiene salidas.
*/
pub fn add_reference(&mut self,ref_value: usize) {
        self.references.push(ref_value);
    }


    /*
Descripción: Elimina una referencia a un bloque de memoria a un I-node.
Entradas: El mismo I-node y la referencia a eliminar.
Salidas: No tiene salidas.
*/
pub fn delete_reference(&mut self,ref_value: usize) {
        self.references.retain(|i| *i != ref_value);
    }


}