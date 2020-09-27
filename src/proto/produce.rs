use crate::proto::{Wired, WireRead, WireWrite, MinVer, RecordBatch, ApiRequest, ApiKey};

impl ApiRequest for ProduceRequest {
    const API_KEY: ApiKey = ApiKey::Produce;
    type Response = ProduceResponseV0;
}

#[derive(Wired)]
pub struct ProduceRecordDataData {
    pub(crate) partition: i32,
    pub(crate) record_set: RecordBatch,
}

#[derive(Wired)]
pub struct ProduceRecordData {
    pub(crate) topic: String,
    pub(crate) data: Vec<ProduceRecordDataData>,
}

#[derive(Wired)]
pub struct ProduceRequest {
    pub(crate) transactional_id: MinVer<Option<String>, 3>,
    pub(crate) acks: i16,
    pub(crate) timeout: i32,
    pub(crate) topic_data: Vec<ProduceRecordData>,
}

#[derive(Wired)]
pub struct ProduceResponseBatchErrorItem {
    batch_index: i32,
    batch_index_error_msg: Option<String>,
}

#[derive(Wired)]
pub struct ProduceResponsePartitionItem {
    partition: i32,
    error_code: i16,
    base_offset: i64,
    log_append_time: MinVer<i64, 2>,
    log_start_offset: MinVer<i64, 5>,
    record_errors: MinVer<Vec<ProduceResponseBatchErrorItem>, 6>,
    error_message: MinVer<Option<String>, 6>,
}

#[derive(Wired)]
pub struct ProduceResponseItem {
    topic: String,
    partition_responses: Vec<ProduceResponsePartitionItem>,
}

#[derive(Wired)]
pub struct ProduceResponseV0 {
    responses: Vec<ProduceResponseItem>,
    throttle_ms: MinVer<i32, 1>,
}