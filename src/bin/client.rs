extern crate loom;
extern crate rpassword;
extern crate getopts;

use getopts::Options;
use std::env;

use loom::wallet::Wallet;

fn print_usage(program: &str, opts: Options) {                                
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));                               
} 

fn new_key_pair() {
    let path = "loom.wallet";
    let prompt = "./loom.wallet password: ";
    let pass = rpassword::prompt_password_stdout(prompt).unwrap();
    let mut w = Wallet::from_file(path, pass.as_bytes()).unwrap_or(
                    Wallet::new());
    let kp = Wallet::new_keypair();
    w.add_key_pair(kp);
    w.to_file(path, pass.as_bytes());
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
        new_key_pair();
        return;
    }
}
