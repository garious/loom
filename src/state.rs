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
    fn lookup(&self, key: [u8; 32]) -> &mut Account {
        return &mut self.accounts[0];
    }
    fn withdrawals(&mut self, msgs: &mut [data::Message]) {
        for m in msgs {
            if m.kind != data::Kind::Transaction {
                continue;
            }
            //TODO(aey) multiple threads
            let acc = self.lookup(m.from);
            let combined = *m.amount + m.fee;
            if acc.balance > combined {
                m.state = data::State::Withdrawn;
                unsafe {
                    atomic_xsub(acc.balance.as_mut_ptr(), 
                                combined);
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
                let acc = self.lookup(m.data.tx.to);
                //TODO(aey) multiple threads
                unsafe {
                    atomic_xadd((&mut acc.balance) as *mut u64,
                                m.data.tx.amount);
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

