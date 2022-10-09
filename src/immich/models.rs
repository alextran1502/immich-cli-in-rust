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
}
