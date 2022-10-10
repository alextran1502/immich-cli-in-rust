use super::models::UploadAsset;
use colored::*;
use openapi::{
    apis::{authentication_api, configuration::Configuration},
    models::{LoginCredentialDto, LoginResponseDto},
};
use reqwest::{multipart, Body};
use std::{error::Error, path::Path, process::exit};
use tokio::fs::File;
use tokio_util::codec::{BytesCodec, FramedRead};

pub async fn ping_server(api_config: &Configuration) {
    println!("[1] Pinging server at {}", api_config.base_path.blue());
    match openapi::apis::server_info_api::ping_server(&api_config).await {
        Ok(_) => {
            println!("[{}] Established connection to server", "✓".green(),);
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
                "✓".green(),
                auth_user.user_email.blue(),
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
            println!("[{}] Found {} existing assets", "✓".green(), asset_id.len());
            asset_id
        }
        Err(_) => {
            println!("[{}] {}", "x".red(), "Failed to get device assets".red());
            exit(1)
        }
    }
}

pub async fn upload(api_config: &Configuration, assets: &Vec<UploadAsset>) {
    println!("[5] {}", "Uploading assets...".blink());

    for asset in assets {
        let file_path = Path::new(&asset.path);
        let file_name = file_path.file_name().unwrap().to_str().unwrap();
        let file_size = file_path.metadata().unwrap().len();

        println!(
            "[{}] Uploading {} ({} Bytes)",
            "⇢".blue(),
            file_name.blue(),
            file_size.to_string().blue()
        );

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
            .text("fileExtension", asset.file_extension.to_string())
            .text("duration", "0:00:00.000000")
            .part("assetData", file_data);

        let url = format!("{}/asset/upload", api_config.base_path);

        match reqwest::Client::new()
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
                    println!("[{}] Uploaded {}", "✓".green(), file_name.blue());
                } else {
                    println!(
                        "[{}] {} {} {}",
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
}
