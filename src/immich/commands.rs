use simplelog::*;
use std::{error::Error, process::exit};

use crate::immich;
use openapi::{
    apis::{authentication_api, configuration::Configuration},
    models::{LoginCredentialDto, LoginResponseDto},
};

pub async fn upload(email: &str, password: &str, directory: &str, server: &str) {
    let mut api_config = Configuration::new();
    api_config.base_path = server.to_string();

    immich::ping_server::ping_server(&api_config).await;

    simplelog::info!("Logging in...");
    let auth_user = immich::login::login(&email, &password, &api_config)
        .await
        .unwrap();
}
