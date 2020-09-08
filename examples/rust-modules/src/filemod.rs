use uuid::Uuid;

pub fn get_uud() -> String {
    Uuid::new_v4().to_string()
}

// this is private function which cannot be accessed outside
fn cannot_access() {
    println!("I cannot be called!!");
}