use std::fs::File;
use result::Result;
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
    }
}
