use crate::android_audio::{self, AudioPlayer, Engine, OutputMix};
use crate::error::Error;
use crate::jni_ffi::ToJavaMsg;
use crate::net_client::Pkt;
use crate::util::window_avg_calc::WindowAvgCalc;
use log::{info, warn};
use std::collections::VecDeque;
use std::sync::{mpsc, Arc, Mutex};
use std::time::{Duration, Instant};

#[derive(Clone)]
pub struct Player {
    player: Arc<Mutex<AudioPlayer>>,
    _mix: Arc<Mutex<OutputMix>>,
    _engine: Arc<Mutex<Engine>>,
    buffer: Arc<Mutex<OutputBuffer>>,
}

impl Player {
    pub fn new(to_java_send: mpsc::Sender<ToJavaMsg>) -> Result<Self, Error> {
        let engine = android_audio::Engine::new()?;
        engine.realize()?;

        let output_mix = engine.create_output_mix()?;
        output_mix.realize()?;

        let settings = android_audio::Settings {
            rate: android_audio::SampleRate::Rate44100,
            format: android_audio::SampleFormat::S16LE,
        };

        let player = engine.create_buffer_player(&output_mix, settings.clone())?;
        player.realize()?;

        Self::construct(to_java_send, settings, engine, output_mix, player)
    }

    pub fn start_playing(&self) -> Result<(), Error> {
        info!("Start playing");
        let player = self.player.lock().unwrap();
        player.set_play_state(android_audio::PlayState::Playing)
    }

    pub fn stop_playing(&self) -> Result<(), Error> {
        info!("Stop playing");
        let player = self.player.lock().unwrap();
        player.set_play_state(android_audio::PlayState::Stopped)
    }

    #[allow(dead_code)]
    pub fn is_playing(&self) -> Result<bool, Error> {
        let player = self.player.lock().unwrap();
        Ok(player.get_play_state()? == android_audio::PlayState::Playing)
    }

    pub fn get_delay(&self) -> Duration {
        let buffer = self.buffer.lock().unwrap();
        buffer.get_avg_delay()
    }

    pub fn enqueue(&self, pkt: &Pkt) {
        let mut buffer = self.buffer.lock().unwrap();

        let post_write_action = buffer.write(pkt);
        match post_write_action {
            PostWriteAction::Nothing => {}
            PostWriteAction::Read => {
                let mut buf = Vec::new();
                let is_success = buffer.read(&mut buf);
                if is_success {
                    let player = self.player.lock().unwrap();
                    let res = player.enqueue(&buf);
                    if let Err(e) = res {
                        warn!("Error enqueueing directly. {}", e);
                    }
                }
            }
        }
    }

    fn construct(
        to_java_send: mpsc::Sender<ToJavaMsg>,
        settings: android_audio::Settings,
        engine: Engine,
        mix: OutputMix,
        mut player: AudioPlayer,
    ) -> Result<Self, Error> {
        let buffer = Arc::new(Mutex::new(OutputBuffer::new(to_java_send, settings)));

        let cb_buffer = buffer.clone();
        player.register_callback(move |buf| -> Result<usize, Error> {
            let mut buffer = cb_buffer.lock()?;

            let is_success = buffer.read(buf);
            let n = if is_success { buf.len() } else { 0 };
            Ok(n)
        })?;

        Ok(Self {
            player: Arc::new(Mutex::new(player)),
            _mix: Arc::new(Mutex::new(mix)),
            _engine: Arc::new(Mutex::new(engine)),
            buffer,
        })
    }
}

impl Drop for Player {
    fn drop(&mut self) {
        let res = self.stop_playing();
        if let Err(e) = res {
            warn!("An error occurred while stopping in the drop. {}", e);
        }
    }
}

struct OutputBuffer {
    to_send: VecDeque<Frame>,
    free: Vec<Frame>,
    last_played: Option<Frame>,
    que_packets: bool,
    is_first_packet: bool,
    avg_to_send_len: WindowAvgCalc,
    to_java_send: mpsc::Sender<ToJavaMsg>,
    settings: android_audio::Settings,
}

struct Frame {
    data: Pkt<'static>,
    created: Instant,
}

#[must_use]
enum PostWriteAction {
    Nothing,
    Read,
}

const JITTER_BUFFER_LEN: usize = 10;
const AVG_OVER: usize = 25;

impl OutputBuffer {
    fn new(to_java_send: mpsc::Sender<ToJavaMsg>, settings: android_audio::Settings) -> Self {
        Self {
            to_send: VecDeque::new(),
            free: Vec::new(),
            last_played: None,
            que_packets: false,
            is_first_packet: true,
            avg_to_send_len: WindowAvgCalc::new(AVG_OVER).unwrap(),
            to_java_send,
            settings,
        }
    }

    fn write(&mut self, pkt: &Pkt) -> PostWriteAction {
        let block = if pkt.is_empty() {
            Frame::new_empty(pkt)
        } else {
            let mut block = match self.free.pop() {
                Some(block) => block,
                None => Frame::new(),
            };
            block.copy_from(pkt);
            block
        };

        self.to_send.push_back(block);
        self.choose_post_write_action()
    }

    fn read(&mut self, buf: &mut Vec<u8>) -> bool {
        if self.que_packets {
            return self.read_last_played_block(buf);
        }

        let block = match self.to_send.pop_front() {
            Some(block) => {
                if block.is_empty() {
                    return self.read_last_played_block(buf);
                } else {
                    block
                }
            }
            None => {
                info!("Nothing to read");
                self.que_packets = true;
                return self.read_last_played_block(buf);
            }
        };

        self.avg_to_send_len.push(block.elapsed());
        self.notify_java_with_new_avg_delay();

        let res = block.copy_to_vec(buf);

        if let Some(last_played) = self.last_played.take() {
            self.free.push(last_played);
        }

        self.last_played = Some(block);
        res
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

    fn get_avg_delay(&self) -> Duration {
        self.avg_to_send_len.get_avg()
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

    fn read_last_played_block(&mut self, buf: &mut Vec<u8>) -> bool {
        match &self.last_played {
            Some(block) => block.copy_to_vec(buf),
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

    fn new_empty(pkt: &Pkt) -> Self {
        assert!(pkt.is_empty());

        Self {
            data: Pkt::new_empty(pkt.cnt),
            created: Instant::now(),
        }
    }

    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    fn copy_from(&mut self, from: &Pkt) {
        self.data.copy_from(&from);
        self.created = Instant::now();
    }

    fn copy_to_vec(&self, buf: &mut Vec<u8>) -> bool {
        self.data.copy_to_vec(buf)
    }

    fn len(&self) -> usize {
        self.data.len()
    }

    fn elapsed(&self) -> Duration {
        self.created.elapsed()
    }
}
