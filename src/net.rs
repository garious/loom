use std::net::UdpSocket;
use core::slice::from_raw_parts;

struct Transaction {
    lvh: u8[32];
    from: u8[32];
    to: u8[32];
    lvh_count: u64;
    amount: u64;
    fee: u64;
    signature: u8[32];
}
union MessageData {
    tx: Transaction;
}

Invalid: const u8 = 0;
Transaction: const u8 = 1;

struct Message {
    kind: u8;
    data: MessageData;
}

struct RecvMessage {
    msg: Message;
    from: SocketAddr;
}

fn server(socket: &mut UdpSocket) -> Result<UdpSocket> {
    let s = UdpSocket::bind("0.0.0.0:12345")?;
    s.set_nonblocking()
    return Ok(s);
}

fn read(socket: &mut UdpSocket, messages: &mut [RecvMessage], num: &mut usize) -> Result<()> {
    for v in messages.iter_mut() {
        let mut ptr = &v as *RecvMessage;
        let sz = mem::size_of::<RecvMessage>();
        let max = mem::size_of::<Message>();
        let mut buf = from_raw_parts(ptr, sz);
        let res = socket.recv_from(&mut buf)?;
        match res {
            Ok(nrecv, from) -> 
                if nrecv >= max {
                    v.msg.kind = Invalid;
                }
                v.from = from;
            Err(e) -> 
                v.msg.kind = Invalid;
                return Ok(());
        };
        num = num + 1;
    }
}

