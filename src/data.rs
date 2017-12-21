#[derive(Copy,Clone)]
#[repr(C)]
#[repr(packed)]
pub struct Transaction {
    pub lvh: [u8; 32],
    pub from: [u8; 32],
    pub to: [u8; 32],
    pub lvh_count: u64,
    pub amount: u64,
    pub fee: u64,
    pub signature: [u8; 32],
}

#[derive(Copy,Clone)]
#[repr(C)]
#[repr(packed)]
pub union MessageData {
    pub tx: Transaction,
}

pub const INVALID: u8 = 0;
pub const TRANSACTION: u8 = 1;

#[repr(C)]
#[repr(packed)]
pub struct Message {
    pub kind: u8,
    pub pad: [u8; 7],
    pub data: MessageData,
}

#[repr(C)]
#[repr(packed)]
pub struct RecvMessage {
    pub msg: Message,
    pub pad: u64,
}
