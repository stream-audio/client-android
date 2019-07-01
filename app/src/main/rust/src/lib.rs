#[macro_use]
mod macros;
mod android_audio;
mod android_helper;
mod error;
mod jni_ffi;
mod net_client;
mod player;
mod util;

use stream_audio_ffmpeg as ffmpeg;

pub fn rust_greeting(to: &str) -> String {
    format!("Hello {}. How are you?", to)
}
