pub(crate) mod format;
pub(crate) mod fetch;
pub(crate) mod produce;
pub(crate) mod list_offsets;
pub(crate) mod api_versions;
pub(crate) mod metadata;
pub(crate) mod leader_isr;
pub(crate) mod offset_commit;
pub(crate) mod offset_fetch;
pub(crate) mod find_coordinator;
pub(crate) mod join_group;
pub(crate) mod heartbeat;
pub(crate) mod leave_group;
pub(crate) mod sync_group;
pub(crate) mod describe_groups;
pub(crate) mod list_groups;
pub(crate) mod create_partitions;
pub(crate) mod offset_delete;

use bytes::{BytesMut, Buf, Bytes, BufMut};

pub use format::*;
use std::ops::Shr;

pub trait ApiRequest: Wired {
    const API_KEY: ApiKey;
    const FLEXIBLE_VER: usize;
    type Response: Wired;
}

#[derive(Clone, Copy, Debug, Hash, Ord, PartialOrd, Eq, PartialEq)]
#[repr(i16)]
pub enum ApiKey {
    Produce = 0,
    Fetch = 1,
    ListOffsets = 2,
    Metadata = 3,
    LeaderAndIsr = 4,
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
    ReadCommited = 1,
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


#[allow(non_camel_case_types)]
pub struct vint(isize);

impl Wired for vint {
    fn to_wire(&self, wire: &mut WireWrite) {
        unimplemented!()
    }

    fn from_wire(wire: &mut WireRead) -> Result<Self, Error> {
        unimplemented!()
    }
}

//https://cwiki.apache.org/confluence/display/KAFKA/KIP-482%3A+The+Kafka+Protocol+should+Support+Optional+Tagged+Fields
#[allow(non_camel_case_types)]
pub struct uvint(usize);

impl Into<usize> for uvint {
    fn into(self) -> usize {
        self.0
    }
}

impl From<usize> for uvint {
    fn from(v: usize) -> Self {
        Self(v)
    }
}

impl Wired for uvint {
    fn to_wire(&self, wire: &mut WireWrite) {
        let mut val = self.0;
        loop {
            let mut c = (val & 0b01111111) as u8;
            val = val >> 7;
            c |= ((val > 0) as u8 & 0b1) << 7;
            println!("{:#b}", c);
            wire.buffer.put_u8(c);
            if val == 0 { break; }
        }
    }

    fn from_wire(wire: &mut WireRead) -> Result<Self, Error> {
        let mut res: usize = 0;
        let mut i = 0;
        loop {
            let b = wire.buffer.get_u8();
            res |= ((b & 0b01111111) as usize) << (i * 7);
            i += 1;
            if (b >> 7) == 0 { break; }
        }
        return Ok(Self(res));
    }
}

#[derive(Debug, Clone)]
pub struct TagBuffer {}

impl Default for TagBuffer {
    fn default() -> Self {
        Self {}
    }
}

impl Wired for TagBuffer {
    fn to_wire(&self, wire: &mut WireWrite) {
        uvint::from(0).to_wire(wire)
    }

    fn from_wire(wire: &mut WireRead) -> Result<Self, Error> {
        let len = uvint::from_wire(wire)?.0 as usize;
        let res = TagBuffer {};
        for i in 0..len {
            let elem_tag = uvint::from_wire(wire)?;
            let elem_len = uvint::from_wire(wire)?;
        }

        Ok(res)
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

// TODO: Add const param ` const TAG_SINCE: usize `
#[derive(Debug, Clone, Wired)]
pub struct TopicMap<T: Wired> {
    pub items: Vec<TopicItem<T>>,
    //pub _tag_buffer: MinVer<(), TAG_SINCE>,
}


