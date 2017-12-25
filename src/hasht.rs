use result::{Result, Error};
use core::marker::PhantomData;

pub trait Key: Eq {
    fn start(&self) -> usize;
    fn unused(&self) -> bool;
}

pub trait Val<T: Key>: Sized + Clone {
    fn key(&self) -> &T;
}

pub struct HashT<K: Key, V: Val<K> > {
    _key: PhantomData<K>,
    _val: PhantomData<V>,
}

impl<K, V> HashT<K, V> 
    where K: Key,
          V: Val<K>
{
    pub fn find(tbl: &[V], key: &K) -> Result<usize> {
        let num_elems = tbl.len();
        let st = key.start();
        let offset = st % num_elems;
        for i in offset .. (offset + num_elems) {
            let pos = i % num_elems;
            unsafe {
                if tbl.get_unchecked(pos).key().unused() { 
                    return Ok(pos);
                }
                if tbl.get_unchecked(pos).key() == key { 
                    return Ok(pos);
                }
            }
        }
        return Err(Error::NoSpace)
    }
    pub fn migrate(src: &[V], dst: &mut [V]) -> Result<()> {
        for i in src.iter() {
            if i.key().unused() {
                continue;
            }
            let p = Self::find(src, i.key())?;
            unsafe {
                *dst.get_unchecked_mut(p) = (*i).clone();
            }
        }
        return Ok(());
    }

}


