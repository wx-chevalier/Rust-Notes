// this is external crate, which we installed via cargo.toml
use uuid::Uuid;

// now, we have created file called 'filemod'
// which is local mod goes by filename
mod filemod;
use filemod::get_uud;

// now , we have created folder called 'foldermodule'
// which is local mod goes by module called 'foldermodule'
mod foldermodule;
// you use following syntax for importing multiple functions from same mod
use foldermodule::foldermodulefile::{folder_module_uuid, mod_to_mod_function};

// till now we have seen using local modules in main file
// now let's see how to do mod to mod rather mod to main
// to use mod in other mod first we need to define called mod in
// main, so that caller mod can have it's scope
mod modtomod;

fn get_uuid() -> String {
    Uuid::new_v4().to_string()
}

fn main() {
    let uuid = get_uuid();
    println!("the uuid is {}", uuid);

    // local module which goes by filename
    let file_mod_uuid = get_uud();
    println!("local module uuid is {}", file_mod_uuid);

    // local folder module
    let folder_mod_uuid = folder_module_uuid();
    println!("folder module uuid is {}", folder_mod_uuid);

    // mod to mod example
    mod_to_mod_function();
}
