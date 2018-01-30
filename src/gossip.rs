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
        let mut v = Vec::new();
        v.clear();
        v.resize(size, Subscriber::default());
        Gossip {
            subs: v,
            now: 0,
            used: 0,
        }
    }
    fn double(&mut self) -> Result<()> {
        let mut v = Vec::new();
        let size = self.subs.len() * 2;
        v.resize(size, Subscriber::default());
        SubT::migrate(&self.subs, &mut v)?;
        self.subs = v;
        Ok(())
    }
    unsafe fn exec(&mut self, m: &data::Message, new: &mut usize) -> Result<()> {
        match m.pld.kind {
            data::Kind::Signature => self.now = m.pld.data.poh.counter,
            data::Kind::Subscribe => {
                let pos = SubT::find(&self.subs, &m.pld.data.sub.key)?;
                let now = self.now;
                let update = Subscriber {
                    key: m.pld.data.sub.key,
                    addr: m.pld.data.sub.addr,
                    port: m.pld.data.sub.port,
                    lastping: now,
                };
                let g = self.subs.get_unchecked_mut(pos);
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
        for m in msgs.iter() {
            let mut new = 0;
            unsafe {
                self.exec(&m, &mut new)?;
            }
            self.used = self.used + new;
            if ((4 * (self.used)) / 3) > self.subs.len() {
                self.double()?;
            }
        }
        Ok(())
    }
}
