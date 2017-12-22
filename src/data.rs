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
pub struct POH {
    pub hash: [u8; 32],
    pub counter: u64,
}

#[derive(Copy,Clone)]
#[repr(C)]
pub struct Signature {
    pub data: [u8; 32],
}


#[derive(Copy,Clone)]
#[repr(C)]
pub union MessageData {
    pub tx: Transaction,
    pub poh: POH,
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
    Signature,
}

#[repr(C)]
#[repr(u8)]
pub enum State {
    Unknown,
    Withdrawn,
    Deposited,
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
    pub state: State,
    pub unused: [u8; 6],
    pub data: MessageData,
}
