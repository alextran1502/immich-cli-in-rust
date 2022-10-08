use std::{ffi::OsStr, path::Path, str::FromStr};

use walkdir::WalkDir;

use crate::FileFilter;
use colored::*;

use super::models::DeviceAsset;

#[derive(Debug, PartialEq)]
enum FileType {
    Image,
    Video,
}

impl FromStr for FileType {
    type Err = ();
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "png" | "jpg" | "jpeg" | "webp" | "dng" => Ok(FileType::Image),
            "mov" | "mp4" => Ok(FileType::Video),
            _ => Err(()),
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

pub fn get_file_metadata(device_asset: &Vec<&DeviceAsset>) {
    println!("[5] {} {}", "Getting file metadata", "...");
    device_asset.iter().for_each(|asset| {
        let path = Path::new(&asset.path);
        let file_name = path.file_name().unwrap().to_str().unwrap();
        let file_size = path.metadata().unwrap().len();
        let fild_id = format!("{}-{}", file_name, file_size);
        let file_type = match path
            .extension()
            .unwrap()
            .to_str()
            .unwrap()
            .parse::<FileType>()
            .unwrap()
        {
            FileType::Image => "IMAGE",
            FileType::Video => "VIDEO",
        };

        println!("{:?}", file_type);
    });
}
