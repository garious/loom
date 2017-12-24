use result::{Result, Error};

pub trait HashT<T, Key: Eq> {
    fn start(&self, k: &Key) -> usize;
    fn unused(&self) -> bool;
    fn key(&self) -> &Key;

    fn find(&self, tbl: &[T], key: &Key) -> Result<usize> {
        let num_elems = tbl.len();
        let st = self.start(key);
        let offset = st % num_elems;
        for i in [offset .. offset + num_elems] {
            let pos = i % num_elems;
            unsafe {
                if tbl.get_unchecked(pos).unused() { 
                    return pos;
                }
                if tbl.get_unchecked(pos).key() == key { 
                    return pos;
                }
            }
        }
        return Err(Error::NoSpace)
    }
    fn migrate(&self, src: &[T], dst: &mut [T]) -> Result<()> {
        for i in src.iter() {
            if i.unused() {
                continue;
            }
            let p = Self::find(src, i.key())?;
            unsafe {
                *dst.get_unchecked(p) = i;
            }
        }
        return Ok(());
    }
}


