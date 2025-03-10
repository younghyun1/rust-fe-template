#[derive(Clone, Debug, serde::Deserialize)]
#[serde(bound(deserialize = "T: serde::de::DeserializeOwned + std::fmt::Debug"))]
pub struct ResponseFormat<T: serde::de::DeserializeOwned + std::fmt::Debug> {
    pub success: bool,
    pub data: Option<T>,
    pub meta: Option<Meta>,
    pub error_code: Option<u32>,
    pub message: Option<String>,
}

#[derive(Clone, serde::Deserialize, Debug)]
pub struct Meta {
    pub time_to_process: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}
