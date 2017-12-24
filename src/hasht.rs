use result::{Result, Error};

pub trait KeyT<T: Eq> {
    fn unused(&self) -> bool;
    fn start(&self) -> usize;
}

pub trait HashT<T> {
    type Key: KeyT<Self::Key>;
    fn key(&self) -> &Self::Key;

    fn find(tbl: &[Self], key: &Self::Key) -> Result<usize> {
        let num_elems = tbl.len();
        let st = key.start();
        let offset = st % num_elems;
        for i in [offset .. offset + num_elems] {
            let pos = i % num_elems;
            unsafe {
                if tbl.get_unchecked(pos).key().unused() { 
                    return pos;
                }
                if tbl.get_unchecked(pos).key() == key { 
                    return pos;
                }
            }
        }
        return Err(Error::NoSpace)
    }
    fn migrate(src: &[Self], dst: &mut [Self]) -> Result<()> {
        for i in src.iter() {
            if i.key().unused() {
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


