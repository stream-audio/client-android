mod output_buffer;

use self::output_buffer::{OutputBuffer, PostWriteAction};
use crate::android_audio::{self, AudioPlayer, Engine, OutputMix};
use crate::error::Error;
use crate::jni_ffi::ToJavaMsg;
use crate::net_client::Pkt;
use log::{info, warn};
use std::sync::{mpsc, Arc, Mutex};
use std::time::Duration;

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

    pub fn increase_delay(&mut self) -> Duration {
        let mut buffer = self.buffer.lock().unwrap();
        buffer.increase_delay()
    }

    pub fn decrease_delay(&mut self) -> Duration {
        let mut buffer = self.buffer.lock().unwrap();
        buffer.decrease_delay()
    }

    pub fn is_delay_fixed(&self) -> bool {
        let buffer = self.buffer.lock().unwrap();
        buffer.is_delay_fixed()
    }

    pub fn fix_delay_at(&mut self, delay: Duration) {
        let mut buffer = self.buffer.lock().unwrap();
        buffer.fix_delay_at(delay);
    }

    pub fn unfix_delay(&mut self) {
        let mut buffer = self.buffer.lock().unwrap();
        buffer.unfix_delay();
    }

    pub fn enqueue(&self, pkt: &Pkt) -> Result<(), Error> {
        let mut buffer = self.buffer.lock().unwrap();

        let post_write_action = buffer.write(pkt);
        match post_write_action {
            PostWriteAction::Nothing => {}
            PostWriteAction::Read => {
                let mut buf = Vec::new();
                info!("Reading from buffer right after writing");
                let is_success = buffer.read(&mut buf)?;
                if is_success {
                    let player = self.player.lock().unwrap();
                    player.enqueue(&buf)?;
                }
            }
        }

        Ok(())
    }

    fn on_read(output_buffer: &Arc<Mutex<OutputBuffer>>, to: &mut Vec<u8>) -> Result<usize, Error> {
        let mut output_buffer = output_buffer.lock()?;

        let is_success = output_buffer.read(to)?;
        let n = if is_success { to.len() } else { 0 };
        Ok(n)
    }

    fn construct(
        to_java_send: mpsc::Sender<ToJavaMsg>,
        settings: android_audio::Settings,
        engine: Engine,
        mix: OutputMix,
        mut player: AudioPlayer,
    ) -> Result<Self, Error> {
        let buffer = Arc::new(Mutex::new(OutputBuffer::new(to_java_send, settings)?));

        let cb_buffer = buffer.clone();
        player.register_callback(move |to| Player::on_read(&cb_buffer, to))?;

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
