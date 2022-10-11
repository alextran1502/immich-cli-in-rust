mod immich;

use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser, Debug)]
#[clap(name = "Immich CLI")]
#[clap(author = "Alex T. <alex.tran1502@gmail.com>")]
#[clap(about = "CLI Utilities for Immich", long_about = None)]
#[clap(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum FileFilter {
    ALL,
    VIDEO,
    IMAGE,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Upload assets to Immich server
    Upload {
        /// Email
        #[arg(short, long)]
        email: String,

        /// Password
        #[arg(short, long)]
        password: String,

        /// Path to assets
        #[arg(short, long, default_value = "./")]
        directory: String,

        /// Immich Server URL
        #[arg(short, long)]
        server: String,

        // File filter
        #[arg(value_enum, long, short, default_value = "all")]
        filter: FileFilter,
    },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Upload {
            email,
            password,
            directory,
            server,
            filter,
        } => immich::commands::upload(email, password, directory, server, filter).await,
    }
}

// Quick Test
// cargo run -- upload -e testuser@email.com -p password -d ~/Downloads -s http://10.1.15.216:2283/api
// docker run -it --rm -v $(pwd):/import test upload -e testuser@email.com -p password -d /import -s http://10.1.15.216:2283/api
