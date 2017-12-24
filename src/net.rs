#![allow(mutable_transmutes)]

use std::cmp::min;
use std::net::UdpSocket;
use std::mem::transmute;
use std::mem::size_of;
use std::slice::from_raw_parts;
use data::{Message, MAX_PACKET};
use result::{Result,fromIO};

pub fn server() -> Result<UdpSocket> {
    let ret = UdpSocket::bind("0.0.0.0:12345")?;
    ret.set_nonblocking(true)?;
    return Ok(ret);
}

pub fn read(socket: UdpSocket, messages: &mut [Message], num: &mut usize) -> Result<()> {
    let sz = size_of::<Message>();
    let max = messages.len();
    while *num < max {
        unsafe {
            let p = &mut messages[*num] as *mut Message;
            if (max - *num) * sz < MAX_PACKET {
                return Ok(());
            }
            let buf = transmute(from_raw_parts(p as *mut u8, MAX_PACKET));
            let (nrecv, _from) = fromIO(socket.recv_from(buf))?;
            *num = *num + nrecv/sz;
        }
    }
    return Ok(());
}

pub fn write(socket: UdpSocket, messages: &[Message], num: &mut usize) -> Result<()> {
    let sz = size_of::<Message>();
    let max = messages.len();
    while *num < max {
        unsafe {
            let p = &messages[*num] as *const Message;
            let bz = min(MAX_PACKET / sz, max - *num) * sz;
            let buf = transmute(from_raw_parts(p as *const u8, bz));
            let sent_size = fromIO(socket.send(buf))?;
            *num = *num + sent_size / sz;
        }
    }
    return Ok(());
}

#[test]
fn server_test() {
    let sz = size_of::<Message>();
    let srv = server().expect("couldn't create a server");
    let client = UdpSocket::bind("0.0.0.0:0").expect("client socket");
    client.connect("127.0.0.1:12345").expect("connect to server");
    let max = MAX_PACKET/sz;
    let mut m = [Message::default(); 26];
    let mut num = 0;
    write(client, &m[0..max], &mut num).expect("write");
    assert!(num == max);
    num = 0;
    read(srv, &mut m, &mut num).expect("read");
    assert!(num == max);
}

