use data;
use hasht;
use result;

struct Subscriber {
    key: [u8; 32],
    addr: SockAddr,
    lastping: u64,
}

struct Gossip {
    subscribers: Vec<Subscriber>,
}

impl Gossip {
    pub fn execute(&self, msgs: &mut [data::Message]) {
        for m in msgs {
            if m.kind == GossipSubscribe {
                let pos = hasht::find(&self.subscribers, m.data.subs.key)?;
                let update = Subscriber{m.data.subs.addr, m.data.key, now};
                *self.subscribers.get_unchecked_mut(pos) = update;
            }
        }
    }
}
