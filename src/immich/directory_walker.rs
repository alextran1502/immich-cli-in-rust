use std::{char::ToLowercase, ffi::OsStr, path::Path};

use walkdir::WalkDir;

pub async fn dir_walk(path: &str) -> Vec<String> {
    let valid_paths: Vec<String> = WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .map(|e| {
            match e
                .path()
                .extension()
                .and_then(OsStr::to_str)
                .unwrap_or("")
                .to_lowercase()
                .as_str()
            {
                "png" | "jpg" | "jpeg" | "webp" => e.path().display().to_string(),
                _ => "".to_string(),
            }
        })
        .filter(|file_path| !file_path.is_empty())
        .collect();
    println!("valid path {:?}", valid_paths.len());
    valid_paths
}
