use std::io::{self, Write};
use std::path::Path;
use std::process::Command;
use chrono::Local;

use crate::utils;
use crate::config::NotoConfig;
use crate::consts::DEFAULT_TEMPLATE;


pub fn new(cfg: NotoConfig, name: &Option<String>) -> std::io::Result<()> {
    let template: String = match cfg.template.as_str() {
        "default" => DEFAULT_TEMPLATE.to_string(),
        _ => DEFAULT_TEMPLATE.to_string(),
    };

    // if no name was provided, generate datetime-stamp to use for filename
    let filename: String = {
        if name.is_some() {
            name.as_ref().unwrap().to_string()
        } else {
            Local::now().format("%Y%m%d%H%M%S").to_string()
        }
    };
    let note_path = format!("{}{}.md", cfg.base_folder.as_str(), filename);
    
    // check if path with name already exists, if so warn the user
    if Path::new(&note_path).exists() {
        print!("A file named {} already exists. Do you want to overwrite it? [y/n]: ", filename);
        io::stdout().flush()?;

        let mut buffer = String::new();
        let stdin = io::stdin();

        stdin.read_line(&mut buffer)?;

        match buffer.trim() {
            "y" | "Y" | "yes" | "Yes" => utils::create_file(&note_path, template).expect("Error creating note: "),
            _ => ()
        }
    } else {
        utils::create_file(&note_path, template).expect("Error creating note: ");
    }


    Command::new(cfg.editor)
        .arg(&note_path)
        .status()
        .expect("Could not open the new note using the default editor");

    Ok(())
}
