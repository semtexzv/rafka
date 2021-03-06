use std::fmt::Display;
use bytes::{BufMut, BytesMut, Buf, Bytes};
use std::ops::{Deref, DerefMut};
use byteorder::{ByteOrder, BigEndian};
use crate::proto::{vint, uvint};

#[derive(Debug)]
pub struct Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unimplemented!()
    }
}

impl std::error::Error for Error {}

pub struct WireWrite<'a> {
    pub(crate) version: usize,
    pub(crate) buffer: &'a mut bytes::BytesMut,
}

pub struct WireRead<'a> {
    pub(crate) version: usize,
    pub(crate) buffer: &'a mut bytes::Bytes,
}

pub trait Wired: Sized {
    fn to_wire(&self, wire: &mut WireWrite);
    fn from_wire(wire: &mut WireRead) -> Result<Self, Error>;

    fn to_wire_compact(&self, wire: &mut WireWrite) {
        self.to_wire(wire);
    }
    fn from_wire_compact(wire: &mut WireRead) -> Result<Self, Error> {
        Self::from_wire(wire)
    }
}

impl Wired for () {
    fn to_wire(&self, wire: &mut WireWrite) {}

    fn from_wire(wire: &mut WireRead) -> Result<Self, Error> {
        Ok(())
    }
}

impl<T: Wired> Wired for Vec<T> {
    fn to_wire(&self, wire: &mut WireWrite) {
        (self.len() as i32).to_wire(wire);
        for item in self {
            item.to_wire(wire);
        }
    }

    fn from_wire(wire: &mut WireRead) -> Result<Self, Error> {
        let len = wire.buffer.get_i32();
        let mut res = Vec::with_capacity(len as _);
        for i in 0..len {
            res.push(T::from_wire(wire)?);
        }
        Ok(res)
    }

    fn to_wire_compact(&self, wire: &mut WireWrite) {
        uvint::from(self.len() + 1).to_wire(wire);
        for item in self {
            item.to_wire_compact(wire);
        }
    }

    fn from_wire_compact(wire: &mut WireRead) -> Result<Self, Error> {
        let len: usize = uvint::from_wire(wire)?.into();
        let len = len - 1;
        let mut res = Vec::with_capacity(len as _);
        for item in 0..len {
            res.push(T::from_wire_compact(wire)?);
        }
        Ok(res)
    }
}

impl Wired for String {
    fn to_wire(&self, wire: &mut WireWrite) {
        (self.len() as i16).to_wire(wire);
        wire.buffer.put(self.as_bytes())
    }

    fn from_wire(wire: &mut WireRead) -> Result<Self, Error> {
        let len = wire.buffer.get_i16();
        let data = wire.buffer.split_to(len as _);
        Ok(String::from_utf8(data.bytes().to_vec()).unwrap())
    }

    fn to_wire_compact(&self, wire: &mut WireWrite) {
        uvint::from(self.len() + 1).to_wire(wire);
        wire.buffer.put(self.as_bytes());
    }

    fn from_wire_compact(wire: &mut WireRead) -> Result<Self, Error> {
        let len: usize = uvint::from_wire(wire)?.into();
        let data = wire.buffer.split_to(len - 1usize);
        Ok(String::from_utf8(data.bytes().to_vec()).unwrap())
    }
}

impl Wired for Option<String> {
    fn to_wire(&self, wire: &mut WireWrite) {
        match self {
            None => {
                (-1 as i16).to_wire(wire);
            }
            Some(v) => {
                v.to_wire(wire);
            }
        }
    }

    fn from_wire(wire: &mut WireRead) -> Result<Self, Error> {
        let len = wire.buffer.get_i16();
        if len < 0 {
            return Ok(None);
        }
        let data = wire.buffer.split_to(len as _);
        Ok(Some(String::from_utf8(data.to_vec()).unwrap()))
    }

    fn to_wire_compact(&self, wire: &mut WireWrite) {
        match self {
            None => {
                (uvint::from(0)).to_wire(wire);
            }
            Some(v) => {
                v.to_wire_compact(wire);
            }
        }
    }

    fn from_wire_compact(wire: &mut WireRead) -> Result<Self, Error> {
        let len: usize = uvint::from_wire(wire)?.into();
        if len == 0 {
            return Ok(None);
        }
        let data = wire.buffer.split_to(len - 1);
        Ok(Some(String::from_utf8(data.bytes().to_vec()).unwrap()))
    }
}

impl Wired for Bytes {
    fn to_wire(&self, wire: &mut WireWrite) {
        (self.len() as i32).to_wire(wire);
        wire.buffer.put(self.bytes());
    }

    fn from_wire(wire: &mut WireRead) -> Result<Self, Error> {
        let len = wire.buffer.get_i32();
        let data = wire.buffer.split_to(len as _);
        Ok(data)
    }

    fn to_wire_compact(&self, wire: &mut WireWrite) {
        uvint::from(self.len() + 1).to_wire(wire);
        wire.buffer.put(self.bytes());
    }

    fn from_wire_compact(wire: &mut WireRead) -> Result<Self, Error> {
        let len: usize = uvint::from_wire(wire)?.into();

        let data = wire.buffer.split_to(len - 1usize);
        Ok(data)
    }
}

// TODO: Option<bytes> for NULLABLE_BYTES
impl Wired for bool {
    #[inline(always)]
    fn to_wire(&self, wire: &mut WireWrite) {
        wire.buffer.put_i8(if *self { 1 } else { 0 })
    }
    #[inline(always)]
    fn from_wire(wire: &mut WireRead) -> Result<Self, Error> {
        Ok(wire.buffer.get_i8() != 0)
    }
}

impl Wired for i8 {
    fn to_wire(&self, wire: &mut WireWrite) {
        wire.buffer.put_i8(*self);
    }

    fn from_wire(wire: &mut WireRead) -> Result<Self, Error> {
        Ok(wire.buffer.get_i8())
    }
}

impl Wired for i16 {
    fn to_wire(&self, wire: &mut WireWrite) {
        wire.buffer.put_i16(*self);
    }

    fn from_wire(wire: &mut WireRead) -> Result<Self, Error> {
        Ok(wire.buffer.get_i16())
    }
}

impl Wired for i32 {
    fn to_wire(&self, wire: &mut WireWrite) {
        wire.buffer.put_i32(*self);
    }

    fn from_wire(wire: &mut WireRead) -> Result<Self, Error> {
        Ok(wire.buffer.get_i32())
    }
}

impl Wired for i64 {
    fn to_wire(&self, wire: &mut WireWrite) {
        wire.buffer.put_i64(*self);
    }

    fn from_wire(wire: &mut WireRead) -> Result<Self, Error> {
        Ok(wire.buffer.get_i64())
    }
}

