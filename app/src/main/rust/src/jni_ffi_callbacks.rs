use crate::android_helper;
use crate::error::{Error, ErrorRepr};
use crate::net_client;
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

struct RustObj {
    net_client: Option<net_client::NetClient>,
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

fn to_rust_obj_mut(ptr: i64) -> Result<&'static mut RustObj, Error> {
    unsafe { (ptr as usize as *mut RustObj).as_mut() }
        .ok_or_else(|| Error::new_null_ptr("object is null".to_owned()))
}

fn to_rust_obj_ref(ptr: i64) -> Result<&'static RustObj, Error> {
    unsafe { (ptr as usize as *mut RustObj).as_ref() }
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
pub extern "C" fn Java_com_willir_audiosharing_RustWrapper_createObjectNative(
    _env: JNIEnv,
    _: JClass,
) -> i64 {
    info!("createObject is called");
    println!("STDCOUT");

    let rust_obj = Box::new(RustObj::new());
    Box::into_raw(rust_obj) as i64
}

#[no_mangle]
pub extern "C" fn Java_com_willir_audiosharing_RustWrapper_destroyObjectNative(
    _env: JNIEnv,
    _: JClass,
    rust_obj: i64,
) {
    info!("destroyObject is called");
    if rust_obj == 0 {
        return;
    }
    let _ = unsafe { Box::from_raw(rust_obj as *mut RustObj) };
}

#[no_mangle]
pub extern "C" fn Java_com_willir_audiosharing_RustWrapper_playNative(
    env: JNIEnv,
    _: JClass,
    rust_obj: i64,
    remote_addr: JString,
) {
    info!("Play is called");

    let remote_addr: String = env.get_string(remote_addr).unwrap().into();

    let remote_addr: std::net::SocketAddr = throw_on_err!(
        remote_addr
            .parse()
            .map_err(|e| Error::new_net_parse(e, remote_addr)),
        env
    );

    let rust_obj: &mut RustObj = throw_on_err!(to_rust_obj_mut(rust_obj), env);

    rust_obj.net_client.take();

    let player = throw_on_err!(Player::new(), env);
    let net_client = throw_on_err!(
        net_client::NetClient::new(remote_addr, "0.0.0.0:25204".parse().unwrap(), player),
        env
    );

    rust_obj.net_client = Some(net_client);
}

#[no_mangle]
pub extern "C" fn Java_com_willir_audiosharing_RustWrapper_stopNative(
    env: JNIEnv,
    _: JClass,
    rust_obj: i64,
) {
    info!("Stop is called");

    let rust_obj: &mut RustObj = throw_on_err!(to_rust_obj_mut(rust_obj), env);

    if let Some(mut net_client) = rust_obj.net_client.take() {
        throw_on_err!(net_client.stop(), env);
    }
}

#[no_mangle]
pub extern "C" fn Java_com_willir_audiosharing_RustWrapper_isPlayingNative(
    env: JNIEnv,
    _: JClass,
    rust_obj: i64,
) -> bool {
    info!("isPlaying is called");

    let rust_obj: &RustObj = throw_on_err!(to_rust_obj_ref(rust_obj), env, false);

    rust_obj.net_client.is_some()
}

/*
#[no_mangle]
pub extern "C" fn Java_com_willir_audiosharing_RustWrapper_connectNative(
    env: JNIEnv,
    _: JClass,
    _player: i64,
    remote_addr: JString,
) {
    info!("connect is called");

    let remote_addr: String = env.get_string(remote_addr).unwrap().into();

    let remote_addr: std::net::SocketAddr = throw_on_err!(
        remote_addr
            .parse()
            .map_err(|e| Error::new_net_parse(e, remote_addr)),
        env
    );

    throw_on_err!(
        net_client::connect_to(remote_addr, "0.0.0.0:25204".parse().unwrap()),
        env
    );
}
*/

#[no_mangle]
pub extern "C" fn JNI_OnLoad(_vm: *mut JavaVM, _reserved: *mut c_void) -> i32 {
    android_helper::init_log();
    info!("JNI OnLoad");
    android_helper::redirect_stdcout();
    jni::JNIVersion::V6.into()
}

impl RustObj {
    fn new() -> Self {
        Self { net_client: None }
    }
}
