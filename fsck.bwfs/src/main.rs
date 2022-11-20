use std::mem;
use sysinfo::{NetworkExt, NetworksExt, ProcessExt, System, SystemExt};

//Checamos cuanto espacio disponible y cuanto usado en general vemos el estado del disco
pub fn check_consistens(fs:&Rb_fs){

    let mut sys = System::new_all();
    sys.refresh_all();
    println!("=> System:");
    println!("total memory: {} KB", sys.total_memory());
    println!("used memory : {} KB", sys.used_memory());
    println!("RB-FS SPACE USED : {} KB", mem::size_of_val(fs.get_disk()));
    println!("RB-FS::MEMORY BLOCK SPACE USED : {} KB", mem::size_of_val(&fs.get_disk().super_block)*&fs.get_disk().super_block.len());
    println!("RB-FS::SUPER BLOCK SPACE USED : {} KB", mem::size_of_val(&fs.get_disk().memory_block)*&fs.get_disk().memory_block.len());


    println!("RB-FS SPACE AVAILABLE : {} KB", sys.total_memory()-sys.used_memory());

}
