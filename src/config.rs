#[derive(Debug, Serialize, Deserialize)]
struct NotoConfig {
    default_template: String,
    base_folder: String,
}

impl ::std::default::Default for NotoConfig {
    fn default() -> Self { Self {
        default_template: "~/Noto/Templates/zettalkasten.md",
        base_folder: "~/Noto/",
    } }
}
