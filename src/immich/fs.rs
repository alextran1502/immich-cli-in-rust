use chrono::prelude::{DateTime, Utc};
use itertools::Itertools;
use std::{ffi::OsStr, path::Path, str::FromStr, time::SystemTime};
use walkdir::WalkDir;

use crate::{immich::models::UploadAsset, FileFilter};
use colored::*;

use super::models::DeviceAsset;

#[derive(Debug, PartialEq)]
enum FileType {
    Image,
    Video,
    Other,
}

impl FromStr for FileType {
    type Err = ();
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "png" | "jpg" | "jpeg" | "webp" | "dng" => Ok(FileType::Image),
            "mov" | "mp4" => Ok(FileType::Video),
            _ => Ok(FileType::Other),
        }
    }
}

pub fn dir_walk(path: &str, filter: &FileFilter) -> Vec<String> {
    println!("[4] {} {}", "Indexing directory", path.blue());
    let valid_paths: Vec<String> = WalkDir::new(path)
        .into_iter()
        .filter_map(|e| match e.ok().filter(|e| e.file_type().is_file()) {
            Some(e) => match e.path().extension().and_then(OsStr::to_str) {
                Some(ext) => match filter {
                    FileFilter::ALL => match ext.to_lowercase().as_str() {
                        "png" | "jpg" | "jpeg" | "webp" | "mov" | "dng" | "mp4" => {
                            Some(e.path().display().to_string())
                        }
                        _ => None,
                    },
                    FileFilter::IMAGE => match ext.to_lowercase().as_str() {
                        "png" | "jpg" | "jpeg" | "webp" | "dng" => {
                            Some(e.path().display().to_string())
                        }
                        _ => None,
                    },
                    FileFilter::VIDEO => match ext.to_lowercase().as_str() {
                        "mov" | "mp4" => Some(e.path().display().to_string()),
                        _ => None,
                    },
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
    println!("[5] {} {}", "Getting file metadata", "...");
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
                .parse::<FileType>()
                .unwrap()
            {
                FileType::Image => "IMAGE",
                FileType::Video => "VIDEO",
                FileType::Other => "OTHER",
            }
            .to_string();

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
            }
        })
        .collect_vec()
}

fn iso8601(st: &std::time::SystemTime) -> String {
    let dt: DateTime<Utc> = st.clone().into();
    format!("{}", dt.format("%+"))
}
