use crate::asset::Asset;

pub fn action() {
    println!(
        "{}",
        String::from_utf8_lossy(&Asset::get("alias.sh").unwrap().data)
    );
}
