#![allow(mutable_transmutes)]
use std::fs::File;
use std::io::{Seek, SeekFrom, Write, Read};
use result::Result;
use std::mem::transmute;
use std::mem::size_of;
use std::slice::from_raw_parts;
use data;

pub struct Ledger {
    file: File,
}

const LEDGER: &str = "./loom.ledger";

impl Ledger {
    pub fn new() -> Result<Ledger> {
        let file = File::open(LEDGER)?;
        let l = Ledger{file: file};
        return Ok(l);
    }
    pub fn append(&mut self, msgs: &[data::Message]) -> Result<()> {
        //TODO(aeyakovenko): the fastest way to do this:
        // have the msgs memory be mmaped
        // then `splice` the mmap fd into the file fd
        let p = &msgs[0] as *const data::Message;
        let sz = size_of::<data::Message>();
        let bz = msgs.len() * sz;
        let buf = unsafe {
            transmute(from_raw_parts(p as *const u8, bz))
        };
        self.file.write_all(buf)?;
        return Ok(());
    }
    pub fn read(&mut self, msgs: &mut [data::Message], start: u64) -> Result<()> {
        //TODO(aeyakovenko): the fastest way to do this:
        // have the msgs memory be mmaped
        // then `splice` from mmap fd
        let mut file = File::open(LEDGER)?;
        let p = &mut msgs[0] as *mut data::Message;
        let sz = size_of::<data::Message>();
        file.seek(SeekFrom::Start(sz as u64 * start))?;
        let bz = msgs.len() * sz;
        let buf = unsafe {
            transmute(from_raw_parts(p as *mut u8, bz))
        };
        self.file.read(buf)?;
        return Ok(());
    }
}
