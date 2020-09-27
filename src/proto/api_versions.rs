use crate::proto::{ApiKey, ApiRequest};
use crate::proto::{Wired, WireRead, WireWrite, MinVer};

#[derive(Wired)]
pub struct Request {}

impl ApiRequest for Request {
    const API_KEY: ApiKey = ApiKey::ApiVersions;
    type Response = ApiVersionsResponse;
}

#[derive(Wired, Debug)]
pub struct ApiVersionsItem {
    pub api_key: ApiKey,
    pub min_version: i16,
    pub max_version: i16,
}

#[derive(Wired, Debug)]
pub struct ApiVersionsResponse {
    pub error_code: i16,
    pub versions: Vec<ApiVersionsItem>,
}

