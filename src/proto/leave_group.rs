use crate::proto::{ApiRequest, ApiKey, TagBuffer};

impl ApiRequest for Request {
    const API_KEY: ApiKey = ApiKey::LeaveGroup;
    const FLEXIBLE_VER: usize = 4;
    type Response = Response;
}
#[derive(Debug, Wired)]
pub struct MemberLeave {
    member_id: String,
    group_instance_id: Option<String>,

    #[wired(since = 4)]
    tags: Option<TagBuffer>,
}

#[derive(Debug, Wired)]
pub struct Request {
    group_id: String,
    #[wired(since = 2)]
    member_id: Option<String>,
    #[wired(since = 3)]
    members: Option<Vec<MemberLeave>>,

    #[wired(since = 4)]
    tags: Option<TagBuffer>,
}

#[derive(Debug, Wired)]
pub struct MemberLeaveResp {
    data: MemberLeave,
    error_code: i16,

    #[wired(since = 4)]
    tags: Option<TagBuffer>,
}

#[derive(Debug, Wired)]
pub struct Response {
    #[wired(since = 1)]
    throttle_time_ms: Option<i32>,
    error_code: i16,
    members: Vec<MemberLeaveResp>,

    #[wired(since = 4)]
    tags: Option<TagBuffer>,
}