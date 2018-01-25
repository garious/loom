extern crate serde;
extern crate serde_json;
extern crate loom;
extern crate getopts;
extern crate crypto;
extern crate rand;

use getopts::Options;
use std::env;
use std::mem::transmute;
use std::mem::size_of;
use std::slice::from_raw_parts;
use std::fs::File;

use loom::data;
use crypto::ed25519;
use rand::Rng;
use rand::os::OsRng;

#[macro_use]
extern crate serde_derive;

type Keypair = ([u64; 8], [u64; 4]);

#[derive(Serialize, Deserialize)]
struct Wallet {
    pubkeys: Vec<[u64;4]>,
    privkeys: Vec<[u64;8]>,
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
}

fn read_wallet_from_file(path: &str) -> Wallet {
    let file = File::open(path).expect("open wallet");
    return serde_json::from_reader(file).expect("parsing error");
}

fn new_keypair() -> Keypair {
    let mut rnd: OsRng = OsRng::new().unwrap();
    let mut seed = [0u8; 64];
    rnd.fill_bytes(&mut seed);
    let (a,b) = ed25519::keypair(&seed);
    let ap = unsafe {
        transmute::<[u8;64], [u64;8]>(a)
    };
    let bp = unsafe {
        transmute::<[u8;32], [u64;4]>(b)
    };
    return (ap, bp);
}

fn sign(kp: Keypair, msg: &mut data::Message) {
    let sz = size_of::<data::Payload>();
    let p = &msg.pld as *const data::Payload;
    let buf = unsafe {
        transmute(from_raw_parts(p as *const u8, sz))
    };
    let pk = unsafe {
        transmute::<[u64;8], [u8;64]>(kp.0)
    };
    msg.sig = ed25519::signature(buf, &pk);
}
fn create() {
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
    if matches.opt_present("c") {
        create();
        return;
    }
}
