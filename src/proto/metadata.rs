use crate::proto::{Wired, WireRead, WireWrite, ApiRequest, ApiKey, TagBuffer};
use crate::client::Client;
use std::future::Future;
use tower::{ServiceExt, Service};
use std::ops::DerefMut;
use crate::transport::CallReq;

impl ApiRequest for Request {
    const API_KEY: ApiKey = ApiKey::Metadata;
    const FLEXIBLE_VER: usize = 9;
    type Response = Response;
}

#[derive(Debug, Wired)]
pub struct Topic {
    value: String,

    #[wired(since = 9)]
    tags: Option<TagBuffer>,
}

#[derive(Debug, Wired)]
pub struct Request {
    topics: Vec<Topic>,
    #[wired(since = 4)]
    allow_auto_topic_creation: Option<bool>,
    #[wired(since = 8)]
    include_cluster_auth_ops: Option<bool>,
    #[wired(since = 8)]
    include_topic_auth_ops: Option<bool>,

    #[wired(since = 9)]
    tags: Option<TagBuffer>,
}


#[derive(Debug, Wired)]
pub struct MetadataPartition {
    error_code: i16,
    part_index: i32,
    leader_id: i32,
    #[wired(since = 7)]
    leader_epoch: Option<i32>,
    replicas: Vec<i32>,
    isr_nodes: Vec<i32>,
    #[wired(since = 5)]
    offline_replicas: Option<Vec<i32>>,

    #[wired(since = 9)]
    tags: Option<TagBuffer>,
}


#[derive(Debug, Wired)]
pub struct MetadataBroker {
    node_id: i32,
    host: String,
    port: i32,
    #[wired(since = 1)]
    rack: Option<Option<String>>,

    #[wired(since = 9)]
    tags: Option<TagBuffer>,
}

#[derive(Debug, Wired)]
pub struct MetadataTopic {
    error_code: i16,
    name: String,

    #[wired(since = 1)]
    is_internal: Option<bool>,
    parts: Vec<MetadataPartition>,

    #[wired(since = 8)]
    topic_auth_ops: Option<i32>,

    #[wired(since = 9)]
    tags: Option<TagBuffer>,
}

#[derive(Debug, Wired)]
pub struct Response {
    #[wired(since = 3)]
    throttle_time_ms: Option<i32>,
    brokers: Vec<MetadataBroker>,
    #[wired(since = 2)]
    cluster_id: Option<Option<String>>,
    #[wired(since = 1)]
    controller_id: Option<i32>,
    topics: Vec<MetadataTopic>,
    #[wired(since = 8)]
    cluster_auth_ops: Option<i32>,

    #[wired(since = 9)]
    tags: Option<TagBuffer>,
}

impl Client {
    pub fn metadata(&self, topics: Vec<String>) -> impl Future<Output=crate::Result<Response>> {
        let req = Request {
            topics: topics.into_iter().map(|t| Topic { value: t, tags: TagBuffer {}.into() }).collect(),
            allow_auto_topic_creation: true.into(),
            include_cluster_auth_ops: true.into(),
            include_topic_auth_ops: true.into(),
            tags: TagBuffer {}.into(),
        };
        let client = self.client.clone();
        let versions = self.version_match(ApiKey::Metadata, (4, 99));
        async move {
            let mut client = client.lock().await;
            let client = ServiceExt::<()>::ready_oneshot(client.deref_mut()).await?;
            let res = client.call(CallReq::new(versions.1, req)).await?;

            Ok(res)
        }
    }
}

#[tokio::test]
async fn test_metadata() {
    let client = Client::connect("localhost:9092").await.unwrap();

    let meta = client.metadata(vec!["test".to_string()]).await.unwrap();

    assert_eq!(meta.throttle_time_ms, 0.into());
    assert_eq!(meta.brokers[0].host, "kafka");
    assert_eq!(meta.brokers[0].node_id, 1);
    assert_eq!(meta.brokers[0].port, 9092);
    assert_eq!(meta.brokers[0].rack, Some(None));

    assert_eq!(meta.topics[0].name, "test");
    assert_eq!(meta.topics[0].error_code, 0);
    assert_eq!(meta.topics[0].parts[0].error_code, 0);
    dbg!(meta);
}