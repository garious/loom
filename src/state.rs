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
    fn key_to_hash(key: [u8; 32]) -> u64 {
        return key[0] as u64<<((7 - 0) * 8) |
               key[1] as u64<<((7 - 1) * 8) |
               key[2] as u64<<((7 - 2) * 8) |
               key[3] as u64<<((7 - 3) * 8) |
               key[4] as u64<<((7 - 4) * 8) |
               key[5] as u64<<((7 - 5) * 8) |
               key[6] as u64<<((7 - 6) * 8) |
               key[7] as u64<<((7 - 7) * 8) ;
    }
    fn lookup(&mut self, key: [u8; 32]) -> &mut Account {
        let cap = self.accounts.capacity();
        let hash = State::key_to_hash(key) as usize;
        let ix = hash % cap;
        for a in self.accounts[ix .. ].iter_mut() {
            let zz = [0u8: 32];
            if a.from == key {
                return &mut a;
            }
            if a.from == zz {
                return &mut a;
            }
        }
        for a in self.accounts[0 .. ix].iter_mut() {
            let zz = [0u8: 32];
            if a.from == key {
                return &mut a;
            }
            if a.from == zz {
                return &mut a;
            }
        }
        //TODO(aey): handle no space
        assert!(false);
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

