use colored::*;
use openapi::{
    apis::{authentication_api, configuration::Configuration},
    models::{LoginCredentialDto, LoginResponseDto},
};
use std::error::Error;

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

pub async fn get_device_assets(device_id: &str) {
    println!("[3] {}", "Getting device assets...".blink());
}
