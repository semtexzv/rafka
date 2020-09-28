use crate::proto::{TopicMap, TagBuffer, ApiRequest, ApiKey};

impl ApiRequest for Request {
    const API_KEY: ApiKey = ApiKey::OffsetCommit;
    const FLEXIBLE_VER: usize = 8;
    type Response = Response;
}

#[derive(Debug, Wired)]
pub struct PartData {
    partition_index: i32,
    commited_offset: i64,
    #[wired(since = 1, until = 1)]
    commit_timestamp: Option<i64>,
    #[wired(since = 6)]
    leader_epoch: Option<i32>,
    metadata: Option<String>,

    #[wired(since = 8)]
    tags: Option<TagBuffer>,
}

#[derive(Debug, Wired)]
pub struct Request {
    group_id: String,
    #[wired(since = 1)]
    generation_id: Option<i32>,
    #[wired(since = 1)]
    member_id: Option<String>,
    #[wired(since = 2, until = 4)]
    retention_time_ms: Option<i64>,
    #[wired(since = 7)]
    group_instance_id: Option<Option<String>>,
    topics: TopicMap<PartData>,

    #[wired(since = 8)]
    tags : Option<TagBuffer>
}


#[derive(Debug, Wired)]
pub struct ResponseParts {
    partition_index: i32,
    error_code: i16,

    #[wired(since = 8)]
    tags : Option<TagBuffer>
}

#[derive(Debug, Wired)]
pub struct Response {
    #[wired(since = 3)]
    throttle_time_ms: Option<i32>,
    topics: TopicMap<ResponseParts>,

    #[wired(since = 8)]
    tags : Option<TagBuffer>
}