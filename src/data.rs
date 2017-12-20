use std::net::SocketAddr;

#[repr(C)]
pub struct Transaction {
    lvh: [u8; 32],
    from: [u8; 32],
    to: [u8; 32],
    lvh_count: u64,
    amount: u64,
    fee: u64,
    signature: [u8; 32],
}
#[repr(C)]
pub union MessageData {
    tx: Transaction,
}

pub const Invalid: u8 = 0;
pub const Transaction: u8 = 1;

#[repr(C)]
pub struct Message {
    kind: u8,
    data: MessageData,
}

#[repr(C)]
pub struct RecvMessage {
    msg: Message,
    pad: u64,
}
