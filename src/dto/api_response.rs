#[derive(Clone, Debug, serde::Deserialize)]
#[serde(bound(deserialize = "T: serde::de::DeserializeOwned + std::fmt::Debug"))]
pub struct ResponseFormat<T: serde::de::DeserializeOwned + std::fmt::Debug> {
    pub success: bool,
    pub data: T,
    pub meta: Meta,
}

#[derive(Clone, serde::Deserialize, Debug)]
pub struct Meta {
    pub time_to_process: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}
