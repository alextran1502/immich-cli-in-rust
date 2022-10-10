use chrono::prelude::{DateTime, Utc};
use itertools::Itertools;
use std::{ffi::OsStr, path::Path, time::SystemTime};
use walkdir::WalkDir;

use crate::{
    immich::models::{SupportFileType, UploadAsset},
    FileFilter,
};
use colored::*;

use super::models::DeviceAsset;

pub fn dir_walk(path: &str, filter: &FileFilter) -> Vec<String> {
    println!("[4] {} {}", "Indexing directory", path.blue());
    let valid_paths: Vec<String> = WalkDir::new(path)
        .into_iter()
        .filter_map(|e| match e.ok().filter(|e| e.file_type().is_file()) {
            Some(e) => match e.path().extension().and_then(OsStr::to_str) {
                Some(ext) => match (
                    filter,
                    ext.to_lowercase()
                        .as_str()
                        .parse::<SupportFileType>()
                        .unwrap(),
                ) {
                    (FileFilter::ALL, _)
                    | (FileFilter::IMAGE, SupportFileType::Image)
                    | (FileFilter::VIDEO, SupportFileType::Video) => {
                        Some(e.path().display().to_string())
                    }
                    _ => None,
                },
                None => None,
            },
            None => None,
        })
        .collect();
    println!(
        "[{}] {} files ({:?}) found",
        "✓".green(),
        valid_paths.len(),
        filter,
    );
    valid_paths
}

pub fn get_file_metadata(device_asset: &Vec<&DeviceAsset>) -> Vec<UploadAsset> {
    device_asset
        .iter()
        .map(|asset| {
            let path = Path::new(&asset.path);
            let file_name = path.file_name().unwrap().to_str().unwrap();
            let file_size = path.metadata().unwrap().len();
            let file_id = format!("{}-{}", file_name, file_size);
            let file_type = match path
                .extension()
                .unwrap()
                .to_str()
                .unwrap()
                .to_lowercase()
                .parse::<SupportFileType>()
                .unwrap()
            {
                SupportFileType::Image => "IMAGE",
                SupportFileType::Video => "VIDEO",
                SupportFileType::Other => "OTHER",
            }
            .to_string();

            let mime_type = match path
                .extension()
                .unwrap()
                .to_str()
                .unwrap()
                .to_lowercase()
                .as_str()
            {
                "png" => "image/png",
                "jpg" | "jpeg" | "jfif" => "image/jpeg",
                "webp" => "image/webp",
                "dng" => "image/dng",
                "heic" | "heif" => "image/heic",
                "tiff" => "image/tiff",
                "nef" => "image/nef",
                "mov" => "video/quicktime",
                "mp4" => "video/mp4",
                "3gp" => "video/3gpp",
                &_ => "application/octet-stream",
            };

            let created_at = match path.metadata().unwrap().created() {
                Ok(created_time) => iso8601(&created_time),
                Err(_) => iso8601(&SystemTime::now()),
            };

            let modified_at = match path.metadata().unwrap().modified() {
                Ok(modified_time) => iso8601(&modified_time),
                Err(_) => iso8601(&SystemTime::now()),
            };

            UploadAsset {
                id: file_id,
                path: asset.path.clone(),
                asset_type: file_type,
                created_at,
                modified_at,
                file_extension: path.extension().unwrap().to_str().unwrap().to_string(),
                mime_type: mime_type.to_string(),
            }
        })
        .collect_vec()
}

fn iso8601(st: &std::time::SystemTime) -> String {
    let dt: DateTime<Utc> = st.clone().into();
    format!("{}", dt.format("%+"))
}
