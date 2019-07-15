mod pkt_decoder;

pub use pkt_decoder::Pkt;

use crate::error::Error;
use crate::ffmpeg;
use crate::jni_ffi::ToJavaMsg;
use crate::player::Player;
use crate::util::interval_measure::IntervalMeasure;
use log::{info, warn};
use mio::net::UdpSocket;
use std::io;
use std::net::SocketAddr;
use std::sync::mpsc;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::thread::{self, JoinHandle};

const UDP_TOKEN: mio::Token = mio::Token(0);
const STOP_TOKEN: mio::Token = mio::Token(1);

pub struct NetClient {
    set_readiness: mio::SetReadiness,
    stop_flag: Arc<AtomicBool>,
    join_handle: Option<JoinHandle<()>>,
}

impl NetClient {
    pub fn new(
        remote_addr: SocketAddr,
        local_addr: SocketAddr,
        player: Player,
        to_java_send: mpsc::Sender<ToJavaMsg>,
    ) -> Result<Self, Error> {
        let socket = UdpSocket::bind(&local_addr)?;

        let poll = mio::Poll::new()?;

        let (registration, set_readiness) = mio::Registration::new2();
        let stop_flag = Arc::new(AtomicBool::new(false));
        let stopper = Stopper {
            registration,
            flag: stop_flag.clone(),
        };

        poll.register(
            &socket,
            UDP_TOKEN,
            mio::Ready::readable(),
            mio::PollOpt::edge(),
        )?;

        poll.register(
            &stopper,
            STOP_TOKEN,
            mio::Ready::readable(),
            mio::PollOpt::level(),
        )?;

        socket.send_to(b"info", &remote_addr)?;

        let from_params = ffmpeg::AudioParams {
            rate: 44100,
            format: ffmpeg::AudioSampleFormat::FloatLe,
        };
        let to_params = ffmpeg::AudioParams {
            rate: 44100,
            format: ffmpeg::AudioSampleFormat::S16Le,
        };
        let resampler = ffmpeg::Resampler::new(from_params, to_params)?;
        let decoder = ffmpeg::Decoder::new(ffmpeg::Codec::Aac)?;

        let poll_loop = PollLoop {
            poll,
            socket,
            addr: remote_addr,
            state: State::InfoRequested,
            player,
            stopper,
            resampler,
            decoder,
            to_java_send,
            interval_measure: IntervalMeasure::new(),
            pkt_decoder: pkt_decoder::PktDecoder::new(),
        };
        let join_handle = thread::spawn(move || poll_loop.poll_loop());

        Ok(Self {
            set_readiness,
            stop_flag,
            join_handle: Some(join_handle),
        })
    }

    pub fn stop(&mut self) -> Result<(), Error> {
        self.stop_flag.store(true, Ordering::SeqCst);
        self.set_readiness.set_readiness(mio::Ready::readable())?;
        if let Some(join_handle) = self.join_handle.take() {
            let res = join_handle.join();
            if let Err(_) = res {
                warn!("Thread with mio::Poll panicked");
            }
        }

        Ok(())
    }
}
impl Drop for NetClient {
    fn drop(&mut self) {
        let res = self.stop();
        if let Err(e) = res {
            warn!("Error stopping network thread: {}", e);
        }
    }
}

#[derive(Debug)]
enum State {
    InfoRequested,
    Started,
}

struct PollLoop {
    poll: mio::Poll,
    socket: UdpSocket,
    addr: SocketAddr,
    state: State,
    player: Player,
    stopper: Stopper,
    resampler: ffmpeg::Resampler,
    decoder: ffmpeg::Decoder,
    to_java_send: mpsc::Sender<ToJavaMsg>,
    interval_measure: IntervalMeasure,
    pkt_decoder: pkt_decoder::PktDecoder,
}

struct Stopper {
    registration: mio::Registration,
    flag: Arc<AtomicBool>,
}

impl PollLoop {
    fn poll_loop(mut self) {
        let mut events = mio::Events::with_capacity(1024);
        let mut buf = vec![0; 65536];

        loop {
            self.poll.poll(&mut events, None).unwrap();
            for event in &events {
                match event.token() {
                    UDP_TOKEN => self.receive_data(&mut buf),
                    STOP_TOKEN => {
                        if self.stopper.is_stopped() {
                            self.send_stop();
                            return;
                        }
                    }
                    _ => unreachable!(),
                }
            }
        }
    }

    fn receive_data(&mut self, buf: &mut [u8]) {
        loop {
            let res = self.socket.recv_from(buf);
            match res {
                Ok((n, _)) => self.process_data(&buf[..n]),
                Err(e) => {
                    if !is_try_again(&e) {
                        warn!("Error receiving data: {}", e);
                    }
                    break;
                }
            }
        }
    }

    fn send_stop(&mut self) {
        let res = self.socket.send_to(b"stop", &self.addr);
        if let Err(e) = res {
            warn!("Error sending stop to: {}. {}", self.addr, e);
        }
    }

    fn process_data(&mut self, buf: &[u8]) {
        match self.state {
            State::InfoRequested => self.send_start(),
            State::Started => {
                let res = self.play(buf);
                if let Err(e) = res {
                    warn!("Error playing buffer: {}", e);
                }
            }
        }
    }

    fn send_start(&mut self) {
        info!("Sending start");
        let res = self.player.start_playing();
        if let Err(e) = res {
            warn!("Error setting start playing: {}", e);
        }

        let res = self.socket.send_to(b"start", &self.addr);
        if let Err(e) = res {
            warn!("Error sending start to {}: {}", self.addr, e);
        }

        self.state = State::Started;
    }

    fn play(&mut self, buf: &[u8]) -> Result<(), Error> {
        let is_changed = self.interval_measure.new_event();
        if is_changed {
            info!("Packet intervals: {}", self.interval_measure);
        }

        let pkt = self.pkt_decoder.parse(buf)?;
        let buf = match pkt.data {
            None => {
                self.player.enqueue(&pkt);
                return Ok(());
            }
            Some(buf) => buf,
        };

        self.decoder.write(&buf)?;
        while let Some(data) = self.decoder.read()? {
            let data = self.resampler.resample(data)?;
            let pkt = Pkt::new_borrower(pkt.cnt, data);
            self.player.enqueue(&pkt);
        }

        Ok(())
    }
}

impl Stopper {
    fn is_stopped(&self) -> bool {
        self.flag.load(Ordering::SeqCst)
    }
}

impl mio::Evented for Stopper {
    fn register(
        &self,
        poll: &mio::Poll,
        token: mio::Token,
        interest: mio::Ready,
        opts: mio::PollOpt,
    ) -> Result<(), std::io::Error> {
        self.registration.register(poll, token, interest, opts)
    }

    fn reregister(
        &self,
        poll: &mio::Poll,
        token: mio::Token,
        interest: mio::Ready,
        opts: mio::PollOpt,
    ) -> Result<(), std::io::Error> {
        self.registration.reregister(poll, token, interest, opts)
    }

    fn deregister(&self, poll: &mio::Poll) -> Result<(), std::io::Error> {
        poll.deregister(&self.registration)
    }
}

fn is_try_again(e: &io::Error) -> bool {
    match e.kind() {
        io::ErrorKind::WouldBlock => true,
        _ => false,
    }
}
