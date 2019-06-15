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
        /*
                if buffer.que_packets {
                    info!("enqueueing directly {}", buf.len());
                    let res = self.player.enqueue(buf);
                    match res {
                        Ok(()) => {
                            buffer.que_packets = false;
                        }
                        Err(e) => {
                            eprintln!("{}", e);
                        }
                    }
                } else {
                    buffer.write(buf);
                }
        */
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

    /*
        pub fn play_from_wav_file(&mut self, f_path: &str) -> Result<(), Error> {
            info!("play_from_wav_file({})", f_path);

            if self.player.get_play_state()? == android_audio::PlayState::Playing {
                return Err(Error::new_wrong_state(
                    "Player is already playing a file, you should stop the current playing first"
                        .to_owned(),
                ));
            }

            let mut f = FromFilePlayer::new(f_path)?;
            let first_block = f.read_next()?;

            self.player
                .register_callback(move |buf| f.enqueue_next(buf))?;
            self.player
                .set_play_state(android_audio::PlayState::Playing)?;
            self.player.enqueue(&first_block)?;

            info!("After player created");

            Ok(())
        }

        pub fn stop_playing(&self) -> Result<(), Error> {
            self.player
                .set_play_state(android_audio::PlayState::Stopped)
        }

        pub fn is_playing(&self) -> Result<bool, Error> {
            Ok(self.player.get_play_state()? == android_audio::PlayState::Playing)
        }
    */
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

const JITTER_BUFFER_LEN: usize = 25;

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

/*
struct FromFilePlayer {
    f: fs::File,
    f_path: String,
}

impl FromFilePlayer {
    fn new(f_path: &str) -> Result<FromFilePlayer, Error> {
        let f = fs::File::open(f_path).map_err(|e| Error::new_io(e, f_path.to_owned()))?;

        let mut res = Self {
            f,
            f_path: f_path.to_owned(),
        };

        res.skip_header()?;

        Ok(res)
    }

    fn enqueue_next(&mut self, buf: &mut Vec<u8>) -> Result<(usize), Error> {
        let n = self.f.read(buf)?;
        Ok(n)
    }

    fn read_next(&mut self) -> Result<Vec<u8>, Error> {
        let mut res = vec![0; 1024];
        let n = self.f.read(&mut res)?;
        res.resize(n, 0);

        Ok(res)
    }

    fn skip_header(&mut self) -> Result<(), Error> {
        let mut buf = [0; 44];
        let n = self
            .f
            .read(&mut buf)
            .map_err(|e| Error::new_io(e, self.f_path.clone()))?;
        if n != 44 {
            return Err(Error::new_wrong_argument(format!(
                "File {} is too short for a WAV file",
                self.f_path
            )));
        }

        if &buf[..4] != b"RIFF" || &buf[8..12] != b"WAVE" {
            return Err(Error::new_wrong_argument(format!(
                "File {} is not a WAV file. The tag doesn't match.",
                self.f_path
            )));
        } else if &buf[36..40] != b"data" {
            return Err(Error::new_wrong_argument(format!(
                "File {} is not a WAV file. Cannot find the data section.",
                self.f_path
            )));
        }

        Ok(())
    }
}
*/

/*
fn thread_loop(
    _engine: android_audio::Engine,
    _mix: android_audio::OutputMix,
    player: android_audio::AudioPlayer,
) {
    info!("Inside loop");
    thread::sleep(time::Duration::from_secs(15));
    info!("After sleep");
    let _ = player.set_play_state(android_audio::PlayState::Stopped);
}
*/
/*
pub fn play(file: String) -> Result<(), Error> {
    info!("play({})", file);

    info!("play path");

    let mut f = FromFilePlayer::new(file)?;

    info!("Open file");

    let engine = android_audio::Engine::new()?;
    engine.realize()?;

    info!("After engine realize");

    let output_mix = engine.create_output_mix()?;
    output_mix.realize()?;

    let settings = android_audio::Settings {
        rate: android_audio::SampleRate::Rate44100,
        format: android_audio::SampleFormat::S16LE,
    };

    let mut player = engine.create_buffer_player(&output_mix, settings)?;
    player.realize()?;

    let first_block = f.read_next()?;

    player.register_callback(move |buf| f.enqueue_next(buf))?;
    player.set_play_state(android_audio::PlayState::Playing)?;
    player.enqueue(&first_block)?;

    info!("After player created");

    thread::spawn(move || {
        thread_loop(engine, output_mix, player);
    });

    Ok(())
}
*/
