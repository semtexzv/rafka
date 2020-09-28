use crate::proto::{TopicMap, TagBuffer, ApiRequest, ApiKey};

impl ApiRequest for Request {
    const API_KEY: ApiKey = ApiKey::OffsetFetch;
    const FLEXIBLE_VER: usize = 6;
    type Response = Response;
}
#[derive(Debug, Wired)]
pub struct Request {
    group_id: String,
    values: TopicMap<i32>,
    #[wired(since = 7)]
    require_stable: Option<bool>,


    #[wired(since = 6)]
    tags: Option<TagBuffer>,
}

#[derive(Debug, Wired)]
pub struct RespPartData {
    partition_index: i32,
    commited_offset: i64,
    #[wired(since = 5)]
    commited_leader_epoch: Option<i32>,
    metadata: Option<String>,
    error_code: i16,

    #[wired(since = 6)]
    tags: Option<TagBuffer>,
}

#[derive(Debug, Wired)]
pub struct Response {
    #[wired(since = 3)]
    throttle_time_ms: Option<i32>,
    topics: TopicMap<RespPartData>,

    #[wired(since = 2)]
    error_code: Option<i16>,

    #[wired(since = 6)]
    tags: Option<TagBuffer>,
}