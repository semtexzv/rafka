use crate::proto::{TagBuffer, ApiRequest, ApiKey};

impl ApiRequest for Request {
    const API_KEY: ApiKey = ApiKey::CreatePartitions;
    const FLEXIBLE_VER: usize = 2;
    type Response = Response;
}

#[derive(Debug, Wired)]
pub struct TopicItem {
    name: String,
    count: i32,
    assignments: Vec<i32>,

    #[wired(since = 6)]
    tags: Option<TagBuffer>,
}

#[derive(Debug, Wired)]
pub struct Request {
    topics: Vec<TopicItem>,
    timeout_ms: i32,
    validate_only: bool,

    #[wired(since = 6)]
    tags: Option<TagBuffer>,
}

#[derive(Debug, Wired)]
pub struct ResItem {
    name: String,
    error_code: i16,
    error_message: Option<String>,

    #[wired(since = 2)]
    tags: Option<TagBuffer>,
}

#[derive(Debug, Wired)]
pub struct Response {
    throttle_time_ms: i32,
    results: Vec<ResItem>,

    #[wired(since = 2)]
    tags: Option<TagBuffer>,
}