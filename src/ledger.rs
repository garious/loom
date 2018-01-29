use std::fs::File;
use result::Result;
use std::mem::transmute;
use std::mem::size_of;
use std::slice::from_raw_parts;
use data;

struct Ledger {
    file: File;
};

impl Ledger {
    fn new(): Result<Ledger> {
        let mut file = File::open(path)?;
        let mut l = Ledger{file: file};
        return l;
    }
        
    fn append(msgs: &[data::Message]) {
        //TODO(aeyakovenko): the fastest way to do this:
        // have the msgs memory be mmaped
        // then `splice` the mmap fd into the file fd
        let p = msgs as *const Message;
        let sz = size_of::<Message>();
        let bz = msgs.len() * sz;
        let buf = unsafe {
            transmute(from_raw_parts(p as *const u8, bz))
        };
        file.write_all(&buf);
    }
}
