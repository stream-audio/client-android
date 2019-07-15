use crate::error::Error;
use std::borrow::Cow;
use std::convert::TryInto;
use std::u32;

pub struct PktDecoder {}

pub struct Pkt<'a> {
    pub cnt: u32,
    pub data: Option<Cow<'a, [u8]>>,
}

impl PktDecoder {
    pub fn new() -> Self {
        Self {}
    }

    pub fn parse<'a>(&mut self, buf: &'a [u8]) -> Result<Pkt<'a>, Error> {
        let (cnt_bytes, buf) = buf.split_at(std::mem::size_of::<u32>());
        let cnt = u32::from_be_bytes(cnt_bytes.try_into().unwrap());

        let pkt = Pkt::new_borrower(cnt, buf);
        Ok(pkt)
    }
}

impl<'a> Pkt<'a> {
    pub fn new_empty(cnt: u32) -> Self {
        Self { cnt, data: None }
    }

    pub fn new_owner(cnt: u32) -> Self {
        Self {
            cnt,
            data: Some(Vec::new().into()),
        }
    }

    pub fn new_borrower(cnt: u32, data: &'a [u8]) -> Self {
        Self {
            cnt,
            data: Some(data.into()),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_none()
    }

    pub fn len(&self) -> usize {
        self.data.as_ref().map(|d| d.len()).unwrap_or(0)
    }

    pub fn copy_from(&mut self, from: &Pkt) {
        let from_data = match &from.data {
            None => {
                return;
            }
            Some(data) => data,
        };

        let data = self.data.get_or_insert(Vec::new().into());
        let data: &mut Vec<u8> = data.to_mut();
        data.resize(from_data.len(), 0);
        data.copy_from_slice(from_data);
        self.cnt = from.cnt;
    }

    pub fn copy_to_vec(&self, to: &mut Vec<u8>) -> bool {
        let data = match &self.data {
            None => {
                return false;
            }
            Some(data) => data,
        };

        to.resize(data.len(), 0);
        to.copy_from_slice(&data);
        true
    }
}
