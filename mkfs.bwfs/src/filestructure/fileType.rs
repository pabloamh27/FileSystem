use serde::{Serialize, Deserialize};
use fuse::{FileAttr, FileType};
use time::Timespec;

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