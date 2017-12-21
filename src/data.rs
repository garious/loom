#[derive(Default)]
#[derive(Copy,Clone)]
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

#[derive(Copy,Clone)]
#[repr(C)]
pub union MessageData {
    pub tx: Transaction,
}

impl Default for MessageData {
    fn default() -> MessageData {
        return MessageData{tx : Transaction::default()};
    }
}

pub const INVALID: u8 = 0;
pub const TRANSACTION: u8 = 1;
pub const MAX_PACKET: usize = 1024*16;

#[derive(Default)]
#[repr(C)]
pub struct Message {
    pub kind: u8,
    pub unused: [u8; 7],
    pub data: MessageData,
}
