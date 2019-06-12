use crate::android_audio;
use crate::android_audio::{AudioPlayer, Engine, OutputMix};
use crate::error::Error;
use log::info;
use std::fs;
use std::io::Read;

pub struct Player {
    player: AudioPlayer,
    _mix: OutputMix,
    _engine: Engine,
}

impl Player {
    fn new(engine: Engine, mix: OutputMix, player: AudioPlayer) -> Self {
        Self {
            player,
            _mix: mix,
            _engine: engine,
        }
    }

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
}

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

pub fn create_player() -> Result<Player, Error> {
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

    Ok(Player::new(engine, output_mix, player))
}

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
