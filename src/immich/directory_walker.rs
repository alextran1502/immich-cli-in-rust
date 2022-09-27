use std::ffi::OsStr;

use walkdir::WalkDir;

pub fn dir_walk(path: &str) -> Vec<String> {
    let valid_paths: Vec<String> = WalkDir::new(path)
        .into_iter()
        .filter_map(|e| match e.ok().filter(|e| e.file_type().is_file()) {
            Some(e) => match e.path().extension().and_then(OsStr::to_str) {
                Some(ext) => match ext.to_lowercase().as_str() {
                    "png" | "jpg" | "jpeg" | "webp" => Some(e.path().display().to_string()),
                    _ => None,
                },
                None => None,
            },
            None => None,
        })
        .collect();

    valid_paths
}
