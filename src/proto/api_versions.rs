use crate::proto::{ApiKey, ApiRequest, TagBuffer};
use crate::proto::{Wired, WireRead, WireWrite};


impl ApiRequest for Request {
    const API_KEY: ApiKey = ApiKey::ApiVersions;
    const FLEXIBLE_VER: usize = 3;
    type Response = ApiVersionsResponse;
}

#[derive(Debug, Wired, Default)]
pub struct Request {
    #[wired(since = 3)]
    pub client_software_name: Option<String>,
    #[wired(since = 3)]
    pub client_software_version: Option<String>,

    #[wired(since = 3)]
    pub tags: Option<TagBuffer>,
}

#[derive(Wired, Debug)]
pub struct ApiVersionsItem {
    pub api_key: ApiKey,
    pub min_version: i16,
    pub max_version: i16,

    #[wired(since = 3)]
    tags: Option<TagBuffer>,
}

#[derive(Wired, Debug)]
pub struct ApiVersionsResponse {
    pub error_code: i16,
    pub versions: Vec<ApiVersionsItem>,
    #[wired(since = 1)]
    pub throttle_time_ms: Option<i32>,

    #[wired(since = 3)]
    tags: Option<TagBuffer>,
}

