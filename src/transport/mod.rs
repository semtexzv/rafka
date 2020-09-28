use tokio_util::codec::{Framed, LengthDelimitedCodec, Encoder, Decoder};
use tokio::prelude::{AsyncRead, AsyncWrite};
use bytes::{Bytes, BytesMut, BufMut, Buf};
use tokio::macros::support::Pin;
use tokio_tower::multiplex::TagStore;
use tokio::stream::StreamExt;
use crate::proto::{Wired, WireWrite, ApiKey, WireRead, ApiRequest, TagBuffer};
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


#[derive(Default, Clone, Wired)]
pub struct RequestHeader {
    pub(crate) api_key: ApiKey,
    pub(crate) api_version: i16,
    pub(crate) correlation_id: i32,
    #[wired(since = 1)]
    pub(crate) client_id: Option<String>,
    // Request Header contains tag buffer in flexible versions
    #[wired(since = 2)]
    pub(crate) tag_buffer: Option<TagBuffer>,
}

#[derive(Debug, Clone, Wired)]
pub struct ResponseHeader {
    pub(crate) correlation_id: i32,
}

#[derive(Default, Clone)]
pub struct RawRequest {
    header: RequestHeader,
    flexible: bool,
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
            version: if item.flexible { 2 } else { 1 },
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
        let ver = req.api_ver as usize;

        let mut wire = WireWrite {
            version: req.api_ver as _,
            buffer: &mut buf,
        };
        let flexible = req.api_ver as usize >= Req::FLEXIBLE_VER;
        if flexible {
            req.req.to_wire_compact(&mut wire);
        } else {
            req.req.to_wire(&mut wire);
        }
        let raw = RawRequest {
            header: RequestHeader {
                api_version: req.api_ver as _,
                api_key: Req::API_KEY,
                correlation_id: 0,
                client_id: Some("hello".to_string()),
                tag_buffer: TagBuffer {}.into(),
            },
            flexible,
            data: buf.freeze(),
        };

        let fut = self.0.call(raw);
        async move {
            let mut res = fut.await.unwrap();

            let mut read = WireRead {
                buffer: &mut res.data,
                version: req.api_ver as _,
            };

            Ok(if flexible {
                TagBuffer::from_wire(&mut read).unwrap();
                Req::Response::from_wire_compact(&mut read).unwrap()
            } else {
                Req::Response::from_wire(&mut read).unwrap()
            })
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
