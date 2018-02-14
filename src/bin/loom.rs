extern crate getopts;
extern crate loom;
extern crate rpassword;
extern crate data_encoding;

use getopts::Options;
use std::env;
use std::string::String;
use data_encoding::BASE32HEX_NOPAD;
use loom::wallet::{EncryptedWallet, Wallet};

struct Cfg {
    host: &str,
    wallet: &str,
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
}

fn load_wallet(cfg: &Cfg) -> Result<Wallet> {
    let prompt = "./loom.wallet password: ";
    let pass = rpassword::prompt_password_stdout(prompt).expect("password");
    let ew = EncryptedWallet::from_file(cfg.wallet).unwrap_or(EncryptedWallet::new());
    let w = Wallet::decrypt(ew, pass.as_bytes()).unwrap_or(Wallet::new());
    return Ok(w);
}

fn new_key_pair(cfg: &Cfg) {
    let mut w = load_wallet();
    let kp = Wallet::new_keypair();
    w.add_key_pair(kp);
    w.encrypt(pass.as_bytes())
        .expect("encrypt")
        .to_file(cfg.wallet)
        .expect("write");
}


fn transfer(cfg: &Cfg, from: String, to: String, amnt: u64) -> Result<()> 
{
    let mut w = load_wallet();
    let fpk = BASE32HEX_NOPAD.decode_len(from).expect("from key");
    let tpk = BASE32HEX_NOPAD.decode_len(to).expect("to key");
    let mut msg = data::Message {
        from: fpk,
        lvh: [0; 32],
        lvh_count: 0,
        fee: 0,
        data: MessageData{tx: Transaction{to: tpk, amount: amnt}},
        version: 0,
        kind: Transaction,
        state: Unknown,
        unused: 0,
    };
    w.sign_with(fpk, &msg)?;
    let s = net::socket()?;
    s.connect(cfg.host);
    net::write(&s, &[msg])?;
}

fn balance(_addr: String) {}

pub fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let mut Cfg = Cfg{host: &"loom.loomprotocol.com:12345", wallet=&"loom.wallet"};
    let mut opts = Options::new();
    opts.optflag("c", "", "create a new address");
    opts.optflag("x", "", "transfer");
    opts.optflag("b", "", "check the balance of destination address");
    opts.optflag("l", "list", "list your addresses and balances");
    opts.optflag("h", "help", "print this help menu");
    opts.optopt("H", "", "loom node address to use instead of loom.looprotocol.com:12345", "HOST:PORT");
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
    if matches.opt_present("H") {
        cfg.host = &matches.opt_str("H").expect("loom host address");
    }
    if matches.opt_present("W") {
        cfg.path = &matches.opt_str("W").expect("loom wallet path");
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
