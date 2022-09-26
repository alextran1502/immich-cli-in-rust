/*
 * Immich
 *
 * Immich API
 *
 * The version of the OpenAPI document: 1.17.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AssetResponseDto {
    #[serde(rename = "type")]
    pub r#type: crate::models::AssetTypeEnum,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "deviceAssetId")]
    pub device_asset_id: String,
    #[serde(rename = "ownerId")]
    pub owner_id: String,
    #[serde(rename = "deviceId")]
    pub device_id: String,
    #[serde(rename = "originalPath")]
    pub original_path: String,
    #[serde(rename = "resizePath")]
    pub resize_path: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "modifiedAt")]
    pub modified_at: String,
    #[serde(rename = "isFavorite")]
    pub is_favorite: bool,
    #[serde(rename = "mimeType")]
    pub mime_type: Option<String>,
    #[serde(rename = "duration")]
    pub duration: String,
    #[serde(rename = "webpPath")]
    pub webp_path: Option<String>,
    #[serde(rename = "encodedVideoPath")]
    pub encoded_video_path: Option<String>,
    #[serde(rename = "exifInfo", skip_serializing_if = "Option::is_none")]
    pub exif_info: Option<Box<crate::models::ExifResponseDto>>,
    #[serde(rename = "smartInfo", skip_serializing_if = "Option::is_none")]
    pub smart_info: Option<Box<crate::models::SmartInfoResponseDto>>,
}

impl AssetResponseDto {
    pub fn new(r#type: crate::models::AssetTypeEnum, id: String, device_asset_id: String, owner_id: String, device_id: String, original_path: String, resize_path: Option<String>, created_at: String, modified_at: String, is_favorite: bool, mime_type: Option<String>, duration: String, webp_path: Option<String>, encoded_video_path: Option<String>) -> AssetResponseDto {
        AssetResponseDto {
            r#type,
            id,
            device_asset_id,
            owner_id,
            device_id,
            original_path,
            resize_path,
            created_at,
            modified_at,
            is_favorite,
            mime_type,
            duration,
            webp_path,
            encoded_video_path,
            exif_info: None,
            smart_info: None,
        }
    }
}


