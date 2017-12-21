use std::net::UdpSocket;
use std::mem::transmute;
use std::mem::size_of;
use std::slice::from_raw_parts;
use data::{RecvMessage, Message, INVALID};
use std::io::Result;

pub fn server(socket: &mut UdpSocket) -> Result<UdpSocket> {
    let s = UdpSocket::bind("0.0.0.0:12345")?;
    s.set_nonblocking(true);
    return Ok(s);
}

pub fn read(socket: &mut UdpSocket, messages: &mut [RecvMessage], num: &mut usize) -> Result<()> {
    for v in messages.iter_mut() {
        unsafe {
            let sz = size_of::<RecvMessage>();
            let p = v as *mut RecvMessage;
            let buf = transmute(from_raw_parts(p, sz));
            let max = size_of::<Message>();
            let res = socket.recv_from(buf);

            match res {
                Ok((nrecv, _from)) => 
                    if nrecv >= max {
                        v.msg.kind = INVALID
                    },
                Err(_) =>  {
                    v.msg.kind = INVALID;
                    break;
                 }
            };
        }
        *num = *num + 1;
    }
    return Ok(());
}

#[test]
fn server_test() {
}

