use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub struct DeviceAsset {
    pub id: String,
    pub path: String,
}

#[derive(Debug, PartialEq)]
pub struct UploadAsset {
    pub id: String,
    pub path: String,
    pub asset_type: String,
    pub created_at: String,
    pub modified_at: String,
    pub file_extension: String,
    pub mime_type: String,
}

#[derive(Debug, PartialEq)]
pub enum SupportFileType {
    Image,
    Video,
    Other,
}

impl FromStr for SupportFileType {
    type Err = ();
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "png" | "jpg" | "jpeg" | "webp" | "dng" | "heic" | "heif" | "tiff" | "nef" | "jfif" => {
                Ok(SupportFileType::Image)
            }
            "mov" | "mp4" | "3gp" => Ok(SupportFileType::Video),
            _ => Ok(SupportFileType::Other),
        }
    }
}

// Folder
pub type FolderName = String;
