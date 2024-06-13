use clap::{Parser, Subcommand};

mod commands;
mod project;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Init,
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Init => {
            let project = commands::init();
            println!("{project:#?}")
        }
    }
}
