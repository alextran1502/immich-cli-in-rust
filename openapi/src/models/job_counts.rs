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
pub struct JobCounts {
    #[serde(rename = "active")]
    pub active: i32,
    #[serde(rename = "completed")]
    pub completed: i32,
    #[serde(rename = "failed")]
    pub failed: i32,
    #[serde(rename = "delayed")]
    pub delayed: i32,
    #[serde(rename = "waiting")]
    pub waiting: i32,
}

impl JobCounts {
    pub fn new(active: i32, completed: i32, failed: i32, delayed: i32, waiting: i32) -> JobCounts {
        JobCounts {
            active,
            completed,
            failed,
            delayed,
            waiting,
        }
    }
}

