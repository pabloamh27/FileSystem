#[path= "src/filestructure/fileTimeSpec.rs"]
#[path= "src/filestructure/fileType.rs"]
use serde::{Serialize, Deserialize};
use fuse::{FileAttr, FileType};
use time::Timespec;

use crate::fileType::*;
use crate::fileTimeSpec::*;


#[derive(Serialize, Deserialize)]
#[serde(remote = "FileAttr")]
pub struct FileAttrDef {
    pub ino: u64,
    pub size: u64,
    pub blocks: u64,
    #[serde(with = "TimespecDef")]
    pub atime: Timespec,
    #[serde(with = "TimespecDef")]
    pub mtime: Timespec,
    #[serde(with = "TimespecDef")]
    pub ctime: Timespec,
    #[serde(with = "TimespecDef")]
    pub crtime: Timespec,
    #[serde(with = "FileTypeDef")]
    pub kind: FileType,
    pub perm: u16,
    pub nlink: u32,
    pub uid: u32,
    pub gid: u32,
    pub rdev: u32,
    pub flags: u32,
}