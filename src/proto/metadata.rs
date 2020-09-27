use crate::proto::{Wired, WireRead, WireWrite, MinVer, ApiRequest, ApiKey};
use crate::client::Client;
use std::future::Future;
use tower::{ServiceExt, Service};
use std::ops::DerefMut;
use crate::transport::CallReq;

impl ApiRequest for Request {
    const API_KEY: ApiKey = ApiKey::Metadata;
    type Response = Response;
}

#[derive(Debug, Wired)]
pub struct Request {
    topics: Vec<String>,
    allow_auto_topic_creation: MinVer<bool, 4>,
    include_cluster_auth_ops: MinVer<bool, 8>,
    include_topic_auth_ops: MinVer<bool, 8>,
}


#[derive(Debug, Wired)]
pub struct MetadataPartition {
    error_code: i16,
    part_index: i32,
    leader_id: i32,
    leader_epoch: MinVer<i32, 7>,
    replicas: Vec<i32>,
    isr_nodes: Vec<i32>,
    offline_replicas: MinVer<Vec<i32>, 5>,
}


#[derive(Debug, Wired)]
pub struct MetadataBroker {
    node_id: i32,
    host: String,
    port: i32,
    rack: MinVer<Option<String>, 1>,
}

#[derive(Debug, Wired)]
pub struct MetadataTopic {
    error_code: i16,
    name: String,
    is_internal: MinVer<bool, 1>,
    parts: Vec<MetadataPartition>,
    topic_auth_ops: MinVer<i32, 8>,
}

#[derive(Debug, Wired)]
pub struct Response {
    throttle_time_ms: MinVer<i32, 3>,
    brokers: Vec<MetadataBroker>,
    cluster_id: MinVer<Option<String>, 2>,
    controller_id: MinVer<i32, 1>,
    topics: Vec<MetadataTopic>,
    cluster_auth_ops: MinVer<i32, 8>,
}

impl Client {
    pub fn metadata(&self, topics: Vec<String>) -> impl Future<Output=crate::Result<Response>> {
        let req = Request {
            topics: topics,
            allow_auto_topic_creation: true.into(),
            include_cluster_auth_ops: None.into(),
            include_topic_auth_ops: None.into(),
        };
        let client = self.client.clone();
        let versions = self.version_match(ApiKey::Metadata, (4, 4));
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
    assert_eq!(meta.brokers[0].rack, MinVer(Some(None)));

    assert_eq!(meta.topics[0].name, "test");
    assert_eq!(meta.topics[0].error_code, 0);
    assert_eq!(meta.topics[0].parts[0].error_code, 0);
    dbg!(meta);
}