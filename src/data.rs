use std::net::SocketAddr;

#[repr(C)]
pub struct Transaction {
    pub lvh: [u8; 32],
    pub from: [u8; 32],
    pub to: [u8; 32],
    pub lvh_count: u64,
    pub amount: u64,
    pub fee: u64,
    pub signature: [u8; 32],
}
#[repr(C)]
pub union MessageData {
    pub tx: Transaction,
}

pub const Invalid: u8 = 0;
pub const Transaction: u8 = 1;

#[repr(C)]
pub struct Message {
    pub kind: u8,
    pub data: MessageData,
}

#[repr(C)]
pub struct RecvMessage {
    pub msg: Message,
    pub pad: u64,
}
