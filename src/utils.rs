use std::fs::File;
use std::io::prelude::*;

pub fn create_file (filename: &str, template_path: &str) -> std::io::Result<()> {
    let template_file = File::open(template_path)?;
    let mut template_content: Vec<u8> = Vec::new();
    template_file.read_to_end(&mut template_content)?;

    let mut new_file = File::create(filename)?;
    new_file.write_all(template_content.as_slice())?;

    new_file.sync_all()?;
    Ok(())
}
