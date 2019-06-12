use crate::android_helper;
use crate::error::{Error, ErrorRepr};
use crate::net_client;
use crate::player;
use crate::player::Player;
use crate::rust_greeting;
use jni::objects::{JClass, JString};
use jni::sys::jstring;
use jni::{JNIEnv, JavaVM};
use log::info;
use std::ffi::c_void;

macro_rules! throw_on_err {
    ($e: expr, $env: ident) => {{
        match $e {
            Ok(d) => d,
            Err(e) => {
                throw_java_exception($env, &e);
                return;
            }
        }
    }};
    ($e: expr, $env: ident, $def_res: expr) => {{
        match $e {
            Ok(d) => d,
            Err(e) => {
                throw_java_exception($env, &e);
                return $def_res;
            }
        }
    }};
}

fn throw_java_exception(env: JNIEnv, e: &Error) {
    match e.repr.as_ref() {
        ErrorRepr::NullPointer(descr) => {
            env.throw_new("java/lang/NullPointerException", descr)
                .unwrap();
            return;
        }
        _ => (),
    }
    env.throw_new("java/lang/Exception", format!("{}", e))
        .unwrap();
}

fn to_player_mut(ptr: i64) -> Result<&'static mut Player, Error> {
    unsafe { (ptr as usize as *mut Player).as_mut() }
        .ok_or_else(|| Error::new_null_ptr("player is null".to_owned()))
}

fn to_player_ref(ptr: i64) -> Result<&'static Player, Error> {
    unsafe { (ptr as usize as *mut Player).as_ref() }
        .ok_or_else(|| Error::new_null_ptr("player is null".to_owned()))
}

#[no_mangle]
pub extern "C" fn Java_com_willir_audiosharing_RustWrapper_greeting(
    env: JNIEnv,
    _: JClass,
    java_pattern: JString,
) -> jstring {
    info!("greeting is called");

    let pattern: String = env
        .get_string(java_pattern)
        .expect("Invalid pattern string")
        .into();

    let world = rust_greeting(&pattern);

    let output = env.new_string(world).expect("Couldn't create java string!");
    output.into_inner()
}

#[no_mangle]
pub extern "C" fn Java_com_willir_audiosharing_RustWrapper_createPlayerNative(
    env: JNIEnv,
    _: JClass,
) -> i64 {
    info!("create_player is called");

    let player: Player = throw_on_err!(player::create_player(), env, 0);
    let player = Box::new(player);

    Box::into_raw(player) as i64
}

#[no_mangle]
pub extern "C" fn Java_com_willir_audiosharing_RustWrapper_destroyPlayerNative(
    _env: JNIEnv,
    _: JClass,
    player: i64,
) {
    info!("destroy_player is called");
    if player == 0 {
        return;
    }
    let _ = unsafe { Box::from_raw(player as *mut Player) };
}

#[no_mangle]
pub extern "C" fn Java_com_willir_audiosharing_RustWrapper_playNative(
    env: JNIEnv,
    _: JClass,
    player: i64,
    f_path: JString,
) {
    info!("Play is called");
    println!("STDCOUT");

    let player = throw_on_err!(to_player_mut(player), env);
    let f_path: String = env.get_string(f_path).unwrap().into();

    throw_on_err!(player.play_from_wav_file(&f_path), env);
}

#[no_mangle]
pub extern "C" fn Java_com_willir_audiosharing_RustWrapper_stopNative(
    env: JNIEnv,
    _: JClass,
    player: i64,
) {
    info!("Stop is called");

    let player = throw_on_err!(to_player_ref(player), env);
    throw_on_err!(player.stop_playing(), env);
}

#[no_mangle]
pub extern "C" fn Java_com_willir_audiosharing_RustWrapper_isPlayingNative(
    env: JNIEnv,
    _: JClass,
    player: i64,
) -> bool {
    info!("isPlaying is called");

    let player = throw_on_err!(to_player_ref(player), env, false);

    throw_on_err!(player.is_playing(), env, false)
}

#[no_mangle]
pub extern "C" fn Java_com_willir_audiosharing_RustWrapper_connectNative(
    env: JNIEnv,
    _: JClass,
    _player: i64,
    remote_addr: JString,
) {
    info!("connect is called");

    let remote_addr: String = env.get_string(remote_addr).unwrap().into();

    throw_on_err!(
        net_client::connect_to(remote_addr.as_ref(), "0.0.0.0:25204"),
        env
    );
}

#[no_mangle]
pub extern "C" fn JNI_OnLoad(_vm: *mut JavaVM, _reserved: *mut c_void) -> i32 {
    android_helper::init_log();
    info!("JNI OnLoad");
    android_helper::redirect_stdcout();
    jni::JNIVersion::V6.into()
}
