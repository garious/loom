use data;
use hasht;
use result::{Result};

#[derive(Default, Copy, Clone)]
#[repr(C)]
struct Subscriber {
    key: [u8; 32],
    addr: u64,
    lastping: u64,
}

impl hasht::Val<[u8;32]> for Subscriber {
    fn key(&self) -> &[u8;32] {
        return &self.key;
    }
}

type SubT = hasht::HashT<[u8;32], Subscriber>;

pub struct Gossip {
    subs: Vec<Subscriber>,
    now: u64,
}

impl Gossip {
    pub fn execute(&mut self, msgs: &mut [data::Message]) -> Result<()> {
        for m in msgs.iter() {
            unsafe {
                match m.kind {
                    data::Kind::Signature => self.now = m.data.poh.counter,
                    data::Kind::Subscribe => {
                        let pos = SubT::find(&self.subs,
                                             &m.data.sub.key)?;
                        let now = self.now;
                        let update = Subscriber{key: m.data.sub.key,
                                                addr: m.data.sub.addr,
                                                lastping: now};
                        *self.subs.get_unchecked_mut(pos) = update
                    }
                    _ => continue,
                }
            }
        }
        return Ok(());
    }
}
