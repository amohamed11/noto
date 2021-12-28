use std::fs::{File, metadata, create_dir};
use std::io::prelude::*;

pub fn create_file (filename: &str, template: String) -> std::io::Result<()> {
    let mut new_file = File::create(filename)?;
    new_file.write_all(template.as_bytes())?;

    new_file.sync_all()?;
    Ok(())
}

pub fn read_file(filename: &str) -> std::io::Result<String> {
    let mut file = File::open(filename)?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;

    Ok(buffer)
}

pub fn create_base_folder(dir: &str) -> std::io::Result<()> {
    let metadata = metadata(dir);
    // if we can't get metadata for the dir then it does not exist
    if metadata.is_err() {
        create_dir(dir).expect("Can't create the base folder for storing notes.");
    }

    Ok(())
}
