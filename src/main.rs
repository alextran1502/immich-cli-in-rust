mod immich;

extern crate simplelog;

use clap::{Parser, Subcommand};
use std::process::exit;

#[derive(Parser, Debug)]
#[clap(name = "Immich CLI")]
#[clap(author = "Alex T. <alex.tran1502@gmail.com>")]
#[clap(version = "1.0")]
#[clap(about = "CLI Utilities for Immich", long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Upload assets to Immich server
    Upload {
        /// Email
        #[clap(value_parser, short, long)]
        email: String,

        /// Password
        #[clap(value_parser, short, long)]
        password: String,

        /// Path to assets
        #[clap(value_parser, short, long, default_value = "./")]
        directory: String,

        /// Immich Server URL
        #[clap(value_parser, short, long)]
        server: String,
    },
}

#[tokio::main]
async fn main() {
    simplelog::TermLogger::init(
        simplelog::LevelFilter::Debug,
        simplelog::Config::default(),
        simplelog::TerminalMode::Mixed,
        simplelog::ColorChoice::Auto,
    )
    .expect("Failed to start simplelog");

    let cli = Cli::parse();

    match &cli.command {
        Commands::Upload {
            email,
            password,
            directory,
            server,
        } => {
            immich::commands::upload(email, password, directory, server).await;

            // Set bearer token
            // config.bearer_access_token = Some(auth_user.access_token);
            //
            // match openapi::apis::asset_api::get_all_assets(&config).await {
            //     Ok(assets) => {
            //         println!("assets: {:?}", assets.len());
            //     }
            //     Err(error) => {
            //         simplelog::error!("Failed to get assets {} ", error);
            //         exit(1);
            //     }
            // }
        }
    }
}

// Quick Test

// cargo run -- upload -e testuser@email.com -p password -d ~/Downloads -s http://10.1.15.216:2283/api
