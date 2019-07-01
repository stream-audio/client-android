use crate::error::Error;
use jni::{objects::GlobalRef, JNIEnv, JavaVM};
use log::error;
use std::sync::mpsc;
use std::time::{Duration, Instant};

pub enum ToJavaMsg {
    Error(Error),
    BufferSizeChanged(Duration),
    Stop,
}

pub fn java_callback_loop(vm: JavaVM, cb: GlobalRef, recv: mpsc::Receiver<ToJavaMsg>) {
    let _guard = log_err!(vm.attach_current_thread(); "attaching thread to Java VM");
    let env = log_err!(vm.get_env(); "Retrieving JNIEnv");
    let mut this: JavaLoop = log_err!(JavaLoop::new(env, cb); "Creating JavaLoop");

    loop {
        let msg = log_err!(recv.recv(); "receiving from channel in java cb loop");

        match msg {
            ToJavaMsg::Error(e) => {
                let res = this.raise_error(&e);
                if let Err(e2) = res {
                    error!("Cannot send error: {} to java due to: {}", e, e2);
                }
            }
            ToJavaMsg::BufferSizeChanged(duration) => log_and_ignore_err!(
                this.notify_buffer_size_changed(duration),
                "notifying java that the buffer size has changed"
            ),
            ToJavaMsg::Stop => {
                break;
            }
        }
    }
}

struct JavaLoop<'a> {
    env: JNIEnv<'a>,
    cb_obj: GlobalRef,
    last_buffer_size_notify: Option<Instant>,
}

const MIN_NOTIFY_DURATION: Duration = Duration::from_millis(500);

impl<'a> JavaLoop<'a> {
    fn new(env: JNIEnv<'a>, cb: GlobalRef) -> Result<Self, Error> {
        Ok(Self {
            env,
            cb_obj: cb,
            last_buffer_size_notify: None,
        })
    }

    fn raise_error(&self, _e: &Error) -> Result<(), Error> {
        Ok(())
    }

    fn notify_buffer_size_changed(&mut self, duration: Duration) -> Result<(), Error> {
        if !self.should_notify_buffer_size_changed() {
            return Ok(());
        }

        let duration = duration.as_millis() as i64;

        self.env.call_method(
            self.cb_obj.as_obj(),
            "onDelayChangedMs",
            "(J)V",
            &[duration.into()],
        )?;

        Ok(())
    }

    fn should_notify_buffer_size_changed(&mut self) -> bool {
        let res = match &self.last_buffer_size_notify {
            Some(t) => t.elapsed() >= MIN_NOTIFY_DURATION,
            None => true,
        };

        if res {
            self.last_buffer_size_notify = Some(Instant::now());
        }

        res
    }
}
