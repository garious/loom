extern crate getopts;
extern crate loom;

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
    loop {
        let mut s = state::State::new(1024);
        let mut g = gossip::Gossip::new(1024);
        let mut m = vec![data::Message::default(); 1024];
        let srv = net::bindall(port).expect("bind server port");
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
}

fn spool(loom: &str) {
    loop {
        let mut s = state::State::new(1024);
        let mut g = gossip::Gossip::new(1024);
        let mut m = vec![data::Message::default(); 1024];
        let mut num = 0;
        let start = num;
        let srv = net::socket().expect("connect to loom server");
        srv.connect(loom).expect("socket connect");
        net::read(&srv, &mut m[start..], &mut num).expect("read");
        let end = num;
        s.execute(&mut m[start..end]).expect("state");
        g.execute(&mut m[start..end]).expect("gossip");
    }
}

pub fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let mut opts = Options::new();
    opts.optflag("h", "help", "print this help menu");
    opts.optopt(
        "s",
        "",
        "Run as a Spool node with the Loom address",
        "ADDRESS",
    );
    opts.optopt("l", "", "Run as a Loom with a listen port", "PORT");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) =>  {
            print_usage(&program, opts);
            panic!(f.to_string());
        }
    };
    if matches.opt_str("s").is_some() {
        let loom: String = matches.opt_str("s").expect("missing loom address");
        spool(&loom);
    } if matches.opt_str("l").is_some() {
        let ports = matches.opt_str("l").expect("missing loom port");
        let port = ports.parse().expect("expecting u16 number for port");
        loom(port);
    } else {
        print_usage(&program, opts);
        return;
    }

}
