use crate::proto::{TagBuffer, ApiRequest, ApiKey};


impl ApiRequest for Request {
    const API_KEY: ApiKey = ApiKey::Hearbeat;
    const FLEXIBLE_VER: usize = 4;
    type Response = Response;
}
#[derive(Debug, Wired)]
pub struct Request {
    group_id: String,
    generation_id: i32,
    member_id: String,
    #[wired(since = 3)]
    group_instance_id: Option<Option<String>>,
    #[wired(since = 4)]
    tags: Option<TagBuffer>,
}

#[derive(Debug, Wired)]
#[wired(compact(since = 4))]
pub struct Response {
    #[wired(since = 1)]
    throttle_time_ms: Option<i32>,
    error_code: i16,
    #[wired(since = 4)]
    tags: Option<TagBuffer>,
}
