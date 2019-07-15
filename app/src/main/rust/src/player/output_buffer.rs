use crate::android_audio;
use crate::jni_ffi::ToJavaMsg;
use crate::net_client::Pkt;
use crate::util::window_avg_calc::WindowAvgCalc;
use log::{info, warn};
use std::collections::VecDeque;
use std::sync::mpsc;
use std::time::{Duration, Instant};

pub struct OutputBuffer {
    to_send: VecDeque<Frame>,
    free: Vec<Frame>,
    last_played: Option<Frame>,
    que_packets: bool,
    is_first_packet: bool,
    avg_to_send_len: WindowAvgCalc,
    to_java_send: mpsc::Sender<ToJavaMsg>,
    settings: android_audio::Settings,
    total_missing: usize,
}

struct Frame {
    data: Pkt<'static>,
    created: Instant,
}

#[must_use]
pub enum PostWriteAction {
    Nothing,
    Read,
}

const JITTER_BUFFER_LEN: usize = 3;
const AVG_OVER: usize = 25;

impl OutputBuffer {
    pub fn new(to_java_send: mpsc::Sender<ToJavaMsg>, settings: android_audio::Settings) -> Self {
        Self {
            to_send: VecDeque::new(),
            free: Vec::new(),
            last_played: None,
            que_packets: false,
            is_first_packet: true,
            avg_to_send_len: WindowAvgCalc::new(AVG_OVER).unwrap(),
            to_java_send,
            settings,
            total_missing: 0,
        }
    }

    pub fn write(&mut self, pkt: &Pkt) -> PostWriteAction {
        let block = if pkt.is_empty() {
            Frame::new_empty(pkt.cnt)
        } else {
            let mut block = match self.free.pop() {
                Some(block) => block,
                None => Frame::new(),
            };
            block.copy_from_pkt(pkt);
            block
        };

        self.add_block(block);
        self.choose_post_write_action()
    }

    pub fn read(&mut self, to: &mut Vec<u8>) -> bool {
        if self.que_packets {
            return self.read_last_played_block(to);
        }

        let block = match self.to_send.pop_front() {
            Some(block) => {
                if block.is_empty() {
                    return self.read_empty_block(&block, to);
                } else {
                    block
                }
            }
            None => {
                info!("Nothing to read");
                self.que_packets = true;
                return self.read_last_played_block(to);
            }
        };

        self.avg_to_send_len.push(block.elapsed());
        self.notify_java_with_new_avg_delay();

        let res = block.copy_to_vec(to);

        if let Some(last_played) = self.last_played.take() {
            self.free.push(last_played);
        }

        self.last_played = Some(block);
        res
    }

    pub fn get_avg_delay(&self) -> Duration {
        self.avg_to_send_len.get_avg()
    }

    fn add_block(&mut self, block: Frame) {
        if self.to_send.is_empty() {
            self.to_send.push_back(block);
            return;
        }

        let new_cnt = block.data.cnt;
        let first_cnt = self.to_send.front().unwrap().data.cnt;
        let last_cnt = self.to_send.back().unwrap().data.cnt;

        if last_cnt < new_cnt {
            self.append_block(block, last_cnt);
            return;
        } else if new_cnt < first_cnt {
            return;
        } else if block.is_empty() {
            return;
        }

        let idx = (new_cnt - first_cnt) as usize;
        if self.to_send[idx].is_empty() {
            self.to_send[idx] = block;
        }
    }

    fn append_block(&mut self, block: Frame, last_cnt: u32) {
        let new_cnt = block.data.cnt;
        for cnt in (last_cnt + 1)..new_cnt {
            self.to_send.push_back(Frame::new_empty(cnt))
        }
        self.to_send.push_back(block);
    }

    fn choose_post_write_action(&mut self) -> PostWriteAction {
        if self.is_first_packet {
            info!("Got first packet");
            self.is_first_packet = false;
            PostWriteAction::Read
        } else if self.que_packets && self.to_send.len() >= JITTER_BUFFER_LEN {
            info!("Jitter buffer is full, start playing");
            self.que_packets = false;
            PostWriteAction::Nothing
        } else {
            PostWriteAction::Nothing
        }
    }

    fn notify_java_with_new_avg_delay(&self) {
        log_and_ignore_err!(self
            .to_java_send
            .send(ToJavaMsg::BufferSizeChanged(self.avg_to_send_len.get_avg())));
    }

    #[allow(dead_code)]
    fn get_pkt_duration(&self) -> Duration {
        if let Some(pkt) = &self.last_played {
            self.calc_pkt_duration(pkt.len())
        } else {
            Duration::from_millis(23)
        }
    }

    fn calc_pkt_duration(&self, bytes: usize) -> Duration {
        let samples = bytes / self.settings.format.get_sample_size() / 2;
        let rate = self.settings.rate.to_hz();

        let micros = (samples as f64 / rate as f64) * 1000000.;
        assert!(15000. <= micros && micros <= 43000.);

        Duration::from_micros(micros as u64)
    }

    fn read_empty_block(&mut self, block: &Frame, to: &mut Vec<u8>) -> bool {
        self.total_missing += 1;
        warn!(
            "Block {} is missing. Total missing: {}",
            block.get_cnt(),
            self.total_missing
        );
        return self.read_last_played_block(to);
    }

    fn read_last_played_block(&mut self, to: &mut Vec<u8>) -> bool {
        match &self.last_played {
            Some(block) => block.copy_to_vec(to),
            None => {
                warn!("No Last Packet");
                false
            }
        }
    }
}

impl Frame {
    fn new() -> Self {
        Self {
            data: Pkt::new_owner(0),
            created: Instant::now(),
        }
    }

    fn new_empty(cnt: u32) -> Self {
        Self {
            data: Pkt::new_empty(cnt),
            created: Instant::now(),
        }
    }

    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    fn get_cnt(&self) -> u32 {
        self.data.cnt
    }

    fn copy_from_pkt(&mut self, from: &Pkt) {
        self.data.copy_from(&from);
        self.created = Instant::now();
    }

    fn copy_to_vec(&self, to: &mut Vec<u8>) -> bool {
        self.data.copy_to_vec(to)
    }

    fn len(&self) -> usize {
        self.data.len()
    }

    fn elapsed(&self) -> Duration {
        self.created.elapsed()
    }
}
