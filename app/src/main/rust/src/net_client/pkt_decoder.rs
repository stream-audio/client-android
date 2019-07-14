use crate::error::Error;
use log::{debug, warn};
use std::borrow::Cow;
use std::convert::TryInto;
use std::u32;

pub struct PktDecoder {
    prev_cnt: u32,
    lost_pkt_qty: u32,
}

pub struct Pkt<'a> {
    pub cnt: u32,
    pub data: Option<Cow<'a, [u8]>>,
}

pub struct PktIter<'a> {
    real_pkt: Option<Pkt<'a>>,
    empty_qty: u32,
    empty_cnt: u32,
}

impl PktDecoder {
    pub fn new() -> Self {
        Self {
            prev_cnt: 0,
            lost_pkt_qty: 0,
        }
    }

    pub fn parse<'a>(&mut self, buf: &'a [u8]) -> Result<PktIter<'a>, Error> {
        let (cnt_bytes, buf) = buf.split_at(std::mem::size_of::<u32>());
        let cnt = u32::from_be_bytes(cnt_bytes.try_into().unwrap());

        let pkt = Pkt::new_borrower(cnt, buf);

        if self.prev_cnt == 0 {
            self.prev_cnt = cnt;
            return Ok(PktIter::new(pkt, 0));
        }

        let mut lost_pkts = 0;
        if self.prev_cnt > cnt {
            warn!(
                "Out of order packet. New pkt cnt: {}, prev pkt cnt: {}",
                cnt, self.prev_cnt
            )
        } else if cnt - self.prev_cnt != 1 {
            lost_pkts = (cnt - self.prev_cnt) - 1;
            self.lost_pkt_qty += lost_pkts;
            warn!(
                "A packet is missing. New pkt cnt: {}, prev pkt cnt: {}. Total lost packets: {}",
                cnt, self.prev_cnt, self.lost_pkt_qty
            )
        }

        if cnt % 32 == 0 {
            debug!("Packet cnt: {}", cnt);
        }

        self.prev_cnt = cnt;
        Ok(PktIter::new(pkt, lost_pkts))
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

impl<'a> PktIter<'a> {
    fn new(pkt: Pkt<'a>, empty_qty: u32) -> Self {
        Self {
            real_pkt: Some(pkt),
            empty_qty,
            empty_cnt: 0,
        }
    }
}

impl<'a> Iterator for PktIter<'a> {
    type Item = Pkt<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.empty_cnt < self.empty_qty {
            let cnt = self.real_pkt.as_ref().unwrap().cnt;
            let cnt = cnt - self.empty_qty + self.empty_cnt;
            self.empty_cnt += 1;
            Some(Pkt::new_empty(cnt))
        } else {
            self.real_pkt.take()
        }
    }
}
