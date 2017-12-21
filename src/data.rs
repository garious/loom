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

pub struct POH {
    pub hash: [u8; 32],
    pub hash_count: u64,
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

#[repr(C)]
#[repr(u8)]
pub enum Kind {
    Invalid,
    Transaction,
    POH,
}

impl Default for Kind {
    fn default() -> Kind {
        return Kind::Invalid;
    }
}


pub const MAX_PACKET: usize = 1024*4;

#[derive(Default)]
#[repr(C)]
pub struct Message {
    pub kind: Kind,
    pub unused: [u8; 7],
    pub data: MessageData,
}
