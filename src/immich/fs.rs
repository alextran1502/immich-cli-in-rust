use chrono::prelude::{DateTime, Utc};
use exif::{In, Reader, Tag, Value};
use itertools::Itertools;
use std::{ffi::OsStr, fs::File, io::BufReader, path::Path, time::SystemTime};
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
                    (FileFilter::ALL, SupportFileType::Image | SupportFileType::Video)
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

            let mut created_at = match path.metadata().unwrap().modified() {
                Ok(created_time) => to_iso8601(&created_time),
                Err(_) => to_iso8601(&SystemTime::now()),
            };

            if file_type == "IMAGE" {
                created_at = get_exif_time(&path.to_str().unwrap());
            }

            let modified_at = match path.metadata().unwrap().modified() {
                Ok(modified_time) => to_iso8601(&modified_time),
                Err(_) => to_iso8601(&SystemTime::now()),
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

fn to_iso8601(st: &std::time::SystemTime) -> String {
    let dt: DateTime<Utc> = st.clone().into();
    format!("{}", dt.format("%+"))
}

fn get_created_at(path: &Path) -> String {
    let created_at = match path.metadata().unwrap().created() {
        Ok(created_time) => to_iso8601(&created_time),
        Err(_) => to_iso8601(&SystemTime::now()),
    };

    created_at
}

fn get_exif_time(path: &str) -> String {
    let exif =
        match Reader::new().read_from_container(&mut BufReader::new(&File::open(path).unwrap())) {
            Ok(exif) => exif,
            Err(_) => return get_created_at(Path::new(path)),
        };

    let datetime = match exif.get_field(Tag::DateTimeOriginal, In::PRIMARY) {
        Some(field) => match field.value {
            Value::Ascii(ref vec) => match vec.get(0) {
                Some(date) => {
                    let datetimedata = exif::DateTime::from_ascii(date).unwrap();
                    let iso8601 = format!(
                        "{}-{:02}-{:02}T{:02}:{:02}:{:02}.000Z",
                        datetimedata.year,
                        datetimedata.month,
                        datetimedata.day,
                        datetimedata.hour,
                        datetimedata.minute,
                        datetimedata.second
                    );

                    iso8601
                }
                None => return get_created_at(Path::new(path)),
            },
            _ => return get_created_at(Path::new(path)),
        },
        None => return get_created_at(Path::new(path)),
    };

    datetime
}
