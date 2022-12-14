use super::models::UploadAsset;
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use openapi::{
    apis::{authentication_api, configuration::Configuration},
    models::{AlbumResponseDto, LoginCredentialDto, LoginResponseDto},
};
use reqwest::{multipart, Body};
use std::{error::Error, path::Path, process::exit};
use tokio::fs::File;
use tokio_util::codec::{BytesCodec, FramedRead};

pub async fn ping_server(api_config: &Configuration) {
    println!(
        "[1] Pinging server at {}",
        api_config.base_path.blue().bold()
    );
    match openapi::apis::server_info_api::ping_server(&api_config).await {
        Ok(_) => {
            println!("[{}] Established connection to server", "✓".green().bold(),);
        }
        Err(_) => {
            println!(
                "[{}] {}",
                "x".red(),
                "Failed to establish connection to server".red()
            );
            std::process::exit(1);
        }
    }
}

pub async fn login(
    email: &str,
    password: &str,
    api_config: &Configuration,
) -> Result<LoginResponseDto, Box<dyn Error>> {
    println!("[2] {}", "Logging in...".blink());

    let login_payload = LoginCredentialDto {
        email: email.to_string(),
        password: password.to_string(),
    };

    let auth_user = match authentication_api::login(&api_config, login_payload).await {
        Ok(auth_user) => {
            println!(
                "[{}] Logged in as {}",
                "✓".green().bold(),
                auth_user.user_email.blue().bold(),
            );
            auth_user
        }
        Err(_) => {
            println!("[{}] {}", "x".red(), "Failed to sign in".red());
            return Err("Failed to sign in".into());
        }
    };

    Ok(auth_user)
}

pub async fn get_device_assets(api_config: &Configuration, device_id: &str) -> Vec<String> {
    println!("[3] {}", "Getting device assets...".blink());

    match openapi::apis::asset_api::get_user_assets_by_device_id(api_config, device_id).await {
        Ok(asset_id) => {
            println!(
                "[{}] Found {} assets that are uploaded from the CLI",
                "✓".green().bold(),
                asset_id.len().to_string().blue().bold()
            );
            asset_id
        }
        Err(_) => {
            println!("[{}] {}", "x".red(), "Failed to get device assets".red());
            exit(1)
        }
    }
}

pub async fn upload(api_config: &Configuration, assets: &Vec<UploadAsset>) {
    let client = reqwest::Client::new();

    let pb = ProgressBar::new(assets.len() as u64);
    let progress_bar_style = ProgressStyle::with_template(
        "[5] {spinner:.blue} Uploading {bar:40.blue.bold/blue.bold} [{pos:>7}/{len:7}] {msg}",
    )
    .unwrap()
    .progress_chars("##-");
    pb.set_style(progress_bar_style);

    for asset in assets {
        let file_path = Path::new(&asset.path);
        let file_name = file_path.file_name().unwrap().to_str().unwrap();
        let file_size = file_path.metadata().unwrap().len();

        pb.set_message(format!(
            "[{}] Uploading {} ({} Bytes)",
            "▶".blue().bold(),
            file_name.blue().bold(),
            indicatif::HumanBytes(file_size).to_string().blue().bold()
        ));

        let file = File::open(asset.path.to_string()).await.unwrap();

        // read file body stream
        let stream = FramedRead::new(file, BytesCodec::new());
        let file_body = Body::wrap_stream(stream);
        let file_data = multipart::Part::stream(file_body)
            .file_name(file_name.to_string())
            .mime_str(asset.mime_type.to_string().as_str())
            .unwrap();

        let form = reqwest::multipart::Form::new()
            .text("deviceAssetId", asset.id.to_string())
            .text("deviceId", "CLI")
            .text("assetType", asset.asset_type.to_string())
            .text("createdAt", asset.created_at.to_string())
            .text("modifiedAt", asset.modified_at.to_string())
            .text("isFavorite", "false")
            .text(
                "fileExtension",
                format!(".{}", asset.file_extension.to_string()),
            )
            .text("duration", "0:00:00.000000")
            .part("assetData", file_data);

        let url = format!("{}/asset/upload", api_config.base_path);

        match client
            .post(url)
            .multipart(form)
            .header(
                "Authorization",
                format!(
                    "Bearer {}",
                    api_config.bearer_access_token.as_ref().unwrap()
                ),
            )
            .send()
            .await
        {
            Ok(res) => {
                if res.status().is_success() {
                    pb.inc(1);
                    // println!("[{}] Uploaded {}", "✓".green().bold(), file_name.blue().bold());
                } else {
                    pb.inc(1);
                    println!(
                        "[{}] {} {} | Error Message: {}",
                        "x".red(),
                        "Failed to upload asset at".red(),
                        file_path.to_str().unwrap().yellow(),
                        res.text().await.unwrap()
                    );
                }
            }
            Err(_) => {
                println!("[{}] {}", "x".red(), "Failed to upload asset".red());
                exit(1)
            }
        }
    }

    pb.finish_with_message(format!("[{}] Finished", "✓".green().bold()))
}

pub async fn get_albums(api_config: &Configuration) -> Vec<AlbumResponseDto> {
    let albums = openapi::apis::album_api::get_all_albums(&api_config, Some(false), None).await;

    if let Ok(albums) = { albums } {
        println!("[5] {}", "Getting existing albums...".blink());
        println!(
            "[{}] Found {} albums",
            "✓".green().bold(),
            albums.len().to_string().blue().bold()
        );

        for album in &albums {
            println!("[{}] {}", "▶".blue().bold(), album.album_name.blue().bold());
        }

        return albums;
    } else {
        println!("[{}] {}", "x".red(), "Failed to get albums".red());
        exit(1)
    }
}
