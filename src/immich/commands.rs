use simplelog::*;

use crate::{immich, FileFilter};
use openapi::apis::configuration::Configuration;

pub async fn upload(
    email: &str,
    password: &str,
    directory: &str,
    server: &str,
    filter: &FileFilter,
) {
    let device_id = "CLI";
    let mut api_config = Configuration::new();
    api_config.base_path = server.to_string();

    immich::request::ping_server(&api_config).await;

    let auth_user = immich::request::login(&email, &password, &api_config)
        .await
        .unwrap();

    // All requests that need auth start here
    api_config.bearer_access_token = Some(auth_user.access_token);

    // Get files from database
    let existing_assets = immich::request::get_device_assets(&api_config, &device_id).await;

    // Get files
    let files = immich::directory_walker::dir_walk(directory, &filter);
}
