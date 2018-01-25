extern crate loom;
extern crate getopts;
extern crate crypto;
extern crate rand;

use getopts::Options;
use std::env;
use std::mem::transmute;
use std::mem::size_of;
use std::slice::from_raw_parts;

use loom::data;
use crypto::ed25519;
use rand::Rng;
use rand::os::OsRng;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
}

type Keypair = ([u8; 64], [u8; 32]);

fn new_keypair() -> Keypair {
    let mut rnd: OsRng = OsRng::new().unwrap();
    let mut seed = [0u8; 64];
    rnd.fill_bytes(&mut seed);
    let keypair: Keypair = ed25519::keypair(&seed);
    return keypair;
}

fn sign(kp: Keypair, msg: &mut data::Message) {
    let sz = size_of::<data::Payload>();
    let p = &msg.pld as *const data::Payload;
    let buf = unsafe {
        transmute(from_raw_parts(p as *const u8, sz))
    };
    msg.sig = ed25519::signature(buf, &kp.0);
}

pub fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optflag("c", "", "create a new address");
    opts.optopt("t", "", "transfer", "ADDRESS");
    opts.optopt("f", "", "from address", "ADDRESS");
    opts.optopt("a", "", "amount", "AMOUNT");
    opts.optopt("b", "", "balance", "ADDRESS");
    opts.optflag("l", "list", "list your addresses and balances");
    opts.optflag("h", "help", "print this help menu");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }
}
