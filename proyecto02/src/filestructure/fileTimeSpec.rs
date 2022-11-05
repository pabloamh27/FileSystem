use serde::{Serialize, Deserialize};
use fuse::{FileAttr, FileType};
use time::Timespec;


// Al serializarf no es posibrle con algunas opciones de fuse
//Entonces las definimos

#[derive(Serialize, Deserialize)]
#[serde(remote = "Timespec")]
pub struct TimespecDef {
    pub sec: i64,
    pub nsec: i32,
}