extern crate getopts;
extern crate loom;
extern crate rpassword;
extern crate data_encoding;

use getopts::Options;
use std::env;
use std::string::String;
use data_encoding::BASE32HEX_NOPAD;
use loom::wallet::{EncryptedWallet, Wallet};
use loom::data;
use loom::net;
use loom::result::Result;

struct Cfg {
    host: String,
    wallet: String,
}
fn vec_to_array(v: Vec<u8>) -> [u8;32] {
    [v[0],v[1],v[2],v[3]
    ,v[4],v[5],v[6],v[7] 
    ,v[8],v[9],v[10],v[11]
    ,v[12],v[13],v[14],v[15]
    ,v[16],v[17],v[18],v[19]
    ,v[20],v[21],v[22],v[23]
    ,v[24],v[25],v[26],v[27]
    ,v[28],v[29],v[30],v[31]]
}
fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
}

fn load_wallet(cfg: &Cfg, pass: String) -> Wallet {
    let ew = EncryptedWallet::from_file(&cfg.wallet)
            .unwrap_or(EncryptedWallet::new());
    Wallet::decrypt(ew, pass.as_bytes()).unwrap_or(Wallet::new())
}

fn new_key_pair(cfg: &Cfg) {
    let prompt = "loom wallet password: ";
    let pass = rpassword::prompt_password_stdout(prompt).expect("password");
    let mut w = load_wallet(cfg, pass.clone());
    let kp = Wallet::new_keypair();
    w.add_key_pair(kp);
    w.encrypt(pass.as_bytes())
        .expect("encrypt")
        .to_file(&cfg.wallet)
        .expect("write");
}

fn transfer(cfg: &Cfg, from: String, to: String, amnt: u64) -> Result<()> 
{
    let prompt = "loom wallet password: ";
    let pass = rpassword::prompt_password_stdout(prompt).expect("password");
    let w = load_wallet(cfg, pass);
    let fpk = BASE32HEX_NOPAD.decode(from.as_bytes()).expect("from key");
    let tpk = BASE32HEX_NOPAD.decode(to.as_bytes()).expect("to key");
    let tx = data::MessageData {
                tx: data::Transaction{
                    to: vec_to_array(tpk),
                    amount: amnt
                }
           };
    let mut msg = data::Message::default();
    msg.pld.from = vec_to_array(fpk);
    msg.pld.data = tx;
    msg.pld.kind = data::Kind::Transaction;
    w.sign_msg(&mut msg)?;
    let s = net::socket()?;
    s.connect(cfg.host.clone())?;
    let mut num = 0;
    while num < 1 {
        net::write(&s, &[msg], &mut num)?;
    }
    Ok(())
}

fn balance(_addr: String) {}

pub fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let mut cfg = Cfg{host: "loom.loomprotocol.com:12345".to_string(),
                      wallet: "loom.wallet".to_string()};
    let mut opts = Options::new();
    opts.optflag("c", "", "create a new address");
    opts.optflag("x", "", "transfer");
    opts.optflag("b", "", "check the balance of destination address");
    opts.optflag("l", "list", "list your addresses and balances");
    opts.optflag("h", "help", "print this help menu");
    opts.optopt("H", "", "loom node address to use instead of loom.looprotocol.com:12345", "HOST:PORT");
    opts.optopt("W", "", "loom wallet instead of loom.wallet", "PATH");
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
        cfg.host = matches.opt_str("H").expect("loom host address");
    }
    if matches.opt_present("W") {
        cfg.wallet = matches.opt_str("W").expect("loom wallet path");
    }
    if matches.opt_present("c") {
        new_key_pair(&cfg);
        return;
    } else if matches.opt_present("x") {
        let to = matches.opt_str("t").expect("missing destination address");
        let from = matches.opt_str("f").expect("missing source address");
        let astr = matches.opt_str("a").expect("missing ammount");
        let a = astr.parse().expect("ammount is not a number");
        transfer(&cfg, to, from, a).expect("transfer");
        return;
    } else if matches.opt_present("b") {
        let to = matches.opt_str("t").expect("missing destination address");
        balance(to);
        return;
    }
}
