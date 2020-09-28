use crate::proto::{TagBuffer, ApiRequest, ApiKey};

impl ApiRequest for Request {
    const API_KEY: ApiKey = ApiKey::ListGroups;
    const FLEXIBLE_VER: usize = 3;
    type Response = Response;
}

#[derive(Debug, Wired)]
pub struct StatesFilter {
    value: String
}

#[derive(Debug, Wired)]
pub struct Request {
    #[wired(since = 4)]
    states_filter: Option<Vec<StatesFilter>>,

    #[wired(since = 3)]
    tag_buffer: Option<TagBuffer>,
}

#[derive(Debug, Wired)]
pub struct Group {
    group_id: String,
    protocol_type: String,
    #[wired(since = 4)]
    group_state: Option<String>,

    #[wired(since = 3)]
    tag_buffer: Option<TagBuffer>,
}

#[derive(Debug, Wired)]
pub struct Response {
    #[wired(since = 1)]
    throttle_time_ms: Option<i32>,
    error_code: i16,
    groups: Vec<Group>,

    #[wired(since = 3)]
    tag_buffer: Option<TagBuffer>,
}