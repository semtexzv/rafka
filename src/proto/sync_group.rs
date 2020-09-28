use bytes::Bytes;
use crate::proto::{TagBuffer, ApiRequest, ApiKey};

impl ApiRequest for Request {
    const API_KEY: ApiKey = ApiKey::SyncGroup;
    const FLEXIBLE_VER: usize = 4;
    type Response = Response;
}
#[derive(Debug, Wired)]
pub struct Assignment {
    member_id: String,
    assign: Bytes,

    #[wired(since = 4)]
    tags: Option<TagBuffer>,
}


#[derive(Debug, Wired)]
pub struct Request {
    group_id: String,
    generation_id: i32,
    member_id: String,
    #[wired(since = 3)]
    group_instance_id: Option<Option<String>>,
    #[wired(since = 4)]
    protocol_type: Option<Option<String>>,
    #[wired(since = 4)]
    protocol_name: Option<Option<String>>,
    assignments: Vec<Assignment>,

    #[wired(since = 4)]
    tags: Option<TagBuffer>,
}

#[derive(Debug, Wired)]
pub struct Response {
    #[wired(since = 1)]
    throttle_time_ms: Option<i32>,
    error_code: i16,
    #[wired(since = 5)]
    protocol_type: Option<String>,
    #[wired(since = 5)]
    protocol_name: Option<String>,

    assignment: Bytes,

    #[wired(since = 4)]
    tags: Option<TagBuffer>,
}