use crate::proto::{Wired, WireRead, WireWrite, MinVer, TopicMap, IsolationLevel, ApiRequest, ApiKey, TopicItem};
use crate::client::Client;
use crate::transport::CallReq;

use std::future::Future;
use tower::{ServiceExt, Service};
use std::ops::DerefMut;


impl ApiRequest for ListOffsetsRequest {
    const API_KEY: ApiKey = ApiKey::ListOffsets;
    type Response = Response;
}

#[derive(Debug, Clone, Wired)]
pub struct ListOffsetsParts {
    pub(crate) partition: i32,
    pub(crate) current_leader_epoch: MinVer<i32, 4>,
    pub(crate) timestamp: i64,
    // Removed in ver1
    //max_num_offsets: i32,
}

#[derive(Debug, Clone, Wired)]
pub struct ListOffsetsRequest {
    pub(crate) replica_id: i32,
    pub(crate) isolation_level: MinVer<IsolationLevel, 2>,
    pub(crate) topics: TopicMap<ListOffsetsParts>,
}

#[derive(Debug, Clone, Wired)]
pub struct ListOffsetsResponseParts {
    pub partition: i32,
    pub error_code: i16,
    pub timestamp: MinVer<i64, 1>,
    pub offset: i64,
    pub leader_epoch: MinVer<i32, 4>,
}

#[derive(Debug, Clone, Wired)]
pub struct Response {
    pub throttle_time_ms: MinVer<i32, 2>,
    pub res: TopicMap<ListOffsetsResponseParts>,
}


impl Client {
    pub fn list_offsets(&self, topics: Vec<(String, Vec<usize>)>)
                        -> impl Future<Output=crate::Result<Response>>
    {
        let req = crate::proto::list_offsets::ListOffsetsRequest {
            replica_id: -1,
            isolation_level: IsolationLevel::ReadUncommited.into(),
            topics: TopicMap
            {
                items: topics.into_iter().map(|(topic, parts)| {
                    TopicItem {
                        topic: topic,
                        value: parts.into_iter().map(|part| {
                            crate::proto::list_offsets::ListOffsetsParts {
                                partition: part as _,
                                current_leader_epoch: None.into(),
                                timestamp: 0,
                            }
                        }).collect(),
                    }
                }).collect()
            },
        };

        let (min_ver, max_ver) = self.version_match(ApiKey::ListOffsets, (1, 1));
        let client = self.client.clone();

        async move {
            let mut client = client.lock().await;
            let client = ServiceExt::<()>::ready_oneshot(client.deref_mut()).await?;
            let res = client.call(CallReq::new(3 as _, req)).await?;
            Ok(res)
        }
    }
}