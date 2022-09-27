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

    // immich::ping_server::ping_server(&api_config).await;

    // let auth_user = immich::login::login(&email, &password, &api_config)
    //     .await
    //     .unwrap();

    // // All requests that need auth start here
    // api_config.bearer_access_token = Some(auth_user.access_token);

    // Get files
    let files = immich::directory_walker::dir_walk(directory).await;
}
