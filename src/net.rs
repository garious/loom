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

pub fn server() -> Result<UdpSocket> {
    let ret = UdpSocket::bind("0.0.0.0:12345")?;
    Ok(ret)
}

pub fn ledger_server() -> Result<UdpSocket> {
    let ret = UdpSocket::bind("0.0.0.0:12346")?;
    //    ret.set_nonblocking(true)?;
    Ok(ret)
}

pub fn client(uri: &str) -> Result<UdpSocket> {
    let ret = UdpSocket::bind("0.0.0.0:0")?;
    ret.connect(uri)?;
    Ok(ret)
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
        unsafe {
            let p = &mut messages[total] as *mut Message;
            if (max - total) * sz < MAX_PACKET {
                return Ok(ix);
            }
            let buf = transmute(from_raw_parts(p as *mut u8, MAX_PACKET));
            trace!("recv_from");
            match socket.recv_from(buf) {
                Err(_) if ix > 0 => {
                    socket.set_nonblocking(false)?;
                    return Ok(ix);
                },
                Err(e) => {
                    info!("recv_from err {:?}", e);
                    return Err(IO(e));
                }
                Ok((nrecv, from)) => {
                    trace!("got recv_from {:?}", nrecv);
                    total = total + nrecv / sz;
                    trace!("total recv_from {:?}", total);
                    *mdata.get_unchecked_mut(ix) = (nrecv / sz, from);
                    ix = ix + 1;
                    socket.set_nonblocking(true)?;
                }
            }
            trace!("done recv_from");
        }
    }
    Ok(ix)
}

pub fn read(socket: &UdpSocket, messages: &mut [Message], num: &mut usize) -> Result<()> {
    let sz = size_of::<Message>();
    let max = messages.len();
    while *num < max {
        unsafe {
            let p = &mut messages[*num] as *mut Message;
            if (max - *num) * sz < MAX_PACKET {
                return Ok(());
            }
            let buf = transmute(from_raw_parts(p as *mut u8, MAX_PACKET));
            let (nrecv, _from) = socket.recv_from(buf)?;
            *num = *num + nrecv / sz;
        }
    }
    Ok(())
}

pub fn write(socket: &UdpSocket, messages: &[Message], num: &mut usize) -> Result<()> {
    let sz = size_of::<Message>();
    let max = messages.len();
    while *num < max {
        unsafe {
            let p = &messages[*num] as *const Message;
            let bz = min(MAX_PACKET / sz, max - *num) * sz;
            let buf = transmute(from_raw_parts(p as *const u8, bz));
            let sent_size = socket.send(buf)?;
            *num = *num + sent_size / sz;
        }
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
        unsafe {
            let p = &msgs[*num] as *const Message;
            let bz = min(MAX_PACKET / sz, max - *num) * sz;
            let buf = transmute(from_raw_parts(p as *const u8, bz));
            let sent_size = socket.send_to(buf, &addr)?;
            *num = *num + sent_size / sz;
        }
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
    let srv = server().expect("couldn't create a server");
    let cli = client("127.0.0.1:12345").expect("client");
    let max = MAX_PACKET / sz;
    let mut m = [Message::default(); 26];
    let mut num = 0;
    write(&cli, &m[0..max], &mut num).expect("write");
    assert!(num == max);

    read(&srv, &mut m, &mut num).expect("read");
    assert!(num == max);
}
