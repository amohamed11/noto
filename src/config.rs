use serde_derive::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct NotoConfig {
    pub template: String,
    pub base_folder: String,
    pub editor: String,
}

impl ::std::default::Default for NotoConfig {
    fn default() -> Self {
        Self {
            template: "default".to_string(),
            base_folder: "/tmp/Noto/".to_string(),
            editor: "vim".to_string(),
        }
    }
}

