extern crate loom;
extern crate getopts;

use loom::net;
use loom::state;
use loom::gossip;
use loom::data;

use getopts::Options;
use std::env;
use std::string::String;


fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
}

fn loom(port: u16) {
    let mut num = 0;
    let start = num;
    net::read(&srv, &mut m[start..], &mut num).expect("read");
    let end = num;
    s.execute(&mut m[start..end]).expect("state");
    g.execute(&mut m[start..end]).expect("gossip");
    for s in &g.subs {
        net::sendtov4(&srv, &m[start..end], &mut num, s.addr, s.port).expect("send");
    }
}

fn spool(loom: str) {
    let mut num = 0;
    let start = num;
    net::read(&srv, &mut m[start..], &mut num).expect("read");
    let end = num;
    s.execute(&mut m[start..end]).expect("state");
    g.execute(&mut m[start..end]).expect("gossip");
}

pub fn main() {
    let srv = net::server().expect("server");
    let mut s = state::State::new(1024);
    let mut g = gossip::Gossip::new(1024);
    let mut m = vec![data::Message::default(); 1024];

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    opts.optopt("s", "", "Run as a Spool node with the Loom address", "ADDRESS");
    opts.optopt("l", "", "Run as a Loom with a listen port", "PORT");

    if matches.opt_str("s").is_some() {
        let loom = matches.opt_str("s").expect("missing loom address");
        spool(loom);
    } else {
        let ports = matches.opt_str("l").expect("missing loom port");
        let port = ports.parse("expecting u16 number for port");
        loom(port);
    }
}
