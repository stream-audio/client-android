use super::to_java::{java_callback_loop, ToJavaMsg};
use crate::android_helper;
use crate::error::{Error, ErrorRepr};
use crate::net_client;
use crate::player::Player;
use crate::rust_greeting;
use jni::objects::{JClass, JObject, JString};
use jni::sys::jstring;
use jni::{JNIEnv, JavaVM};
use log::{error, info, trace};
use std::ffi::c_void;
use std::mem::drop;
use std::sync::mpsc;
use std::thread::{self, JoinHandle};

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
    player: Option<Player>,
    java_cb_send: mpsc::Sender<ToJavaMsg>,
    java_cb_thread: Option<JoinHandle<()>>,
}

fn throw_java_exception(env: JNIEnv, e: &Error) {
    match e.repr.as_ref() {
        ErrorRepr::NullPointer(descr) => {
            env.throw_new("java/lang/NullPointerException", descr)
                .unwrap();
        }
        _ => {
            env.throw_new("java/lang/Exception", format!("{}", e))
                .unwrap();
        }
    }
}

extern "C" fn greeting(env: JNIEnv, _: JClass, java_pattern: JString) -> jstring {
    info!("greeting is called");

    let pattern: String = env
        .get_string(java_pattern)
        .expect("Invalid pattern string")
        .into();

    let world = rust_greeting(&pattern);

    let output = env.new_string(world).expect("Couldn't create java string!");
    output.into_inner()
}

extern "C" fn create_object(env: JNIEnv, _: JClass, cb: JObject) -> i64 {
    info!("createObject is called");
    println!("STDCOUT");

    let rust_obj = Box::new(throw_on_err!(RustObj::new(&env, cb), env, 0));
    RustObj::boxed_into_raw(rust_obj)
}

extern "C" fn destroy_object(_env: JNIEnv, _: JClass, rust_obj: i64) {
    info!("destroyObject is called");
    drop(RustObj::from_raw_box(rust_obj));
}

extern "C" fn play(env: JNIEnv, _: JClass, rust_obj: i64, remote_addr: JString) {
    info!("Play is called");

    let remote_addr: String = env.get_string(remote_addr).unwrap().into();

    let remote_addr: std::net::SocketAddr = throw_on_err!(
        remote_addr
            .parse()
            .map_err(|e| Error::new_net_parse(e, remote_addr)),
        env
    );

    let rust_obj: &mut RustObj = throw_on_err!(RustObj::from_raw_mut(rust_obj), env);

    rust_obj.net_client.take();

    let player = throw_on_err!(Player::new(rust_obj.java_cb_send.clone()), env);
    let net_client = throw_on_err!(
        net_client::NetClient::new(
            remote_addr,
            "0.0.0.0:25204".parse().unwrap(),
            player.clone(),
            rust_obj.java_cb_send.clone()
        ),
        env
    );

    rust_obj.player = Some(player);
    rust_obj.net_client = Some(net_client);
}

extern "C" fn stop(env: JNIEnv, _: JClass, rust_obj: i64) {
    info!("Stop is called");

    let rust_obj: &mut RustObj = throw_on_err!(RustObj::from_raw_mut(rust_obj), env);

    drop(rust_obj.player.take());
    if let Some(mut net_client) = rust_obj.net_client.take() {
        throw_on_err!(net_client.stop(), env);
    }
}

extern "C" fn is_playing(env: JNIEnv, _: JClass, rust_obj: i64) -> bool {
    let rust_obj: &RustObj = throw_on_err!(RustObj::from_raw_ref(rust_obj), env, false);

    rust_obj.net_client.is_some()
}

extern "C" fn get_delay_ms(env: JNIEnv, _: JClass, rust_obj: i64) -> i64 {
    let rust_obj: &RustObj = throw_on_err!(RustObj::from_raw_ref(rust_obj), env, 0);
    let player = match &rust_obj.player {
        Some(player) => player,
        None => {
            throw_java_exception(env, &Error::new_wrong_state("Player object is not created"));
            return 0;
        }
    };

    let delay = player.get_delay();
    delay.as_millis() as i64
}

extern "C" fn increase_delay(env: JNIEnv, _: JClass, rust_obj: i64) -> i64 {
    let rust_obj: &mut RustObj = throw_on_err!(RustObj::from_raw_mut(rust_obj), env, 0);
    let player = match &mut rust_obj.player {
        Some(player) => player,
        None => {
            throw_java_exception(env, &Error::new_wrong_state("Player object is not created"));
            return 0;
        }
    };

    let delay = player.increase_delay();
    delay.as_millis() as i64
}

extern "C" fn decrease_delay(env: JNIEnv, _: JClass, rust_obj: i64) -> i64 {
    let rust_obj: &mut RustObj = throw_on_err!(RustObj::from_raw_mut(rust_obj), env, 0);
    let player = match &mut rust_obj.player {
        Some(player) => player,
        None => {
            throw_java_exception(env, &Error::new_wrong_state("Player object is not created"));
            return 0;
        }
    };

    let delay = player.decrease_delay();
    delay.as_millis() as i64
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn JNI_OnLoad(vm: JavaVM, _reserved: *mut c_void) -> i32 {
    android_helper::init_log();
    info!("JNI OnLoad!!");
    android_helper::redirect_stdcout();

    let env = vm.get_env();
    let env = match env {
        Ok(env) => env,
        Err(e) => {
            error!("Error getting creating env from vm: {}", e);
            return jni::sys::JNI_ERR;
        }
    };

    let res = register_methods(env);
    if let Err(e) = res {
        error!("Error registering methods, {}", e);
        return jni::sys::JNI_ERR;
    }

    jni::JNIVersion::V6.into()
}

fn register_methods(env: JNIEnv) -> Result<(), Error> {
    let cls = env.find_class("com/streamaudio/client/service/rust/RustWrapper")?;

    let methods = [
        jni::sys::JNINativeMethod {
            name: b"greeting\0".as_ptr() as _,
            signature: b"(Ljava/lang/String;)Ljava/lang/String;\0".as_ptr() as _,
            fnPtr: greeting as *mut c_void,
        },
        jni::sys::JNINativeMethod {
            name: b"createObjectNative\0".as_ptr() as _,
            signature: b"(Lcom/streamaudio/client/service/rust/RustCb;)J\0".as_ptr() as _,
            fnPtr: create_object as *mut c_void,
        },
        jni::sys::JNINativeMethod {
            name: b"destroyObjectNative\0".as_ptr() as _,
            signature: b"(J)V\0".as_ptr() as _,
            fnPtr: destroy_object as *mut c_void,
        },
        jni::sys::JNINativeMethod {
            name: b"playNative\0".as_ptr() as _,
            signature: b"(JLjava/lang/String;)V\0".as_ptr() as _,
            fnPtr: play as *mut c_void,
        },
        jni::sys::JNINativeMethod {
            name: b"stopNative\0".as_ptr() as _,
            signature: b"(J)V\0".as_ptr() as _,
            fnPtr: stop as *mut c_void,
        },
        jni::sys::JNINativeMethod {
            name: b"isPlayingNative\0".as_ptr() as _,
            signature: b"(J)Z\0".as_ptr() as _,
            fnPtr: is_playing as *mut c_void,
        },
        jni::sys::JNINativeMethod {
            name: b"getDelayMsNative\0".as_ptr() as _,
            signature: b"(J)J\0".as_ptr() as _,
            fnPtr: get_delay_ms as *mut c_void,
        },
        jni::sys::JNINativeMethod {
            name: b"increaseDelayNative\0".as_ptr() as _,
            signature: b"(J)J\0".as_ptr() as _,
            fnPtr: increase_delay as *mut c_void,
        },
        jni::sys::JNINativeMethod {
            name: b"decreaseDelayNative\0".as_ptr() as _,
            signature: b"(J)J\0".as_ptr() as _,
            fnPtr: decrease_delay as *mut c_void,
        },
    ];

    let res = jni_non_void_call!(
        env.get_native_interface(),
        RegisterNatives,
        cls.into_inner(),
        methods.as_ptr(),
        methods.len() as _
    );

    if res != 0 {
        return Err(jni::errors::ErrorKind::Other(res).into());
    }

    Ok(())
}

impl RustObj {
    fn new(env: &JNIEnv, cb: JObject) -> Result<Self, Error> {
        let vm = env.get_java_vm()?;
        let cb = env.new_global_ref(cb)?;

        let (send, recv) = mpsc::channel();
        let thread = thread::spawn(move || java_callback_loop(vm, cb, recv));
        Ok(Self {
            net_client: None,
            player: None,
            java_cb_send: send,
            java_cb_thread: Some(thread),
        })
    }

    fn boxed_into_raw(r: Box<Self>) -> i64 {
        Box::into_raw(r) as i64
    }

    fn from_raw_ref(ptr: i64) -> Result<&'static RustObj, Error> {
        unsafe { (ptr as usize as *const RustObj).as_ref() }
            .ok_or_else(|| Error::new_null_ptr("object is null".to_owned()))
    }

    fn from_raw_mut(ptr: i64) -> Result<&'static mut RustObj, Error> {
        unsafe { (ptr as usize as *mut RustObj).as_mut() }
            .ok_or_else(|| Error::new_null_ptr("object is null".to_owned()))
    }

    fn from_raw_box(ptr: i64) -> Option<Box<RustObj>> {
        if ptr == 0 {
            None
        } else {
            Some(unsafe { Box::from_raw(ptr as *mut RustObj) })
        }
    }

    fn stop(&mut self) {
        log_and_ignore_err!(self.java_cb_send.send(ToJavaMsg::Stop));
        if let Some(thread) = self.java_cb_thread.take() {
            let res = thread.join();
            if let Err(_) = res {
                error!("Couldn't join java callback thread");
            }
        }
    }
}
impl Drop for RustObj {
    fn drop(&mut self) {
        self.stop();
    }
}
