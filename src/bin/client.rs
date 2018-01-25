extern crate loom;
extern crate getopts;
extern crate rand;
extern crate sha2;
extern crate ed25519_dalek;

use getopts::Options;
use std::env;
use std::mem::transmute;
use std::mem::size_of;
use std::slice::from_raw_parts;

use rand::Rng;
use rand::OsRng;
use sha2::Sha512;
use ed25519_dalek::Keypair;
use ed25519_dalek::Signature;

use loom::data;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
}

fn new_keypair() -> Keypair {
    
    let mut cspring: OsRng = OsRng::new().unwrap();
    let keypair: Keypair = Keypair::generate::<Sha512>(&mut cspring);
    return keypair;
}

fn sign(kp: Keypair, msg: &mut data::Message) {
    let sz = size_of::<data::Payload>();
    let p = &msg.payload as *const data::Payload;
    let buf = transmute(from_raw_parts(p as *const u8, sz));
    msg.sig = kp.sign::<Sha512>(buf);
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
