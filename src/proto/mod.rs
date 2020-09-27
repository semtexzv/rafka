pub(crate) mod format;
pub(crate) mod fetch;
pub(crate) mod produce;
pub(crate) mod list_offsets;
pub(crate) mod api_versions;
pub(crate) mod metadata;

use bytes::{BytesMut, Buf, Bytes};

pub use format::*;

pub trait ApiRequest: Wired {
    const API_KEY: ApiKey;
    type Response: Wired;
}

#[derive(Clone, Copy, Debug, Hash, Ord, PartialOrd, Eq, PartialEq)]
#[repr(i16)]
pub enum ApiKey {
    Produce = 0,
    Fetch = 1,
    ListOffsets = 2,
    Metadata = 3,
    LeaderAndlsr = 4,
    StopReplica = 5,
    UpdateMetadata = 6,
    ControlledShutdown = 7,
    OffsetCommit,
    OffsetFetch,
    FindCoordinator,
    JoinGroup,
    Hearbeat,
    LeaveGroup,
    SyncGroup,
    DescribeGroups,
    ListGroups,
    SaslHandshake,
    ApiVersions,
    CreateTopic,
    DeleteTopics,
    DeleteRecords,
    InitProducerId,
    OffsetForLeaderEpoch,
    AddPartitionsToTxn,
    AddOffsetsToTxn,
    EndTxn,
    WritneTxnMarkers,
    TxnOffsetCommit,
    DescribeAcls,
    CreateAcls,
    DeleteAcs,
    DescribeConfigs,
    AlterConfigs,
    AlterReplicaLogDirs,
    DescriveLogDirs,
    SaslAuthenticate,
    CreatePartitions,
    CreateDelegationToken,
    RenewDelegationToken,
    ExpireDelegationToken,
    DescribeDelegationToken,
    DeleteGroups,
    ElectLeaders,
    IncrementalAlterConfigs,
    AlterPartitionReassignments,
    ListPartitionReassignments,
    OffsetDelete,
    DescribeClientQuotas,
    AlterClientQuotas = 49,
}

impl Default for ApiKey {
    fn default() -> Self {
        return ApiKey::Produce;
    }
}

impl Wired for ApiKey {
    fn to_wire(&self, wire: &mut WireWrite) {
        (unsafe { std::mem::transmute::<_, i16>(*self) }).to_wire(wire)
    }

    fn from_wire(wire: &mut WireRead) -> Result<Self, Error> {
        let v = i16::from_wire(wire)?;
        Ok(unsafe { std::mem::transmute::<_, Self>(v) })
    }
}

#[derive(Clone, Copy, Debug)]
#[repr(i8)]
pub enum IsolationLevel {
    ReadUncommited = 0,
    ReadCommited,
}

impl Wired for IsolationLevel {
    fn to_wire(&self, wire: &mut WireWrite) {
        (unsafe { std::mem::transmute::<_, i8>(*self) }).to_wire(wire)
    }

    fn from_wire(wire: &mut WireRead) -> Result<Self, Error> {
        let v = i8::from_wire(wire)?;
        Ok(unsafe { std::mem::transmute::<_, Self>(v) })
    }
}

#[derive(Default, Clone, Wired)]
pub struct RequestHeader {
    pub(crate) api_key: ApiKey,
    pub(crate) api_version: i16,
    pub(crate) correlation_id: i32,
    pub(crate) client_id: Option<String>,
}

#[derive(Debug, Clone, Wired)]
pub struct ResponseHeader {
    pub(crate) correlation_id: i32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MinVer<T: Wired, const VER: usize>(pub Option<T>);

impl<T: Wired, const VER: usize> Wired for MinVer<T, VER> {
    fn to_wire(&self, wire: &mut WireWrite) {
        if wire.version >= VER {
            self.0.as_ref().expect("Need to provide value for this version").to_wire(wire);
        }
    }

    fn from_wire(wire: &mut WireRead) -> Result<Self, Error> {
        if wire.version >= VER {
            return Ok(Self(Some(T::from_wire(wire)?)));
        }
        Ok(Self(None))
    }
}

impl<T: Wired, const VER: usize> From<Option<T>> for MinVer<T, VER> {
    fn from(v: Option<T>) -> Self {
        Self(v)
    }
}

impl<T: Wired, const VER: usize> From<T> for MinVer<T, VER> {
    fn from(v: T) -> Self {
        Self(Some(v))
    }
}

#[allow(non_camel_case_types)]
pub struct vint(i32);

impl Wired for vint {
    fn to_wire(&self, wire: &mut WireWrite) {
        unimplemented!()
    }

    fn from_wire(wire: &mut WireRead) -> Result<Self, Error> {
        unimplemented!()
    }
}

#[derive(Wired)]
pub struct RecordHeader {
    key: String,
    value: Bytes,
}

#[derive(Wired)]
pub struct Record {
    len: vint,
    attrs: i8,
    timestamp_delta: vint,
    offset_delta: vint,
    key: Bytes,
    value: Bytes,
    headers: Vec<String>,
}

#[derive(Wired)]
pub struct RecordBatch {
    first_offset: i64,
    len: i32,
    part_leader_epoch: i32,
    magic: i8,
    crc: i32,
    attrs: i16,
    last_offset_delta: i32,
    first_timestamp: i64,
    max_timestamp: i64,
    producer_id: i64,
    producer_epoch: i16,
    first_sequence: i32,
    records: Vec<Record>,
}

#[derive(Debug, Clone, Wired)]
pub struct TopicItem<T: Wired> {
    pub topic: String,
    pub value: Vec<T>,
}

#[derive(Debug, Clone, Wired)]
pub struct TopicMap<T: Wired> {
    pub items: Vec<TopicItem<T>>
}
