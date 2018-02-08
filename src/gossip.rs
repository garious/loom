//! track gossip subscribers

use data;
use hasht::{HashT, Key, Val};
use result::Result;

#[derive(Default, Copy, Clone)]
#[repr(C)]
pub struct Subscriber {
    pub key: [u8; 32],
    pub addr: [u8; 4],
    pub port: u16,
    pub lastping: u64,
}

impl Val<[u8; 32]> for Subscriber {
    fn key(&self) -> &[u8; 32] {
        &self.key
    }
}

type SubT = HashT<[u8; 32], Subscriber>;

pub struct Gossip {
    pub subs: Vec<Subscriber>,
    now: u64,
    used: usize,
}

impl Gossip {
    pub fn new(size: usize) -> Gossip {
        Gossip {
            subs: vec![Subscriber::default(); size],
            now: 0,
            used: 0,
        }
    }
    fn double(&mut self) -> Result<()> {
        let size = self.subs.len() * 2;
        let mut v = vec![Subscriber::default(); size];
        SubT::migrate(&self.subs, &mut v)?;
        self.subs = v;
        Ok(())
    }
    fn exec(&mut self, m: &data::Message, new: &mut usize) -> Result<()> {
        match m.pld.kind {
            data::Kind::Signature => self.now = m.pld.get_poh().counter,
            data::Kind::Subscribe => {
                let pos = SubT::find(&self.subs, &m.pld.get_sub().key)?;
                let now = self.now;
                let update = Subscriber {
                    key: m.pld.get_sub().key,
                    addr: m.pld.get_sub().addr,
                    port: m.pld.get_sub().port,
                    lastping: now,
                };
                let g = unsafe { self.subs.get_unchecked_mut(pos) };
                if g.key.unused() {
                    *new = *new + 1;
                }
                *g = update;
            }
            _ => return Ok(()),
        }
        Ok(())
    }
    pub fn execute(&mut self, msgs: &mut [data::Message]) -> Result<()> {
        for m in msgs {
            let mut new = 0;
            self.exec(&m, &mut new)?;
            self.used = self.used + new;
            if ((4 * (self.used)) / 3) > self.subs.len() {
                self.double()?;
            }
        }
        Ok(())
    }
}
