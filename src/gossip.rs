use data;
use hasht::{Key};
use result::{Result};

pub struct Gossip {
    subs: Vec<data::Subscriber>,
    now: u64,
    used: usize,
}

impl Gossip {
    pub fn new(size: usize) -> Gossip {
        let mut v = Vec::new();
        v.clear();
        v.resize(size, data::Subscriber::default());
        return Gossip{subs: v, now: 0, used: 0};
    }
    fn double(&mut self) -> Result<()> {
        let mut v = Vec::new();
        let size = self.subs.len()*2;
        v.resize(size, data::Subscriber::default());
        data::SubT::migrate(&self.subs, &mut v)?;
        self.subs = v;
        return Ok(());
    }
    unsafe fn exec(&mut self,
                   m: &data::Message,
                   new: &mut usize) -> Result<()> {
        match m.kind {
            data::Kind::Signature => self.now = m.data.poh.counter,
            data::Kind::Subscribe => {
                let pos = data::SubT::find(&self.subs,
                                     &m.data.sub.key)?;
                let now = self.now;
                let update = data::Subscriber{key: m.data.sub.key,
                                              addr: m.data.sub.addr,
                                              lastping: now};
                let g = self.subs.get_unchecked_mut(pos);
                if g.key.unused() {
                    *new = *new + 1;
                }
                *g = update;
            }
            _ => return Ok(()),
        }
        return Ok(());
    }
    pub fn execute(&mut self, msgs: &mut [data::Message]) -> Result<()> {
        for m in msgs.iter() {
            let mut new = 0;
            unsafe {
                self.exec(&m, &mut new)?;
            }
            self.used = self.used + new;
            if ((4*(self.used))/3) > self.subs.len() {
                self.double()?;
            }
        }
        return Ok(());
    }
    //
    //downstream broadcast algorithm
    //everyone lower rank
    //      l
    //   s s s s 
    // ss ss ss ss
    // so basically arange a heap based on "rank" and 
    // broadcast down the heap based on the width of the heap
    // rank is based on bond size
    pub fn downstream(subs: &[data::Subscriber],
                      state: &[data::Account]) -> Result<()> {
        return Ok(());
    }


}
