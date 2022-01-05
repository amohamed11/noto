use clap::{AppSettings, Parser, Subcommand};
use confy;

mod commands;
mod utils;
mod config;
mod consts;

#[derive(Parser)]
#[clap(name = consts::APP_NAME)]
#[clap(version = "0.1")]
#[clap(about = "A minimal note-taking cli tool for jogging down your thoughts quickly.")]
#[clap(global_setting(AppSettings::PropagateVersion))]
#[clap(global_setting(AppSettings::UseLongFormatForHelpSubcommand))]
#[clap(setting(AppSettings::SubcommandRequiredElseHelp))]
struct Cli {
    #[clap(subcommand)]
    command: Commands,

    /// Path for configuration file. Default is ~/.config/noto/config.
    #[clap(short, long)]
    config: Option<String>,
}

#[derive(Subcommand)]
enum Commands {
    /// create a new note with current timestamp for id
    New { name: Option<String> },

    /// Set the folder for storing your notes. Make sure the folder path is absolute.
    SetFolder { folder: String },

    /// Set editor to use for opening the new note.
    SetEditor { editor: String },
}


fn main() -> Result<(), std::io::Error> {
    let cli: Cli = Cli::parse();
    let cfg: config::NotoConfig = confy::load(consts::APP_NAME).expect("Failed to read config");

    match &cli.command {
        Commands::New { name } => {
            // warn user about selecting a proper base folder for Noto
            if cfg.base_folder == consts::DEFAULT_BASE_FOLDER {
                println!("No base folder selected. Noto will continue using /tmp/Noto/ for now. Change this using the `set-folder` command.")
            }

            // create base folder if it doesn't already exist
            utils::create_base_folder(&cfg.base_folder)?;

            commands::new(cfg, name).unwrap();
        },

        Commands::SetFolder { folder } => {
            let cfg = config::NotoConfig {
                base_folder: folder.to_string(),
                ..cfg
            };
            confy::store(consts::APP_NAME, &cfg).expect("Couldn't update configuration file");
            dbg!(&cfg);
        },

        Commands::SetEditor { editor } => {
            let cfg = config::NotoConfig {
                editor: editor.to_string(),
                ..cfg
            };
            confy::store(consts::APP_NAME, &cfg).expect("Couldn't update configuration file");
            dbg!(&cfg);
        }
    }

    Ok(())
}
