use crate::android_audio;
use crate::android_audio::{AudioPlayer, Engine, OutputMix};
use crate::error::Error;
use log::{info, warn};
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

pub struct Player {
    player: AudioPlayer,
    _mix: OutputMix,
    _engine: Engine,
    buffer: Arc<Mutex<OutputBuffer>>,
}

impl Player {
    pub fn new() -> Result<Self, Error> {
        let engine = android_audio::Engine::new()?;
        engine.realize()?;

        let output_mix = engine.create_output_mix()?;
        output_mix.realize()?;

        let settings = android_audio::Settings {
            rate: android_audio::SampleRate::Rate44100,
            format: android_audio::SampleFormat::S16LE,
        };

        let player = engine.create_buffer_player(&output_mix, settings)?;
        player.realize()?;

        Self::construct(engine, output_mix, player)
    }

    pub fn start_playing(&self) -> Result<(), Error> {
        info!("Start playing");
        self.player
            .set_play_state(android_audio::PlayState::Playing)
    }

    pub fn stop_playing(&self) -> Result<(), Error> {
        info!("Stop playing");
        self.player
            .set_play_state(android_audio::PlayState::Stopped)
    }

    #[allow(dead_code)]
    pub fn is_playing(&self) -> Result<bool, Error> {
        Ok(self.player.get_play_state()? == android_audio::PlayState::Playing)
    }

    pub fn enqueue(&self, buf: &[u8]) {
        let mut buffer = self.buffer.lock().unwrap();

        let post_write_action = buffer.write(buf);
        match post_write_action {
            PostWriteAction::Nothing => {}
            PostWriteAction::Read => {
                let mut buf = Vec::new();
                let is_success = buffer.read(&mut buf);
                if is_success {
                    let res = self.player.enqueue(&buf);
                    if let Err(e) = res {
                        warn!("Error enqueueing directly. {}", e);
                    }
                }
            }
        }
    }

    fn construct(engine: Engine, mix: OutputMix, player: AudioPlayer) -> Result<Self, Error> {
        let buffer = Arc::new(Mutex::new(OutputBuffer::new()));

        let mut res = Self {
            player,
            _mix: mix,
            _engine: engine,
            buffer,
        };

        let buffer = res.buffer.clone();
        res.player
            .register_callback(move |buf| -> Result<usize, Error> {
                let mut buffer = buffer.lock()?;

                let is_success = buffer.read(buf);
                let n = if is_success { buf.len() } else { 0 };
                Ok(n)
            })?;

        Ok(res)
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
    to_send: VecDeque<Vec<u8>>,
    free: Vec<Vec<u8>>,
    que_packets: bool,
    is_first_packet: bool,
}

#[must_use]
enum PostWriteAction {
    Nothing,
    Read,
}

const JITTER_BUFFER_LEN: usize = 10;

impl OutputBuffer {
    fn new() -> Self {
        Self {
            to_send: VecDeque::new(),
            free: Vec::new(),
            que_packets: false,
            is_first_packet: true,
        }
    }

    fn write(&mut self, buf: &[u8]) -> PostWriteAction {
        let mut block = match self.free.pop() {
            Some(block) => block,
            None => Vec::new(),
        };

        block.resize(buf.len(), 0);
        block.copy_from_slice(buf);
        self.to_send.push_back(block);

        if self.is_first_packet {
            info!("Got first packet");
            self.is_first_packet = false;
            PostWriteAction::Read
        } else if self.que_packets && self.to_send.len() >= JITTER_BUFFER_LEN {
            info!("Jitter buffer is full, start playing");
            self.que_packets = false;
            PostWriteAction::Read
        } else {
            PostWriteAction::Nothing
        }
    }

    fn read(&mut self, buf: &mut Vec<u8>) -> bool {
        if self.que_packets {
            return false;
        }

        let block = match self.to_send.pop_front() {
            Some(block) => block,
            None => {
                info!("Nothing to read");
                self.que_packets = true;
                return false;
            }
        };

        buf.resize(block.len(), 0);
        buf.copy_from_slice(&block);

        self.free.push(block);

        true
    }
}
