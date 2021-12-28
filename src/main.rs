mod commands;
mod utils;
mod config;
mod consts;

use clap::{AppSettings, Parser, Subcommand};
use confy;

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

    /// Set the folder for storing your notes
    #[clap(short, long)]
    folder: Option<String>,

    /// Path for configuration file. Default is ~/.config/noto/config.
    #[clap(short, long)]
    config: Option<String>,
}

#[derive(Subcommand)]
enum Commands {
    /// create a new note with current timestamp for id
    New { template_path: Option<String> },
}

fn main() -> Result<(), confy::ConfyError> {
    let cfg: config::NotoConfig = confy::load("noto")?;
    dbg!(&cfg);

    let cli = Cli::parse();

    match &cli.command {
        Commands::New { template_path } => {
            commands::new(&cfg.default_template.to_string(), template_path).unwrap();
        }
    }

    Ok(())
}
