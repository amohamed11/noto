use chrono::{DateTime, Local};

use crate::utils;
use crate::consts::ZETTELKASTEN;


pub fn new(default_template: &String, template_path: &Option<String>) -> std::io::Result<()> {
    let mut template: String = match default_template.as_str() {
        "zettelkasten" => ZETTELKASTEN.to_string(),
        _ => ZETTELKASTEN.to_string(),
    };

    //TODO handle using custom templates from files
    if template_path.is_some() {
        template = utils::read_file(template_path.as_ref().unwrap().as_str())?;
    }

    // generate datetime-stamp to use for filename
    let now: DateTime<Local> = Local::now();
    let note_name = now.format("%Y%m%d%H%M%S").to_string() + ".md";

    let result = utils::create_file(&note_name, template);
    if result.is_err() {
        panic!("ERROR: {:?}", result.err());
    }
    println!("Noto out!");

    Ok(())
}
