use crate::proto::{TagBuffer, ApiRequest, ApiKey};
use bytes::Bytes;

impl ApiRequest for Request {
    const API_KEY: ApiKey = ApiKey::DescribeGroups;
    const FLEXIBLE_VER: usize = 5;
    type Response = Response;
}

#[derive(Debug, Wired)]
#[wired(compact(since = 5))]
pub struct Request {
    groups: Vec<String>,
    #[wired(since = 3)]
    include_auth_ops: Option<bool>,

    #[wired(since = 5)]
    tags: Option<TagBuffer>,

}

#[derive(Debug, Wired)]
#[wired(compact(since = 5))]
pub struct ResGroup {
    error_code: i16,

    group_id: String,
    group_state: String,
    protocol_type: String,
    protocol_data: String,
    members: Vec<ResMember>,
    #[wired(since = 3)]
    authorized_operations: Option<i32>,

    #[wired(since = 5)]
    tags: Option<TagBuffer>,
}

#[derive(Debug, Wired)]

pub struct ResMember {
    member_id: String,
    #[wired(since = 4)]
    group_instance_id: Option<Option<String>>,
    client_id: String,
    client_host: String,
    member_metadata: Bytes,
    member_assignment: Bytes,

    #[wired(since = 5)]
    tags: Option<TagBuffer>,
}

#[derive(Debug, Wired)]
pub struct Response {
    #[wired(since = 1)]
    throttle_time_ms: Option<i32>,
    groups: Vec<ResGroup>,

    #[wired(since = 5)]
    tags: Option<TagBuffer>,
}