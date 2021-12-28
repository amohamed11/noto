use crate::utils;

pub fn new() {
    let result = utils::create_file("test.txt", "");
    if result.is_err() {
        panic!("Couldn't create a new note file.");
    }
    println!("Noto out!");
}
