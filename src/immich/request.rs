use super::models::UploadAsset;
use colored::*;
use openapi::{
    apis::{authentication_api, configuration::Configuration},
    models::{LoginCredentialDto, LoginResponseDto},
};
use reqwest::multipart;
use std::{error::Error, path::Path, process::exit};

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

pub async fn upload_asset(api_config: &Configuration, assets: &Vec<UploadAsset>) {
    println!("[6] {}", "Uploading assets...".blink());

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

        let form = reqwest::multipart::Form::new()
            .text("username", "seanmonstar")
            .text("password", "secret");

        match openapi::apis::asset_api::upload_file(api_config, file_path.to_path_buf()).await {
            Ok(_) => {
                println!("[{}] Uploaded {}", "✓".green(), file_name.blue());
            }
            Err(_) => {
                println!("[{}] {}", "x".red(), "Failed to upload asset".red());
            }
        }
    }
    // asset.iter().for_each(|asset| {
    //     println!("Uploading {}", asset.path.blue());
    //     let path_to_file = Path::new(&asset.path).to_path_buf();
    //     if let Ok(_) = openapi::apis::asset_api::upload_file(api_config, path_to_file).await {
    //         println!("[{}] {}", "✓".green(), "Uploaded".green());
    //     } else {
    //         println!("[{}] {}", "x".red(), "Failed to upload".red());
    //     }
    // });
}
