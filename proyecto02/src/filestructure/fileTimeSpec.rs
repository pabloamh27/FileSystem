use serde::{Serialize, Deserialize};
use time::Timespec;


// Al serializarf no es posibrle con algunas opciones de fuse
//Entonces las definimos

#[derive(Serialize, Deserialize)]
#[serde(remote = "Timespec")]
/*
Descripción: Define los atributos de tiempo que va a tener un archivo o directorio.
Entradas: No tiene entradas.
Salidas: No tiene salidas.
*/
pub(crate) struct TimespecDef {
    pub sec: i64,
    pub nsec: i32,
}