use std::collections::HashMap;
use tower::{Service, ServiceExt};
use tokio::io::{AsyncWrite, AsyncRead};
use crate::transport;
use transport::CallReq;
use tokio::net::{ToSocketAddrs, TcpStream};
use std::future::Future;
use crate::proto::{MinVer, TopicMap, TopicItem, ApiKey, IsolationLevel};
use std::sync::Arc;
use tokio::sync::Mutex;
use futures::future::poll_fn;
use std::ops::DerefMut;

pub struct Client
{
    // TODO: make this generic
    pub(crate) client: Arc<Mutex<transport::TypedClient<TcpStream>>>,
    pub(crate) api_versions: HashMap<crate::proto::ApiKey, (usize, usize)>,
}


impl Client {
    pub async fn connect(addr: impl ToSocketAddrs) -> anyhow::Result<Client> {
        let io = TcpStream::connect(addr).await?;
        let mut client = transport::new(io).await;

        let request = crate::proto::api_versions::Request {};
        let req = CallReq::new(0, request);

        let ready = ServiceExt::<CallReq<crate::proto::api_versions::Request>>::ready_and(&mut client);
        let versions = ready.await?.call(req).await?;
        crate::res_from_code(versions.error_code).unwrap();
        let api_versions = versions.versions.into_iter().map(|v| {
            (v.api_key, (v.min_version as usize, v.max_version as usize))
        }).collect();

        Ok(Client {
            client: Arc::new(Mutex::new(client)),
            api_versions,
        })
    }

    pub fn server_versions(&self, key: ApiKey) -> (usize, usize) {
        self.api_versions.get(&key).cloned().unwrap()
    }

    pub fn version_match(&self, key: ApiKey, (c_min, c_max): (usize, usize)) -> (usize, usize) {
        let (s_min, s_max) = self.api_versions.get(&key).cloned().unwrap();
        (s_min.max(c_max), s_max.min(c_max))
    }

}

#[tokio::test]
async fn test_list_offsets() {
    let client = Client::connect("localhost:9092").await.unwrap();
    let offsets = client.list_offsets(vec![("test".to_string(), vec![0usize, 1])]).await.unwrap();

    assert_eq!(offsets.res.items[0].topic, "test");
    assert_eq!(offsets.res.items[0].value[0].partition, 0);
}