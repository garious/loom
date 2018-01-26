use std::slice::from_raw_parts;
use std::fs::File;
use std::mem::transmute;
use std::mem::size_of;

use crypto::ed25519;
use rand::Rng;
use rand::os::OsRng;

use data;
use serde_json;


type Keypair = ([u64; 8], [u64; 4]);

#[derive(Serialize, Deserialize)]
pub struct Wallet {
    pub pubkeys: Vec<[u64;4]>,
    pub privkeys: Vec<[u64;8]>,
}

impl Wallet {
    pub fn new() -> Wallet {
        return Wallet{pubkeys: Vec::new(), privkeys: Vec::new()}
    }
    pub fn add_key_pair(&mut self, pk: Keypair) {
        self.privkeys.push(pk.0);
        self.pubkeys.push(pk.1);
    }
    pub fn read_wallet_from_file(path: &str) -> Wallet {
        let file = File::open(path).expect("open wallet");
        return serde_json::from_reader(file).expect("parsing error");
    }
    pub fn write_wallet_to_file(path: &str, w: &Wallet) {
        let mut file = File::open(path).expect("open wallet");
        serde_json::to_writer(&mut file, w).unwrap();
    }
    pub fn new_keypair() -> Keypair {
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
    pub fn sign(kp: Keypair, msg: &mut data::Message) {
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
}
