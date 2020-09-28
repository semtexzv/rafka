use bytes::Bytes;
use crate::proto::{TagBuffer, ApiRequest, ApiKey};

impl ApiRequest for Request {
    const API_KEY: ApiKey = ApiKey::JoinGroup;
    const FLEXIBLE_VER: usize = 6;
    type Response = Response;
}

#[derive(Debug, Wired)]
pub struct Proto {
    name: String,
    metadata: Bytes,

    #[wired(since = 6)]
    tags: Option<TagBuffer>,
}

#[derive(Debug, Wired)]
pub struct Request {
    group_id: String,
    session_timeout: i32,
    #[wired(since = 1)]
    rebalance_timeout: Option<i32>,
    member_id: String,
    group_instance_id: Option<String>,
    protocol_type: String,
    protocols: Vec<Proto>,

    #[wired(since = 6)]
    tags: Option<TagBuffer>,
}

#[derive(Debug, Wired)]
pub struct Members {
    member_id: String,
    #[wired(since = 5)]
    group_instance_id: Option<Option<String>>,
    metadata: Bytes,

    #[wired(since = 6)]
    tags: Option<TagBuffer>,
}

#[derive(Debug, Wired)]
pub struct Response {
    #[wired(since = 2)]
    throttle_time_ms: Option<i32>,
    error_code: i16,
    generation_id: i32,
    protocol_type: String,
    protocol_name: String,
    leader: String,
    member_id: String,
    members: Vec<Members>,

    #[wired(since = 6)]
    tags: Option<TagBuffer>,
}