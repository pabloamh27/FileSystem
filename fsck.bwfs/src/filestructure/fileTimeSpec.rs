use serde::{Serialize, Deserialize};
use time::Timespec;




/*
Descripción: Define los atributos de tiempo que va a tener un archivo o directorio.
Entradas: No tiene entradas.
Salidas: No tiene salidas.
*/
#[derive(Serialize, Deserialize)]
#[serde(remote = "Timespec")]
pub(crate) struct TimespecDef {
    pub sec: i64,
    pub nsec: i32,
}