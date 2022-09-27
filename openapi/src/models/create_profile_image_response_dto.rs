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
pub struct CreateProfileImageResponseDto {
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "profileImagePath")]
    pub profile_image_path: String,
}

impl CreateProfileImageResponseDto {
    pub fn new(user_id: String, profile_image_path: String) -> CreateProfileImageResponseDto {
        CreateProfileImageResponseDto {
            user_id,
            profile_image_path,
        }
    }
}

