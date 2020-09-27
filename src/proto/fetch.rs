use crate::proto::{Wired, WireRead, WireWrite, MinVer, IsolationLevel, TopicMap, RecordBatch, ApiKey, ApiRequest};


#[derive(Wired)]
pub struct FetchPartitions {
    partition: i32,
    current_leader_epoch: MinVer<i32, 9>,
    offset: i64,
    log_start_offset: MinVer<i64, 5>,
    max_bytes: i32,
}

#[derive(Wired)]
pub struct ForgottenTopics {
    topic: String,
    partitions: Vec<i32>,
}

impl ApiRequest for FetchRequest {
    const API_KEY: ApiKey = ApiKey::Fetch;
    type Response = FetchResponse;
}

#[derive(Wired)]
pub struct FetchRequest {
    replica_id: i32,
    max_wait: i32,
    min_bytes: i32,
    max_bytes: MinVer<i32, 3>,
    isolation: MinVer<IsolationLevel, 4>,
    session_id: MinVer<i32, 7>,
    session_epoch: MinVer<i32, 7>,
    topics: TopicMap<FetchPartitions>,
    forgotten_topics_data: MinVer<ForgottenTopics, 7>,
    rack_id: MinVer<String, 11>,
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
    last_stable_offset: MinVer<i64, 4>,
    log_start_offset: MinVer<i64, 5>,
    aborted_txs: MinVer<Vec<FetchResponseAbortedTx>, 4>,
    preferred_read_replic: MinVer<i32, 11>,
    record_set: RecordBatch,
}

#[derive(Wired)]
pub struct FetchResponseTopic {
    topic: String,
    partitions: Vec<FetchResponsePart>,
}

#[derive(Wired)]
pub struct FetchResponse {
    throttle_time_ms: MinVer<i32, 1>,
    error_code: MinVer<i16, 7>,
    session_id: MinVer<i32, 7>,
    responses: Vec<FetchResponseTopic>,
}
