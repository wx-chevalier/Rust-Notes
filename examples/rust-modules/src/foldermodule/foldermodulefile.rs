use uuid::Uuid;

// now let's call other local mod
use crate::modtomod::something;

pub fn folder_module_uuid() -> String {
    Uuid::new_v4().to_string()
}

pub fn mod_to_mod_function() {
    println!("hey!! from other mod {}", something::mod_to_mod());
}
