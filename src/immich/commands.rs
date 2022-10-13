use std::{collections::HashMap, path::Path};

use crate::{
    immich::{self, models::DeviceAsset},
    FileFilter,
};
use colored::*;
use itertools::Itertools;
use openapi::apis::configuration::Configuration;

use super::models::FolderName;

pub async fn upload(
    email: &str,
    password: &str,
    directory: &str,
    server: &str,
    filter: &FileFilter,
    album: &bool,
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

    // Get assets from each sub-folder and group them together by folder name
    let asset_on_device = immich::fs::dir_walk(directory, &filter);

    // Get assets that are not on the database
    let asset_to_upload = remove_uploaded_asset(asset_on_database, asset_on_device);

    println!("{:?}", asset_to_upload);
    // flat hashmap
    let files_to_upload = asset_to_upload.values().flatten().collect_vec();

    // println!(
    //     "[{}] Found {} new files to upload",
    //     "✓".green(),
    //     files_to_upload.len().to_string().green()
    // );
    let files_with_metadata = immich::fs::get_file_metadata(&files_to_upload);

    immich::request::upload(&api_config, &files_with_metadata).await;
}

fn remove_uploaded_asset(
    asset_on_database: Vec<String>,
    asset_on_device: HashMap<FolderName, Vec<DeviceAsset>>,
) -> HashMap<FolderName, Vec<DeviceAsset>> {
    let mut asset_to_upload: HashMap<FolderName, Vec<DeviceAsset>> = HashMap::new();

    for (folder_name, device_assets) in asset_on_device {
        let mut valid_files: Vec<DeviceAsset> = Vec::new();
        for device_asset in device_assets {
            if asset_on_database
                .iter()
                .find(|asset_id| asset_id.to_string() == device_asset.id)
                .is_none()
            {
                valid_files.push(device_asset);
            }
        }

        asset_to_upload.insert(folder_name, valid_files);
    }

    asset_to_upload
}
