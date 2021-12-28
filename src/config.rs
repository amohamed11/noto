use serde_derive::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct NotoConfig {
    pub default_template: String,
    pub base_folder: String,
}

impl ::std::default::Default for NotoConfig {
    fn default() -> Self {
        Self {
            default_template: "zettalkasten".to_string(),
            base_folder: "/tmp/Noto/".to_string(),
        }
    }
}

