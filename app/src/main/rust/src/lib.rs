mod android_audio;
mod android_helper;
mod error;
#[allow(non_snake_case)]
mod jni_ffi_callbacks;
mod net_client;
mod player;

pub fn rust_greeting(to: &str) -> String {
    format!("Hello {}. How are you?", to)
}
