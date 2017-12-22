use data;
use core::intrinsics::{atomic_xadd, atomic_xsub};

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
    fn new(size: usize) -> State {
        return State{accounts: Vec::new(size)};
    }
    fn lookup(&self, key: [u8; 32]) -> Account {
        return self.accounts[0];
    }
    fn withdrawals(&mut self, msgs: &mut [data::Message]) {
        for m in msgs {
            if m.kind != data::Kind::Transaction {
                continue;
            }
            //TODO(aey) multiple threads
            let acc = self.lookup(m.from);
            let combined = *m.amount + m.fee;
            if self.accounts[acc].balance > combined {
                m.state = data::State::Withdrawn;
                unsafe {
                    atomic_xsub(self.accounts[acc].balance.as_mut_ptr(), 
                                combined.as_ptr());
                }
            }
        }
    }
    fn deposits(&mut self, msgs: &mut [data::Message]) {
        for m in msgs {
            if m.kind != data::Kind::Transaction {
                continue;
            }
            if m.state == data::State::Withdrawn {
                let acc = self.lookup(m.to);
                //TODO(aey) multiple threads
                unsafe {
                    atomic_xadd((&mut self.accounts[acc].balance) as *mut u64,
                                (&m.data.tx.amount) as *const u64);
                }
                m.state = data::State::Deposited;
            }
        }
    }
}

#[cfg(test)]
use std::mem::uninitialized;

#[test]
fn state_test() {
    let mut s: State = State::new(64);
}

