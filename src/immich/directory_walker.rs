use std::ffi::OsStr;

use walkdir::WalkDir;

use crate::FileFilter;
use colored::*;

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
