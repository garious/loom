#![allow(mutable_transmutes)]
use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};
use result::Result;
use std::net::UdpSocket;
use std::mem::transmute;
use std::mem::size_of;
use std::slice::from_raw_parts;
use data;

pub struct Ledger {
    file: File,
}

//TODO(aeyakovenko): config file somewhere
const LEDGER: &str = "./loom.ledger";

impl Ledger {
    pub fn new() -> Result<Ledger> {
        let file = File::open(LEDGER)?;
        Ok(Ledger { file })
    }
    fn get_ledger(&self, sock: &UdpSocket, get: &data::GetLedger) -> Result<()> {
        let mut mem = Vec::new();
        mem.resize(get.num as usize, data::Message::default());
        Self::load(&mut mem, get.start)?;
        let p = &mem[0] as *const data::Message;
        let sz = size_of::<data::Message>();
        let bz = mem.len() * sz;
        assert!(cfg!(target_endian = "little"));
        let buf = unsafe { transmute(from_raw_parts(p as *const u8, bz)) };
        sock.send(buf)?;
        Ok(())
    }
    fn exec(&self, sock: &UdpSocket, m: &data::Message) -> Result<()> {
        if let data::Kind::GetLedger = m.pld.kind {
            let get = &m.pld.get_get();
            self.get_ledger(sock, get)?;
        }
        Ok(())
    }
    pub fn execute(&self, sock: &UdpSocket, msgs: &mut [data::Message]) -> Result<()> {
        for m in msgs {
            self.exec(sock, &m)?;
        }
        Ok(())
    }
    pub fn append(&mut self, msgs: &[data::Message]) -> Result<()> {
        //TODO(aeyakovenko): the fastest way to do this:
        // have the msgs memory be mmaped
        // then `splice` the mmap fd into the file fd
        let p = &msgs[0] as *const data::Message;
        let sz = size_of::<data::Message>();
        let bz = msgs.len() * sz;
        assert!(cfg!(target_endian = "little"));
        let buf = unsafe { transmute(from_raw_parts(p as *const u8, bz)) };
        self.file.write_all(buf)?;
        Ok(())
    }
    pub fn load(msgs: &mut [data::Message], start: u64) -> Result<()> {
        //TODO(aeyakovenko): the fastest way to do this:
        // have the msgs memory be mmaped
        // then `splice` from mmap fd, or to the socket directly
        let mut file = File::open(LEDGER)?;
        let p = &mut msgs[0] as *mut data::Message;
        let sz = size_of::<data::Message>();
        file.seek(SeekFrom::Start(sz as u64 * start))?;
        let bz = msgs.len() * sz;
        assert!(cfg!(target_endian = "little"));
        let buf = unsafe { transmute(from_raw_parts(p as *mut u8, bz)) };
        file.read(buf)?;
        Ok(())
    }
}
