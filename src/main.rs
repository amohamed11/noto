use clap::{AppSettings, Parser, Subcommand};
use confy;

mod commands;
mod utils;
mod config;
mod consts;

#[derive(Parser)]
#[clap(name = "Noto")]
#[clap(author = "Anas M. <mail@amohamed.ca>")]
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
    New { template_path: Option<String> },

    /// Set the folder for storing your notes. Make sure the folder path is absolute.
    SetFolder { folder: Option<String> },
}


fn main() -> Result<(), std::io::Error> {
    let cli: Cli = Cli::parse();
    let cfg: config::NotoConfig =  {
        if let Some(path) = cli.config {
            confy::load_path(path)
        } else {
            confy::load("noto")
        }
        .expect("Failed to read config")
    };

    match &cli.command {
        Commands::New { template_path } => {
            dbg!(&cfg);
            // warn user about selecting a proper base folder for Noto
            if cfg.base_folder == consts::DEFAULT_BASE_FOLDER {
                println!("No base folder selected. Noto will continue using /tmp/Noto/ for now, please select a base folder using the `folder` command.")
            }

            // create base folder if it doesn't already exist
            utils::create_base_folder(&cfg.base_folder)?;

            commands::new(cfg, template_path).unwrap();
        },

        Commands::SetFolder { folder } => {
            // update folder if user passed folder flag
            let mut new_folder: String = cfg.base_folder;
            if let Some(path) = folder {
                new_folder = path.to_string();
            };

            let cfg = config::NotoConfig {
                base_folder: new_folder,
                ..cfg
            };
            confy::store("noto", &cfg).expect("Couldn't update configuration file");
            dbg!(&cfg);
        }
    }

    Ok(())
}
