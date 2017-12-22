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
        return State{accounts: Vec::with_capacity(size)};
    }
    fn lookup(&mut self, key: [u8; 32]) -> &mut Account {
        return &mut self.accounts[0];
    }
    fn withdrawals(&mut self, msgs: &mut [data::Message]) {
        for m in msgs {
            if m.kind != data::Kind::Transaction {
                continue;
            }
            //TODO(aey) multiple threads
            unsafe {
                let acc = self.lookup(m.data.tx.from);
                let combined = m.data.tx.amount + m.data.tx.fee;
                if acc.balance > combined {
                    m.state = data::State::Withdrawn;
                        atomic_xsub((&mut acc.balance) as *mut u64,
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
                unsafe {
                    let acc = self.lookup(m.data.tx.to);
                    //TODO(aey) multiple threads
                    atomic_xadd((&mut acc.balance) as *mut u64,
                                m.data.tx.amount);
                }
                m.state = data::State::Deposited;
            }
        }
    }
}

#[test]
fn state_test() {
    let mut s: State = State::new(64);
    let mut msgs = [];
    s.withdrawals(&mut msgs);
    s.deposits(&mut msgs);
}

