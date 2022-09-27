/*
 * Immich
 *
 * Immich API
 *
 * The version of the OpenAPI document: 1.17.0
 * 
 * Generated by: https://openapi-generator.tech
 */


/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TimeGroupEnum {
    #[serde(rename = "day")]
    Day,
    #[serde(rename = "month")]
    Month,

}

impl ToString for TimeGroupEnum {
    fn to_string(&self) -> String {
        match self {
            Self::Day => String::from("day"),
            Self::Month => String::from("month"),
        }
    }
}

impl Default for TimeGroupEnum {
    fn default() -> TimeGroupEnum {
        Self::Day
    }
}



