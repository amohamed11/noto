use std::process::Command;
use chrono::{DateTime, Local};

use crate::utils;
use crate::config::NotoConfig;
use crate::consts::DEFAULT_TEMPLATE;


pub fn new(cfg: NotoConfig, template_path: &Option<String>) -> std::io::Result<()> {
    let mut template: String = match cfg.template.as_str() {
        "default" => DEFAULT_TEMPLATE.to_string(),
        _ => DEFAULT_TEMPLATE.to_string(),
    };

    if template_path.is_some() {
        template = utils::read_file(template_path.as_ref().unwrap().as_str())?;
    }

    // generate datetime-stamp to use for filename
    let now: DateTime<Local> = Local::now();
    let note_path = format!("{}{}.md", cfg.base_folder.as_str(), now.format("%Y%m%d%H%M%S").to_string());

    let result = utils::create_file(&note_path, template);
    if result.is_err() {
        panic!("ERROR: {:?}", result.err());
    }

    Command::new(cfg.editor)
        .arg(&note_path)
        .status()
        .expect("Could not open the new note using the default editor");

    Ok(())
}
