use std::path::Path;

use crate::{
    immich::{self, models::DeviceAsset},
    FileFilter,
};
use itertools::Itertools;
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

    // Get deviceAssetId from database
    let asset_on_database = immich::request::get_device_assets(&api_config, &device_id).await;

    // Get files
    let asset_on_device = immich::fs::dir_walk(directory, &filter)
        .iter()
        .map(|file| {
            let path = Path::new(&file);
            let file_name = path.file_name().unwrap().to_str().unwrap();
            let file_size = path.metadata().unwrap().len();
            let fild_id = format!("{}-{}", file_name, file_size);

            DeviceAsset {
                id: fild_id,
                path: file.to_string(),
            }
        })
        .collect_vec();

    // Get files that are not on device
    let files_to_upload = asset_on_device
        .iter()
        .filter(|file| {
            asset_on_database
                .iter()
                .find(|asset_id| asset_id.to_string() == file.id)
                .is_none()
        })
        .collect_vec();

    let files_with_metadata = immich::fs::get_file_metadata(&files_to_upload);
    println!("{:?}", files_with_metadata.len());

    immich::request::upload_asset(&api_config, &files_with_metadata).await;
}
