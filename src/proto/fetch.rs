use crate::proto::{Wired, WireRead, WireWrite, IsolationLevel, TopicMap, RecordBatch, ApiKey, ApiRequest};

// Fetch does not use compact encoding
impl ApiRequest for Request {
    const API_KEY: ApiKey = ApiKey::Fetch;
    const FLEXIBLE_VER: usize = 99;

    type Response = Response;
}

#[derive(Wired)]
pub struct FetchPartitions {
    partition: i32,
    #[wired(since = 9)]
    current_leader_epoch: Option<i32>,
    offset: i64,
    #[wired(since = 5)]
    log_start_offset: Option<i64>,
    max_bytes: i32,
}

#[derive(Wired)]
pub struct Request {
    replica_id: i32,
    max_wait_ms: i32,
    min_bytes: i32,
    #[wired(since = 3)]
    max_bytes: Option<i32>,
    #[wired(since = 4)]
    isolation: Option<IsolationLevel>,
    #[wired(since = 7)]
    session_id: Option<i32>,
    #[wired(since = 7)]
    session_epoch: Option<i32>,
    topics: TopicMap<FetchPartitions>,
    #[wired(since = 7)]
    forgotten_topics_data: Option<TopicMap<i32>>,
    #[wired(since = 11)]
    rack_id: Option<String>,
}

#[derive(Wired)]
pub struct FetchResponseAbortedTx {
    producer_id: i64,
    first_offset: i64,
}

#[derive(Wired)]
pub struct FetchResponsePart {
    partition: i32,
    error_code: i16,
    hwm: i64,
    #[wired(since = 4)]
    last_stable_offset: Option<i64>,
    #[wired(since = 5)]
    log_start_offset: Option<i64>,
    #[wired(since = 4)]
    aborted_transactions: Option<Vec<FetchResponseAbortedTx>>,
    #[wired(since = 11)]
    preferred_read_replica: Option<i32>,
    record_set: RecordBatch,
}

#[derive(Wired)]
pub struct Response {
    #[wired(since = 1)]
    throttle_time_ms: Option<i32>,
    #[wired(since = 7)]
    error_code: Option<i16>,
    #[wired(since = 7)]
    session_id: Option<i32>,
    responses: TopicMap<FetchResponsePart>,
}
