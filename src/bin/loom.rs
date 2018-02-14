extern crate getopts;
extern crate loom;
extern crate rpassword;

use getopts::Options;
use std::env;
use std::string::String;

use loom::wallet::{EncryptedWallet, Wallet};

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
}

fn new_key_pair() {
    let path = "loom.wallet";
    let prompt = "./loom.wallet password: ";
    let pass = rpassword::prompt_password_stdout(prompt).expect("password");
    println!("HERE");
    let ew = EncryptedWallet::from_file(path).unwrap_or(EncryptedWallet::new());
    println!("HERE2");
    let mut w = Wallet::decrypt(ew, pass.as_bytes()).unwrap_or(Wallet::new());
    let kp = Wallet::new_keypair();
    w.add_key_pair(kp);
    w.encrypt(pass.as_bytes())
        .expect("encrypt")
        .to_file(path)
        .expect("write");
}

fn transfer(_from: String, _to: String, _amnt: u64) {}

fn balance(_addr: String) {}

pub fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optflag("c", "", "create a new address");
    opts.optflag("x", "", "transfer");
    opts.optflag("b", "", "check the balance of destination address");
    opts.optflag("l", "list", "list your addresses and balances");
    opts.optflag("h", "help", "print this help menu");
    opts.optopt("t", "", "destination address", "ADDRESS");
    opts.optopt("f", "", "source address", "ADDRESS");
    opts.optopt("a", "", "amount", "AMOUNT");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }
    if matches.opt_present("c") {
        new_key_pair();
        return;
    } else if matches.opt_present("x") {
        let to = matches.opt_str("t").expect("missing destination address");
        let from = matches.opt_str("f").expect("missing source address");
        let astr = matches.opt_str("a").expect("missing ammount");
        let a = astr.parse().expect("ammount is not a number");
        transfer(to, from, a);
        return;
    } else if matches.opt_present("b") {
        let to = matches.opt_str("t").expect("missing destination address");
        balance(to);
        return;
    }
}
