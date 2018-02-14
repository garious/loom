//! wallet library

use std::slice::from_raw_parts;
use std::fs::File;
use std::mem::transmute;
use std::mem::size_of;
use std::io::Read;
use std::io::Write;

use crypto::ed25519;
use rand::Rng;
use rand::os::OsRng;

use data;
use result::Result;
use serde_json;
use aes;

type Keypair = ([u64; 8], [u64; 4]);

#[derive(Serialize, Deserialize)]
pub struct EncryptedWallet {
    pub pubkeys: Vec<[u64; 4]>,
    pub privkeys: Vec<u8>,
}

#[derive(Serialize, Deserialize)]
pub struct Wallet {
    pub pubkeys: Vec<[u64; 4]>,
    pub privkeys: Vec<[u64; 8]>,
}

impl EncryptedWallet {
    pub fn new() -> EncryptedWallet {
        EncryptedWallet {
            pubkeys: Vec::new(),
            privkeys: Vec::new(),
        }
    }
    pub fn from_file(path: &str) -> Result<EncryptedWallet> {
        let mut file = File::open(path)?;
        let mut e = Vec::new();
        let _sz = file.read_to_end(&mut e)?;
        let ew: EncryptedWallet = serde_json::from_slice(&e)?;
        Ok(ew)
    }
    pub fn to_file(&self, path: &str) -> Result<()> {
        let mut file = File::create(path)?;
        let d = serde_json::to_vec(self)?;
        file.write_all(&d)?;
        Ok(())
    }
}

impl Wallet {
    pub fn new() -> Wallet {
        Wallet {
            pubkeys: Vec::new(),
            privkeys: Vec::new(),
        }
    }
    pub fn add_key_pair(&mut self, pk: Keypair) {
        self.privkeys.push(pk.0);
        self.pubkeys.push(pk.1);
    }
    pub fn decrypt(ew: EncryptedWallet, pass: &[u8]) -> Result<Wallet> {
        let d = aes::decrypt(&ew.privkeys, pass, &[])?;
        let pks = serde_json::from_slice(&d)?;
        let w = Wallet {
            pubkeys: ew.pubkeys,
            privkeys: pks,
        };
        Ok(w)
    }
    pub fn encrypt(self, pass: &[u8]) -> Result<EncryptedWallet> {
        let pks = serde_json::to_vec(&self.privkeys)?;
        let e = aes::encrypt(&pks, pass, &[])?;
        let ew = EncryptedWallet {
            pubkeys: self.pubkeys,
            privkeys: e,
        };
        Ok(ew)
    }
    pub fn new_keypair() -> Keypair {
        let mut rnd: OsRng = OsRng::new().unwrap();
        let mut seed = [0u8; 64];
        rnd.fill_bytes(&mut seed);
        let (a, b) = ed25519::keypair(&seed);
        assert!(cfg!(target_endian = "little"));
        let ap = unsafe { transmute::<[u8; 64], [u64; 8]>(a) };
        let bp = unsafe { transmute::<[u8; 32], [u64; 4]>(b) };
        (ap, bp)
    }
    pub fn sign(kp: Keypair, msg: &mut data::Message) {
        let sz = size_of::<data::Payload>();
        let p = &msg.pld as *const data::Payload;
        assert!(cfg!(target_endian = "little"));
        let buf = unsafe { transmute(from_raw_parts(p as *const u8, sz)) };
        let pk = unsafe { transmute::<[u64; 8], [u8; 64]>(kp.0) };
        msg.sig = ed25519::signature(buf, &pk);
    }
}
