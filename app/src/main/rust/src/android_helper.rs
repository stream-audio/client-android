use android_logger;
use android_logger::Config;
use libc;
use libc::{c_char, c_void};
use log::Level;
use log::{info, warn};
use std::ffi;
use std::thread;

pub fn init_log() {
    android_logger::init_once(
        Config::default()
            .with_min_level(Level::Info)
            .with_tag("StreamAudio"),
    );
}

pub fn redirect_stdcout() {
    unsafe {
        let mut pfd: [i32; 2] = [0, 0];
        libc::pipe(pfd.as_mut_ptr());

        let fd0 = pfd[0];
        let fd1 = pfd[1];

        libc::dup2(fd1, libc::STDOUT_FILENO);
        libc::dup2(fd1, libc::STDERR_FILENO);

        thread::spawn(move || thread_reader(fd0));
    }
}

fn thread_reader(fd: i32) {
    let mut buf: Vec<c_char> = vec![0; 128];
    unsafe {
        loop {
            let res = libc::read(fd, buf.as_mut_ptr() as *mut c_void, buf.len() - 1);
            if res <= 0 {
                warn!(
                    "libc::read returned: {}/{}. Stopping thread.",
                    res,
                    *libc::__errno()
                );
                break;
            }

            buf[res as usize] = 0;
            let s = ffi::CStr::from_ptr(buf.as_ptr()).to_string_lossy();

            info!("{}", s);
        }
    }
}
