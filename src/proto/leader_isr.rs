use crate::proto::{TopicMap, ApiRequest, ApiKey, TagBuffer};


impl ApiRequest for Request {
    const API_KEY: ApiKey = ApiKey::LeaderAndIsr;
    const FLEXIBLE_VER: usize = 4;

    type Response = Response;
}

#[derive(Debug, Wired)]
pub struct PartStateData {
    part_index: i32,
    controller_epoch: i32,
    leader: i32,
    leader_epoch: i32,
    isr: Vec<i32>,
    zk_ver: i32,
    replicas: Vec<i32>,
    #[wired(since = 3)]
    adding_replicas: Option<Vec<i32>>,
    #[wired(since = 3)]
    removing_replicas: Option<Vec<i32>>,
    #[wired(since = 1)]
    is_new: Option<bool>,

    #[wired(since = 4)]
    tags: Option<TagBuffer>,
}

#[derive(Debug, Wired)]
pub struct PartStates {
    topic_name: String,
    data: PartStateData,

    #[wired(since = 4)]
    tags: Option<TagBuffer>,
}

#[derive(Debug, Wired)]
pub struct LiveLeader {
    broker_id: i32,
    host_name: String,
    port: i32,

    #[wired(since = 4)]
    tags: Option<TagBuffer>,
}

#[derive(Debug, Wired)]
pub struct Request {
    controller_id: i32,
    controller_epoch: i32,
    #[wired(since = 3)]
    broker_epoch: Option<i32>,
    #[wired(until = 1)]
    ungrouped_part_states: Option<PartStates>,
    #[wired(since = 2)]
    topic_states: Option<TopicMap<PartStateData>>,
    live_readers: Vec<LiveLeader>,

    #[wired(since = 4)]
    tags: Option<TagBuffer>,
}

#[derive(Debug, Wired)]
pub struct PartError {
    topic_name: String,
    part_idx: i32,
    error_code: i32,

    #[wired(since = 4)]
    tags: Option<TagBuffer>,
}

#[derive(Debug, Wired)]
pub struct Response {
    error_code: i16,
    part_errors: Vec<PartError>,

    #[wired(since = 4)]
    tags: Option<TagBuffer>,
}