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
pub mod modtomod;
