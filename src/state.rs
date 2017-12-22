use data;
use core::intrinsics::{atomic_xadd, atomic_xsub};
use std::io::Result;
use std::io::Error;
use std::io::ErrorKind;

#[repr(C)]
struct Account {
    from: [u8; 32],
    balance: u64,
}

#[repr(C)]
pub struct State {
    accounts: Vec<Account>,
}

impl State {
    pub fn new(size: usize) -> State {
        return State{accounts: Vec::with_capacity(size)};
    }
    fn key_to_hash(key: [u8; 32]) -> u64 {
        return ((key[0] as u64) << ((7 - 0) * 8)) |
               ((key[1] as u64) << ((7 - 1) * 8)) |
               ((key[2] as u64) << ((7 - 2) * 8)) |
               ((key[3] as u64) << ((7 - 3) * 8)) |
               ((key[4] as u64) << ((7 - 4) * 8)) |
               ((key[5] as u64) << ((7 - 5) * 8)) |
               ((key[6] as u64) << ((7 - 6) * 8)) |
               ((key[7] as u64) << ((7 - 7) * 8)) ;
    }
    fn lookup(&mut self, key: [u8; 32]) -> Result<&mut Account> {
        let cap = self.accounts.capacity();
        let hash = State::key_to_hash(key) as usize;
        let ix = hash % cap;
        let x_ptr = self.accounts.as_mut_ptr();
        unsafe {
            for i in ix .. cap {
                let a = x_ptr.offset(i as isize);
                if (*a).from == key {
                    return Ok(&mut (*a));
                }
                if (*a).from == [0u8;32] {
                    return Ok(&mut (*a));
                }
            }
            return Err(Error::new(ErrorKind::Other, "oh no!"));
        }
    }

    pub fn withdrawals(&mut self, msgs: &mut [data::Message]) -> Result<()> {
        for m in msgs {
            if m.kind != data::Kind::Transaction {
                continue;
            }
            //TODO(aey) multiple threads
            unsafe {
                let acc = self.lookup(m.data.tx.from)?;
                let combined = m.data.tx.amount + m.data.tx.fee;
                if acc.balance > combined {
                    m.state = data::State::Withdrawn;
                        atomic_xsub((&mut acc.balance) as *mut u64,
                                    combined);
                }
            }
        }
        return Ok(());
    }
    pub fn deposits(&mut self, msgs: &mut [data::Message]) -> Result<()> {
        for m in msgs {
            if m.kind != data::Kind::Transaction {
                continue;
            }
            if m.state == data::State::Withdrawn {
                unsafe {
                    let acc = self.lookup(m.data.tx.to)?;
                    //TODO(aey) multiple threads
                    atomic_xadd((&mut acc.balance) as *mut u64,
                                m.data.tx.amount);
                    if acc.from == [0u8;32] {
                        acc.from = m.data.tx.to;
                    }
                }
                m.state = data::State::Deposited;
            }
        }
        return Ok(());
    }
}

#[test]
fn state_test() {
    let mut s: State = State::new(64);
    let mut msgs = [];
    s.withdrawals(&mut msgs).expect("withdrawals");
    s.deposits(&mut msgs).expect("deposits");
}

