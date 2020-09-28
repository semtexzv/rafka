use crate::proto::{TagBuffer, ApiRequest, ApiKey};

impl ApiRequest for Request {
    const API_KEY: ApiKey = ApiKey::FindCoordinator;
    const FLEXIBLE_VER: usize = 3;
    type Response = Response;
}
#[derive(Debug, Wired)]
pub struct Request {
    key: String,
    #[wired(since = 1)]
    key_type: Option<i8>,

    #[wired(since = 3)]
    tags: Option<TagBuffer>,
}

#[derive(Debug, Wired)]
pub struct Response {
    #[wired(since = 1)]
    throttle_time_ms: Option<i32>,
    error_code: i16,
    #[wired(since = 1)]
    error_message: Option<Option<String>>,
    node_id: i32,
    host: String,
    port: i32,

    #[wired(since = 3)]
    tags: Option<TagBuffer>,
}