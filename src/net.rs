use std::net::UdpSocket;
use mem::size_of;
use core::slice::from_raw_parts;
use data::{RecvMessage, Message};
use std::io::Result;

pub fn server(socket: &mut UdpSocket) -> Result<UdpSocket> {
    let s = UdpSocket::bind("0.0.0.0:12345")?;
    s.set_nonblocking(true);
    return Ok(s);
}

pub fn read(socket: &mut UdpSocket, messages: &mut [RecvMessage], num: &mut usize) -> Result<()> {
    for v in messages.iter_mut() {
        let mut ptr = &v as *mut RecvMessage;
        let sz = size_of::<RecvMessage>();
        let max = size_of::<Message>();
        let buf = from_raw_parts(ptr, sz);
        let res = socket.recv_from(&mut buf)?;
        match res {
            Ok(nrecv, from) -> 
                if nrecv >= max {
                    v.msg.kind = Invalid;
                }
            Err(e) -> 
                v.msg.kind = Invalid;
                return Ok(());
        };
        *num = *num + 1;
    }
    return Ok(());
}

#[test]
fn server_test() {
}

