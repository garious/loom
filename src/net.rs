//! network code, assuming all endpoints are reading and writing little endian C99 LP64 layout.

#![allow(mutable_transmutes)]

use std::cmp::min;
use std::net::UdpSocket;
use std::mem::transmute;
use std::mem::size_of;
use std::slice::from_raw_parts;
use std::net::SocketAddr;
use std::net::Ipv4Addr;
use std::net::IpAddr;
use data::{Message, MAX_PACKET};
use result::Result;
use result::Error::IO;

pub fn bindall(port: u16) -> Result<UdpSocket> {
    let ipv4 = Ipv4Addr::new(0, 0, 0, 0);
    let addr = SocketAddr::new(IpAddr::V4(ipv4), port);
    let rv = UdpSocket::bind(&addr)?;
    return Ok(rv);
}

pub fn socket() -> Result<UdpSocket> {
    let ret = UdpSocket::bind("0.0.0.0:0")?;
    Ok(ret)
}

pub fn read_from(
    socket: &UdpSocket,
    messages: &mut [Message],
    mdata: &mut [(usize, SocketAddr)],
) -> Result<usize> {
    let sz = size_of::<Message>();
    let max = messages.len();
    let mut total = 0usize;
    let mut ix = 0usize;
    socket.set_nonblocking(false)?;
    while total < max {
        let p = &mut messages[total] as *mut Message;
        if (max - total) * sz < MAX_PACKET {
            return Ok(ix);
        }
        assert!(cfg!(target_endian = "little"));
        let buf = unsafe { transmute(from_raw_parts(p as *mut u8, MAX_PACKET)) };
        trace!("recv_from");
        match socket.recv_from(buf) {
            Err(_) if ix > 0 => {
                socket.set_nonblocking(false)?;
                return Ok(ix);
            }
            Err(e) => {
                info!("recv_from err {:?}", e);
                return Err(IO(e));
            }
            Ok((nrecv, from)) => {
                trace!("got recv_from {:?}", nrecv);
                total += nrecv / sz;
                trace!("total recv_from {:?}", total);
                *mdata.get_mut(ix).unwrap() = (nrecv / sz, from);
                ix += 1;
                socket.set_nonblocking(true)?;
            }
        }
        trace!("done recv_from");
    }
    Ok(ix)
}

pub fn read(socket: &UdpSocket, messages: &mut [Message], num: &mut usize) -> Result<()> {
    let sz = size_of::<Message>();
    let max = messages.len();
    while *num < max {
        let p = &mut messages[*num] as *mut Message;
        if (max - *num) * sz < MAX_PACKET {
            return Ok(());
        }
        assert!(cfg!(target_endian = "little"));
        let buf = unsafe { transmute(from_raw_parts(p as *mut u8, MAX_PACKET)) };
        let (nrecv, _from) = socket.recv_from(buf)?;
        *num = *num + nrecv / sz;
    }
    Ok(())
}

pub fn write(socket: &UdpSocket, messages: &[Message], num: &mut usize) -> Result<()> {
    let sz = size_of::<Message>();
    let max = messages.len();
    while *num < max {
        let p = &messages[*num] as *const Message;
        let bz = min(MAX_PACKET / sz, max - *num) * sz;
        assert!(cfg!(target_endian = "little"));
        let buf = unsafe { transmute(from_raw_parts(p as *const u8, bz)) };
        let sent_size = socket.send(buf)?;
        *num += sent_size / sz;
    }
    Ok(())
}

pub fn send_to(
    socket: &UdpSocket,
    msgs: &[Message],
    num: &mut usize,
    addr: SocketAddr,
) -> Result<()> {
    let sz = size_of::<Message>();
    let max = msgs.len();
    while *num < max {
        let p = &msgs[*num] as *const Message;
        let bz = min(MAX_PACKET / sz, max - *num) * sz;
        assert!(cfg!(target_endian = "little"));
        let buf = unsafe { transmute(from_raw_parts(p as *const u8, bz)) };
        let sent_size = socket.send_to(buf, &addr)?;
        *num = *num + sent_size / sz;
    }
    Ok(())
}

pub fn sendtov4(
    socket: &UdpSocket,
    msgs: &[Message],
    num: &mut usize,
    a: [u8; 4],
    port: u16,
) -> Result<()> {
    let ipv4 = Ipv4Addr::new(a[0], a[1], a[2], a[3]);
    let addr = SocketAddr::new(IpAddr::V4(ipv4), port);
    send_to(socket, msgs, num, addr)
}

#[test]
fn read_write_test() {
    let sz = size_of::<Message>();
    let srv = bindall(12345).expect("couldn't create a server");
    let cli = socket().expect("socket create");
    cli.connect("127.0.0.1:12345").expect("client");
    let max = MAX_PACKET / sz;
    let mut m = [Message::default(); 26];
    let mut num = 0;
    write(&cli, &m[0..max], &mut num).expect("write");
    assert!(num == max);

    read(&srv, &mut m, &mut num).expect("read");
    assert!(num == max);
}
