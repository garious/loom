extern crate loom;
use loom::net;
use loom::state;
use loom::gossip;
use loom::data;

pub fn main() {
    let srv = net::server().expect("server");
    let mut s = state::State::new(1024);
    let mut g = gossip::Gossip::new(1024);
    let mut m = Vec::new();
    m.resize(1024, data::Message::default());
    loop {
        let mut num = 0;
        let start = num;
        net::read(&srv, &mut m[start..], &mut num).expect("read");
        let end = num;
        s.execute(&mut m[start..end]).expect("state");
        g.execute(&mut m[start..end]).expect("gossip");
    }
}
