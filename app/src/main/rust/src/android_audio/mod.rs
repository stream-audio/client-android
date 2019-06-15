#[allow(dead_code, unused_attributes, bad_style)]
mod audio_ffi;
#[allow(dead_code, unused_attributes, bad_style)]
mod audio_ffi_defines;

use crate::error::Error;
use audio_ffi as a_ffi;
use audio_ffi::SLuint32;
use audio_ffi_defines::*;
use log::{error, info};
use std::cell::Cell;
use std::ffi::c_void;
use std::fmt;
use std::mem;
use std::ptr;

#[link(name = "OpenSLES")]
extern "C" {}

#[derive(Debug)]
pub struct SlError {
    repr: SlErrorRepr,
}

#[derive(Debug)]
enum SlErrorRepr {
    Sl(a_ffi::SLresult, String),
    UnknownMethod(String),
}

impl SlError {
    fn new_sl(err_code: a_ffi::SLresult, context: String) -> Self {
        Self {
            repr: SlErrorRepr::Sl(err_code, context),
        }
    }
    fn new_unknown_method(method_name: String) -> Self {
        Self {
            repr: SlErrorRepr::UnknownMethod(method_name),
        }
    }
}

impl fmt::Display for SlError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.repr {
            SlErrorRepr::Sl(err_code, context) => {
                write!(f, "OpenSLES error: {}", sl_result_to_str(*err_code))?;
                if !context.is_empty() {
                    write!(f, " during {}", context)?;
                }
            }
            SlErrorRepr::UnknownMethod(name) => {
                write!(f, "OpenSLES method {} is not defined", name)?;
            }
        }

        Ok(())
    }
}
impl std::error::Error for SlError {}

macro_rules! try_sl {
    ($e: expr, $ctx: expr) => {{
        let res = $e;
        if res != SL_RESULT_SUCCESS {
            return Err(SlError::new_sl(res, $ctx.into()).into());
        }
        res
    }};
    ($e: expr) => {{
        let res = $e;
        if res != SL_RESULT_SUCCESS {
            return Err(SlError::new_sl(res, String::new()).into());
        }
        res
    }};
}

macro_rules! call_sl {
    ($obj: expr, $name: tt $(, $args:expr )*) => {{
        let ptr = $obj;
        match (**ptr).$name {
            Some(f) => try_sl!(f(ptr, $($args),*), format!("calling {}", stringify!($name))),
            None => {
                return Err(SlError::new_unknown_method(stringify!($name).into()).into());
            }
        }
    }};
}

#[allow(unused_macros)]
macro_rules! call_sl_unchecked {
    ($obj: expr, $name: tt $(, $args:expr )*) => {{
        let ptr = $obj;
        match (**ptr).$name {
            Some(f) => {
                let res = f(ptr, $($args),*);
                if res != SL_RESULT_SUCCESS {
                    error!("OpenSLES method {} error: {}", stringify!($name), sl_result_to_str(res));
                }
            }
            None => {
                error!("Calling unknown OpenSLES method {}", stringify!($name));
            }
        }
    }};
}

macro_rules! call_sl_ignore_res {
    ($obj: expr, $name: tt $(, $args:expr )*) => {{
        let ptr = $obj;
        match (**ptr).$name {
            Some(f) => {
                let _ = f(ptr, $($args),*);
            }
            None => {
                error!("Calling unknown OpenSLES method {}", stringify!($name));
            }
        }
    }};
}

pub struct Settings {
    pub rate: SampleRate,
    pub format: SampleFormat,
}

#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum SampleRate {
    Rate8000,
    Rate44100,
    Rate48000,
}

#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum SampleFormat {
    U8LE,
    S16LE,
    S32LE,
}

pub struct Engine {
    obj: Object,
    itf: Cell<Option<a_ffi::SLEngineItf>>,
}

pub struct OutputMix {
    obj: Object,
}

pub struct AudioPlayer {
    obj: Object,
    play_itf: Cell<Option<a_ffi::SLPlayItf>>,
    buffer_que_itf: Cell<Option<a_ffi::SLAndroidSimpleBufferQueueItf>>,
    play_cb: Option<Box<PlayCallbackWrapper>>,
}

#[allow(dead_code)]
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum PlayState {
    Stopped,
    Paused,
    Playing,
}

impl OutputMix {
    pub fn realize(&self) -> Result<(), Error> {
        self.obj.realize()
    }

    fn from_raw(raw_ptr: a_ffi::SLObjectItf) -> Self {
        Self {
            obj: Object { raw_ptr },
        }
    }
}

impl Engine {
    pub fn new() -> Result<Self, Error> {
        unsafe {
            let mut raw_ptr: a_ffi::SLObjectItf = ptr::null_mut();
            try_sl!(
                a_ffi::slCreateEngine(&mut raw_ptr, 0, ptr::null(), 0, ptr::null(), ptr::null()),
                "engine creation"
            );

            Ok(Self::from_raw(raw_ptr))
        }
    }

    pub fn realize(&self) -> Result<(), Error> {
        self.obj.realize()
    }

    pub fn create_output_mix(&self) -> Result<OutputMix, Error> {
        unsafe {
            let mut mix_raw: a_ffi::SLObjectItf = ptr::null_mut();
            let itf = self.interface()?;

            let ids = [a_ffi::SL_IID_ENVIRONMENTALREVERB];
            let req = [SL_BOOLEAN_FALSE];

            call_sl!(
                itf,
                CreateOutputMix,
                &mut mix_raw,
                1,
                ids.as_ptr(),
                req.as_ptr()
            );

            Ok(OutputMix::from_raw(mix_raw))
        }
    }

    pub fn create_buffer_player(
        &self,
        mix: &OutputMix,
        settings: Settings,
    ) -> Result<AudioPlayer, Error> {
        unsafe {
            let mut loc_bufq = a_ffi::SLDataLocator_AndroidSimpleBufferQueue {
                locatorType: SL_DATALOCATOR_ANDROIDSIMPLEBUFFERQUEUE,
                numBuffers: 5,
            };

            let mut format_pcm = a_ffi::SLDataFormat_PCM {
                formatType: SL_DATAFORMAT_PCM,
                numChannels: 2,
                samplesPerSec: settings.rate.to_raw(),
                bitsPerSample: settings.format.to_raw(),
                containerSize: settings.format.to_raw(),
                channelMask: 0,
                endianness: settings.format.to_raw_endian(),
            };

            let mut audio_src = a_ffi::SLDataSource {
                pLocator: mem::transmute(&mut loc_bufq),
                pFormat: mem::transmute(&mut format_pcm),
            };

            let mut loc_outmix = a_ffi::SLDataLocator_OutputMix {
                locatorType: SL_DATALOCATOR_OUTPUTMIX,
                outputMix: mix.obj.raw_ptr,
            };

            let mut audio_snk = a_ffi::SLDataSink {
                pLocator: mem::transmute(&mut loc_outmix),
                pFormat: ptr::null_mut(),
            };

            let ids = [a_ffi::SL_IID_BUFFERQUEUE, a_ffi::SL_IID_VOLUME];
            let req = [SL_BOOLEAN_TRUE, SL_BOOLEAN_TRUE];

            let itf = self.interface()?;

            let mut raw_ptr: a_ffi::SLObjectItf = ptr::null_mut();
            call_sl!(
                itf,
                CreateAudioPlayer,
                &mut raw_ptr,
                &mut audio_src,
                &mut audio_snk,
                2,
                ids.as_ptr(),
                req.as_ptr()
            );

            Ok(AudioPlayer::from_raw(raw_ptr))
        }
    }

    fn from_raw(raw_ptr: a_ffi::SLObjectItf) -> Self {
        Self {
            obj: Object { raw_ptr },
            itf: Cell::new(None),
        }
    }

    fn interface(&self) -> Result<a_ffi::SLEngineItf, Error> {
        unsafe { self.obj.interface(&self.itf, a_ffi::SL_IID_ENGINE) }
    }
}
unsafe impl Send for Engine {}

impl AudioPlayer {
    pub fn realize(&self) -> Result<(), Error> {
        self.obj.realize()
    }

    pub fn set_play_state(&self, state: PlayState) -> Result<(), Error> {
        unsafe {
            let itf = self.play_interface()?;
            call_sl!(itf, SetPlayState, state.to_raw());
            Ok(())
        }
    }

    pub fn get_play_state(&self) -> Result<PlayState, Error> {
        let itf = self.play_interface()?;
        let mut raw_state: SLuint32 = 0;
        unsafe {
            call_sl!(itf, GetPlayState, &mut raw_state);
        }
        PlayState::from_raw(raw_state)
    }

    pub fn enqueue(&self, buf: &[u8]) -> Result<(), Error> {
        info!("enqueue({})", buf.len());
        let itf = self.buffer_que_interface()?;
        Self::enqueue_raw(itf, buf)
    }

    pub fn register_callback<F>(&mut self, cb: F) -> Result<(), Error>
    where
        F: FnMut(&mut Vec<u8>) -> Result<usize, Error> + 'static,
    {
        self.play_cb = Some(Box::new(PlayCallbackWrapper::new(cb)));

        unsafe extern "C" fn wrapper(itf: a_ffi::SLAndroidSimpleBufferQueueItf, ctx: *mut c_void) {
            let cb_data: &mut PlayCallbackWrapper = mem::transmute(ctx);
            cb_data.call(itf);
        }

        info!(
            "sizeof play_cb: {}",
            mem::size_of::<Option<Box<PlayCallbackWrapper>>>()
        );

        let itf = self.buffer_que_interface()?;
        let cb_ref: &PlayCallbackWrapper = self.play_cb.as_ref().unwrap().as_ref();
        unsafe {
            call_sl!(itf, RegisterCallback, Some(wrapper), mem::transmute(cb_ref));
        }

        Ok(())
    }

    fn enqueue_raw(itf: a_ffi::SLAndroidSimpleBufferQueueItf, buf: &[u8]) -> Result<(), Error> {
        unsafe {
            call_sl!(
                itf,
                Enqueue,
                mem::transmute(buf.as_ptr()),
                buf.len() as SLuint32
            );

            Ok(())
        }
    }

    fn from_raw(raw_ptr: a_ffi::SLObjectItf) -> AudioPlayer {
        Self {
            obj: Object { raw_ptr },
            play_itf: Cell::new(None),
            buffer_que_itf: Cell::new(None),
            play_cb: None,
        }
    }

    fn play_interface(&self) -> Result<a_ffi::SLPlayItf, Error> {
        unsafe { self.obj.interface(&self.play_itf, a_ffi::SL_IID_PLAY) }
    }

    fn buffer_que_interface(&self) -> Result<a_ffi::SLAndroidSimpleBufferQueueItf, Error> {
        unsafe {
            self.obj
                .interface(&self.buffer_que_itf, a_ffi::SL_IID_BUFFERQUEUE)
        }
    }
}
impl Drop for AudioPlayer {
    fn drop(&mut self) {
        let _ = self.set_play_state(PlayState::Stopped);
        self.play_cb = None;
    }
}
unsafe impl Send for AudioPlayer {}

struct PlayCallbackWrapper {
    cb: Box<FnMut(&mut Vec<u8>) -> Result<(usize), Error>>,
    buf: Vec<u8>,
}

impl PlayCallbackWrapper {
    fn new<F>(cb: F) -> Self
    where
        F: FnMut(&mut Vec<u8>) -> Result<(usize), Error> + 'static,
    {
        Self {
            cb: Box::new(cb),
            buf: vec![0; 1024],
        }
    }

    fn call(&mut self, itf: a_ffi::SLAndroidSimpleBufferQueueItf) {
        let res = (self.cb)(&mut self.buf);
        let n = match res {
            Ok(n) => n,
            Err(e) => {
                error!("An error occurred inside the play callback {}", e);
                return;
            }
        };

        if n == 0 {
            return;
        }

        let res = AudioPlayer::enqueue_raw(itf, &self.buf[..n]);
        if let Err(e) = res {
            error!("An error occurred: {}", e);
            return;
        }
    }
}

impl SampleRate {
    fn to_raw(&self) -> SLuint32 {
        match self {
            SampleRate::Rate8000 => SL_SAMPLINGRATE_8,
            SampleRate::Rate44100 => SL_SAMPLINGRATE_44_1,
            SampleRate::Rate48000 => SL_SAMPLINGRATE_48,
        }
    }
}

impl SampleFormat {
    fn to_raw(&self) -> SLuint32 {
        match self {
            SampleFormat::U8LE => SL_PCMSAMPLEFORMAT_FIXED_8 as SLuint32,
            SampleFormat::S16LE => SL_PCMSAMPLEFORMAT_FIXED_16 as SLuint32,
            SampleFormat::S32LE => SL_PCMSAMPLEFORMAT_FIXED_32 as SLuint32,
        }
    }
    fn to_raw_endian(&self) -> SLuint32 {
        match self {
            SampleFormat::U8LE | SampleFormat::S16LE | SampleFormat::S32LE => {
                SL_BYTEORDER_LITTLEENDIAN
            }
        }
    }
}

impl PlayState {
    fn to_raw(&self) -> SLuint32 {
        match self {
            PlayState::Stopped => SL_PLAYSTATE_STOPPED,
            PlayState::Paused => SL_PLAYSTATE_PAUSED,
            PlayState::Playing => SL_PLAYSTATE_PLAYING,
        }
    }
    fn from_raw(raw_state: SLuint32) -> Result<Self, Error> {
        match raw_state {
            SL_PLAYSTATE_STOPPED => Ok(PlayState::Stopped),
            SL_PLAYSTATE_PAUSED => Ok(PlayState::Paused),
            SL_PLAYSTATE_PLAYING => Ok(PlayState::Playing),
            _ => Err(Error::new_wrong_argument(format!(
                "Unknown raw state value: {}",
                raw_state
            ))),
        }
    }
}

struct Object {
    raw_ptr: a_ffi::SLObjectItf,
}

impl Object {
    fn realize(&self) -> Result<(), Error> {
        unsafe {
            call_sl!(self.raw_ptr, Realize, SL_BOOLEAN_FALSE);
            Ok(())
        }
    }

    unsafe fn interface<T>(
        &self,
        itf_ref: &Cell<Option<T>>,
        id: a_ffi::SLInterfaceID,
    ) -> Result<T, Error>
    where
        T: RawPointer,
    {
        let itf = itf_ref.get();

        match &itf {
            Some(i) => Ok(*i),
            None => {
                let itf = self.init_interface::<T>(id)?;
                itf_ref.set(Some(itf));
                Ok(itf)
            }
        }
    }

    unsafe fn init_interface<T: RawPointer>(&self, id: a_ffi::SLInterfaceID) -> Result<T, Error> {
        let mut itf: T = RawPointer::new_null();

        call_sl!(self.raw_ptr, GetInterface, id, itf.mut_ptr_to());
        Ok(itf)
    }
}
impl Drop for Object {
    fn drop(&mut self) {
        unsafe {
            call_sl_ignore_res!(self.raw_ptr, Destroy);
        }
    }
}
unsafe impl Send for Object {}

trait RawPointer: Sized + Copy {
    fn new_null() -> Self;
    fn mut_ptr_to(&mut self) -> *mut c_void;
    fn ptr_to(&self) -> *const c_void;
}

impl<T> RawPointer for *const T {
    fn new_null() -> Self {
        unsafe { mem::transmute(ptr::null_mut::<c_void>()) }
    }

    fn mut_ptr_to(&mut self) -> *mut c_void {
        unsafe { mem::transmute(self) }
    }

    fn ptr_to(&self) -> *const c_void {
        unsafe { mem::transmute(self) }
    }
}

fn sl_result_to_str(err_code: a_ffi::SLresult) -> String {
    match err_code {
        SL_RESULT_SUCCESS => "success".to_owned(),
        SL_RESULT_PRECONDITIONS_VIOLATED => "preconditions_violated".to_owned(),
        SL_RESULT_PARAMETER_INVALID => "parameter_invalid".to_owned(),
        SL_RESULT_MEMORY_FAILURE => "memory_failure".to_owned(),
        SL_RESULT_RESOURCE_ERROR => "resource_error".to_owned(),
        SL_RESULT_RESOURCE_LOST => "resource_lost".to_owned(),
        SL_RESULT_IO_ERROR => "io_error".to_owned(),
        SL_RESULT_BUFFER_INSUFFICIENT => "buffer_insufficient".to_owned(),
        SL_RESULT_CONTENT_CORRUPTED => "content_corrupted".to_owned(),
        SL_RESULT_CONTENT_UNSUPPORTED => "content_unsupported".to_owned(),
        SL_RESULT_CONTENT_NOT_FOUND => "content_not_found".to_owned(),
        SL_RESULT_PERMISSION_DENIED => "permission_denied".to_owned(),
        SL_RESULT_FEATURE_UNSUPPORTED => "feature_unsupported".to_owned(),
        SL_RESULT_INTERNAL_ERROR => "internal_error".to_owned(),
        SL_RESULT_UNKNOWN_ERROR => "unknown_error".to_owned(),
        SL_RESULT_OPERATION_ABORTED => "operation_aborted".to_owned(),
        SL_RESULT_CONTROL_LOST => "control_lost".to_owned(),
        _ => format!("unknown error code: {}", err_code),
    }
}
