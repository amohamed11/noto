use dirs::home_dir;
use serde_derive::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct NotoConfig {
    pub template: String,
    pub base_folder: String,
    pub editor: String,
}

impl ::std::default::Default for NotoConfig {
    fn default() -> Self {
        // use dirs to find the user's home directory.
        // if no home dir is found, default to linux tmp dir
        let folder = {
            if let Some(home) = home_dir() {
                home.to_str().unwrap().to_owned() + "/Noto/"
            } else {
                "/tmp/Noto/".to_string()
            }
        };

        Self {
            template: "default".to_string(),
            base_folder: folder,
            editor: "vim".to_string(),
        }
    }
}

