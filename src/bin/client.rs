extern crate loom;

use getopts::Options;
use std::env;

use loom::data;
use loom::wallet::Wallet;

fn create() {
    let path = "loom.wallet";
    let prompt = "./loom.wallet password: ";
    let pass = rpassword::prompt_password_stdout("prompt").unwrap();
    let w = Self::from_file(path, pass).or_else(Wallet::new());
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
