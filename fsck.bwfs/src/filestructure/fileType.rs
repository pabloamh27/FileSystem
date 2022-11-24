use serde::{Serialize, Deserialize};
use fuse::FileType;


#[derive(Serialize, Deserialize)]
#[serde(remote = "FileType")]
/*
Descripción: Define los tipos que va a tener un archivo o directorio.
Entradas: No tiene entradas.
Salidas: No tiene salidas.
*/
pub enum FileTypeDef {
    NamedPipe,
    CharDevice,
    BlockDevice,
    Directory,
    RegularFile,
    Symlink,
    Socket,
}