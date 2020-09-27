use tokio_util::codec::{Framed, LengthDelimitedCodec, Encoder, Decoder};
use tokio::prelude::{AsyncRead, AsyncWrite};
use bytes::{Bytes, BytesMut, BufMut, Buf};
use tokio::macros::support::Pin;
use tokio_tower::multiplex::TagStore;
use tokio::stream::StreamExt;
use crate::proto::{Wired, RequestHeader, WireWrite, ApiKey, WireRead, ApiRequest};
use tokio::io::Error;
use std::ops::DerefMut;
use tower::{Service, ServiceExt};
use std::sync::Arc;
use tokio::sync::Mutex;
use std::future::Future;
use std::task::{Context, Poll};
use std::marker::PhantomData;
use futures::FutureExt;
use byteorder::{BigEndian, ByteOrder};
use crate::proto::api_versions::Request;

#[derive(Default, Clone)]
pub struct RawRequest {
    header: RequestHeader,
    data: Bytes,
}

pub struct RawResponse {
    corr_id: i32,
    data: Bytes,
}

pub struct Codec {}

impl Encoder<RawRequest> for Codec {
    type Error = std::io::Error;

    fn encode(&mut self, item: RawRequest, dst: &mut BytesMut) -> Result<(), Self::Error> {
        dst.put_i32(0);

        let mut wire = WireWrite {
            version: item.header.api_version as _,
            buffer: dst,
        };
        item.header.to_wire(&mut wire);
        dst.put(item.data);

        let len = dst.len();
        BigEndian::write_i32(dst.as_mut(), (len - 4) as i32);
        Ok(())
    }
}

impl Decoder for Codec {
    type Item = RawResponse;
    type Error = std::io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        println!("remaining: {:?}", src.len());
        if src.len() == 0 {
            return Ok(None);
        }
        let len = src.get_i32();
        let corr = src.get_i32();
        let data = src.split_to((len - 4) as usize);
        Ok(Some(RawResponse {
            corr_id: corr,
            data: data.into(),
        }))
    }
}

pub struct Tagger {
    counter: i32
}

impl tokio_tower::multiplex::TagStore<RawRequest, RawResponse> for Tagger {
    type Tag = i32;

    fn assign_tag(mut self: Pin<&mut Self>, r: &mut RawRequest) -> Self::Tag {
        self.deref_mut().counter += 1;
        r.header.correlation_id = self.counter;
        self.counter
    }

    fn finish_tag(self: Pin<&mut Self>, r: &RawResponse) -> Self::Tag {
        r.corr_id
    }
}


type MessageTransport<T> = Framed<T, Codec>;

type MultiplexTransport<T> = tokio_tower::multiplex::MultiplexTransport<
    MessageTransport<T>,
    Tagger
>;

type TowerError<T> = tokio_tower::Error<MultiplexTransport<T>, RawRequest>;

type RawClient<T> = tokio_tower::multiplex::Client<MultiplexTransport<T>, TowerError<T>, RawRequest>;


pub struct CallReq<Req> {
    api_ver: usize,
    req: Req,
}

impl<Req> CallReq<Req> {
    pub fn new(ver: usize, req: Req) -> CallReq<Req> {
        CallReq {
            api_ver: ver,
            req,
        }
    }
}

pub struct TypedClient<T>(RawClient<T>)
    where
        T: AsyncRead + AsyncWrite + 'static + Send;

impl<Req, T> tower::Service<CallReq<Req>> for TypedClient<T>
    where
        T: AsyncRead + AsyncWrite + 'static + Send,
        Req: ApiRequest + Send + 'static,
{
    type Response = Req::Response;
    type Error = TowerError<T>;
    type Future = Pin<Box<dyn Future<Output=Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.0.poll_ready(cx)
    }

    fn call(&mut self, req: CallReq<Req>) -> Self::Future {
        let mut buf = bytes::BytesMut::new();

        let mut wire = WireWrite {
            version: req.api_ver as _,
            buffer: &mut buf,
        };

        let encode = req.req.to_wire(&mut wire);
        let raw = RawRequest {
            header: RequestHeader {
                api_version: req.api_ver as _,
                api_key: Req::API_KEY,
                correlation_id: 0,
                client_id: Some("hello".to_string()),
            },
            data: buf.freeze(),
        };

        let fut = self.0.call(raw);
        async move {
            let mut res = fut.await.unwrap();

            Ok(Req::Response::from_wire(&mut WireRead {
                buffer: &mut res.data,
                version: req.api_ver as _,
            }).unwrap())
        }.boxed()
    }
}

pub async fn new<T>(io: T) -> TypedClient<T>
    where T: AsyncRead + AsyncWrite + Send + 'static
{
    let tagger = Tagger {
        counter: 1
    };

    let msg_transport = MessageTransport::new(io, Codec {});
    let t = MultiplexTransport::new(msg_transport, tagger);

    let client = TypedClient(RawClient::new(t));
    client
}

impl<T> tower::Service<()> for TypedClient<T>
    where
        T: AsyncRead + AsyncWrite + 'static + Send,
{
    type Response = ();
    type Error = TowerError<T>;
    type Future = futures::future::Ready<Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.0.poll_ready(cx)
    }

    fn call(&mut self, req: ()) -> Self::Future {
        futures::future::ok(())
    }
}
/*
#[tokio::test]
async fn test1() {
    std::env::set_var("RUST_LOG", "trace");
    let tp = tokio::net::TcpStream::connect("localhost:9092").await.unwrap();

    let client = new(tp).await;

    let mut cl = client.lock().await;
    let req = new_req::<Request>(1, crate::proto::api_versions::Request {});

    let rf = ServiceExt::<CallReq<Request>>::ready_and(cl.deref_mut());
    let f1 = rf.await.unwrap().call(req);

    let r1 = f1.await.unwrap();
}
 */