use hasht::{HashT, Key, Val};
#[derive(Default, Copy, Clone)]
#[repr(C)]
pub struct Transaction {
    pub lvh: [u8; 32],
    pub from: [u8; 32],
    pub to: [u8; 32],
    pub lvh_count: u64,
    pub amount: u64,
    pub fee: u64,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct POH {
    pub hash: [u8; 32],
    pub counter: u64,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Signature {
    pub data: [u8; 64],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Subscriber {
    pub key: [u8; 32],
    pub addr: [u8; 4],
    pub port: u16,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct GetLedger {
    pub start: u64,
    pub num: u64,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union MessageData {
    pub tx: Transaction,
    pub poh: POH,
    pub sub: Subscriber,
    pub get: GetLedger,
}

impl Default for MessageData {
    fn default() -> MessageData {
        MessageData {
            tx: Transaction::default(),
        }
    }
}

#[derive(PartialEq)]
#[repr(u8)]
pub enum Kind {
    Invalid,
    Transaction,
    Signature,
    Subscribe,
    GetLedger,
}

impl Default for Kind {
    fn default() -> Kind {
        Kind::Invalid
    }
}
impl Copy for Kind {}

impl Clone for Kind {
    fn clone(&self) -> Kind {
        *self
    }
}

#[derive(PartialEq)]
#[repr(u8)]
pub enum State {
    Unknown,
    Withdrawn,
    Deposited,
}
impl Copy for State {}

impl Clone for State {
    fn clone(&self) -> State {
        *self
    }
}

impl Default for State {
    fn default() -> State {
        State::Unknown
    }
}
pub const MAX_PACKET: usize = 1024 * 4;

#[derive(Default, Copy, Clone)]
#[repr(C)]
pub struct Payload {
    pub version: u32,
    pub kind: Kind,
    pub state: State,
    pub unused: u16,
    pub data: MessageData,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Message {
    pub pld: Payload,
    pub sig: [u8; 64],
}

impl Default for Message {
    fn default() -> Message {
        #[cfg_attr(rustfmt, rustfmt_skip)]
        let sig = [0,0,0,0, 0,0,0,0
                  ,0,0,0,0, 0,0,0,0
                  ,0,0,0,0, 0,0,0,0
                  ,0,0,0,0, 0,0,0,0
                  ,0,0,0,0, 0,0,0,0
                  ,0,0,0,0, 0,0,0,0
                  ,0,0,0,0, 0,0,0,0
                  ,0,0,0,0, 0,0,0,0];
        Message {
            pld: Payload::default(),
            sig: sig,
        }
    }
}

#[derive(Default, Copy, Clone)]
#[repr(C)]
pub struct Account {
    pub from: [u8; 32],
    pub balance: u64,
}

impl Key for [u8; 32] {
    fn start(&self) -> usize {
        #[cfg_attr(rustfmt, rustfmt_skip)]
        let st = ((self[0] as u64) << ((7 - 0) * 8)) |
                 ((self[1] as u64) << ((7 - 1) * 8)) |
                 ((self[2] as u64) << ((7 - 2) * 8)) |
                 ((self[3] as u64) << ((7 - 3) * 8)) |
                 ((self[4] as u64) << ((7 - 4) * 8)) |
                 ((self[5] as u64) << ((7 - 5) * 8)) |
                 ((self[6] as u64) << ((7 - 6) * 8)) |
                 ((self[7] as u64) << ((7 - 7) * 8)) ;
        st as usize
    }

    fn unused(&self) -> bool {
        *self == [0u8; 32]
    }
}

impl Val<[u8; 32]> for Account {
    fn key(&self) -> &[u8; 32] {
        &self.from
    }
}
pub type AccountT = HashT<[u8; 32], Account>;
