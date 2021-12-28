use clap::{AppSettings, Parser, Subcommand};

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
}

#[derive(Subcommand)]
enum Commands {
    /// create a new note with current timestamp for id
    New,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::New => {
            println!("Noto out!");
        }
    }
}
