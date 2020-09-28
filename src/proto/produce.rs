use crate::proto::{Wired, WireRead, WireWrite, RecordBatch, ApiRequest, ApiKey, TopicMap};

// Does not use flexible encoding
impl ApiRequest for Request {
    const API_KEY: ApiKey = ApiKey::Produce;
    const FLEXIBLE_VER: usize = 99;

    type Response = Response;
}

#[derive(Wired)]
pub struct ProducePart {
    pub(crate) partition: i32,
    pub(crate) record_set: RecordBatch,
}

#[derive(Wired)]
pub struct Request {
    #[wired(since = 3)]
    pub(crate) transactional_id: Option<Option<String>>,
    pub(crate) acks: i16,
    pub(crate) timeout: i32,
    pub(crate) topic_data: TopicMap<ProducePart>,
}

#[derive(Wired)]
pub struct ProduceResponseBatchErrorItem {
    batch_index: i32,
    batch_index_error_msg: Option<String>,
}

#[derive(Wired)]
pub struct ProduceResponsePartition {
    partition: i32,
    error_code: i16,
    base_offset: i64,
    #[wired(since = 2)]
    log_append_time: Option<i64>,
    #[wired(since = 5)]
    log_start_offset: Option<i64>,
    #[wired(since = 8)]
    record_errors: Option<Vec<ProduceResponseBatchErrorItem>>,
    #[wired(since = 8)]
    error_message: Option<Option<String>>,
}

#[derive(Wired)]
pub struct Response {
    responses: TopicMap<ProduceResponsePartition>,
    #[wired(since = 1)]
    throttle_ms: Option<i32>,
}