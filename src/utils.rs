use std::fs::File;
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
