mod immich;

extern crate log;
extern crate simplelog;

use simplelog::*;

use std::process::exit;

use clap::{Parser, Subcommand};

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
            println!(
                "email: {}, password: {}, directory: {}, server: {}",
                email, password, directory, server
            );

            let mut config = openapi::apis::configuration::Configuration::new();
            config.base_path = server.to_string();

            simplelog::info!("Pinging server...");
            match openapi::apis::server_info_api::ping_server(&config).await {
                Ok(_) => {}
                Err(_) => {
                    simplelog::error!("[ERROR] Failed to connect to server - check URL");
                    exit(1);
                }
            }

            simplelog::info!("Logging in...");
            let auth_user = match immich::sign_in(&email, &password, &config).await {
                Ok(auth_user) => {
                    simplelog::info!("Logged in as {}", auth_user.user_email);
                    auth_user
                }
                Err(_) => {
                    simplelog::error!("Failed to sign in");
                    exit(1);
                }
            };

            // Set bearer token
            config.bearer_access_token = Some(auth_user.access_token);

            match openapi::apis::asset_api::get_all_assets(&config).await {
                Ok(assets) => {
                    println!("assets: {:?}", assets.len());
                }
                Err(error) => {
                    simplelog::error!("Failed to get assets {} ", error);
                    exit(1);
                }
            }
        }
    }
}

// Quick Test

// cargo run -- upload -e testuser@email.com -p password -d ~/Downloads -s http://10.1.15.216:2283/api
