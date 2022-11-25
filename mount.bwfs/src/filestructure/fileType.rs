use serde::{Serialize, Deserialize};
use fuse::FileType;


/*
Descripción: Define los tipos que va a tener un archivo o directorio.
Entradas: No tiene entradas.
Salidas: No tiene salidas.
*/
#[derive(Serialize, Deserialize)]
#[serde(remote = "FileType")]
pub enum FileTypeDef {
    NamedPipe,
    CharDevice,
    BlockDevice,
    Directory,
    RegularFile,
    Symlink,
    Socket,
}