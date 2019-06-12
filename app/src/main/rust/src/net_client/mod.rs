use crate::error::Error;
use log::info;
use std::net::{ToSocketAddrs, UdpSocket};
//use std::sync::{
//    atomic::{AtomicBool, Ordering},
//    Arc,
//};

pub fn connect_to<A: ToSocketAddrs + std::fmt::Display>(
    remote_addr: A,
    local_addr: A,
) -> Result<(), Error> {
    info!("Addresses: remote: {}, local{}", remote_addr, local_addr);

    let socket = UdpSocket::bind(local_addr)?;

    info!("Socket is created");

    let info_buf = b"info";
    socket.send_to(info_buf.as_ref(), remote_addr)?;

    info!("Data sent");

    let mut in_buf = vec![0; 2048];
    let n = socket.recv_from(in_buf.as_mut_slice())?.0;

    let in_str = String::from_utf8_lossy(&in_buf[..n]);
    info!("{}", in_str);

    Ok(())
}
