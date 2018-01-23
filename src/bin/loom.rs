extern crate loom;
use loom::net;
use loom::state;
use loom::gossip;
use loom::data;
use std::mem::uninitialized;

pub fn main() {
    let srv = net::server().expect("server");
    let mut s = state::State::new(1024);
    let mut g = gossip::Gossip::new(1024);
    let mut m: [data::Message; 1024] = unsafe { uninitialized() };
    loop {
        let mut num = 0;
        let start = num;
        net::read(&srv, &mut m[start .. ], &mut num).expect("read");
        let end = num;
        s.execute(&mut m[start .. end]).expect("state");
        g.execute(&mut m[start .. end]).expect("gossip");
        for s in g.subs.iter() {
            net::sendtov4(&srv, &m[start .. end], &mut num,
                          s.addr, s.port).expect("send");
        }
    }
}
