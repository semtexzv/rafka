use crate::proto::{TopicMap, ApiRequest, ApiKey};


// Does not use flexible encoding
impl ApiRequest for Request {
    const API_KEY: ApiKey = ApiKey::OffsetDelete;
    const FLEXIBLE_VER: usize = 99;
    type Response = ();
}
#[derive(Debug, Wired)]
pub struct Request {
    group_id: String,
    topics: TopicMap<i32>,
}

#[derive(Debug, Wired)]
pub struct RespPart {
    partition_index: i32,
    error_code: i16,
}

#[derive(Debug, Wired)]
pub struct Response {
    error_code: i16,
    throttle_time_ms: i32,
    topics: TopicMap<RespPart>,
}