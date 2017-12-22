use data;
use core::intrinsics::{atomic_xadd, atomic_xsub};

#[repr(C)]
struct Account {
    from: [u8; 32],
    balance: u64,
}

#[repr(C)]
pub struct State<N> {
    accounts: [Account; N],
}

impl<N> State<N> {
    fn withdrawals(&mut self, msgs: &mut [data::Message]) {
        for m in msgs {
            if m.kind != data::State::Transaction {
                continue;
            }
            //TODO(aey) multiple threads
            let acc = self.lookup(msgs[order[0]].from);
            let combined = cm.amount + cm.fee;
            if self.accounts[acc].balance > combined {
                cm.state = data::State::Withdrawn;
                unsafe {
                    atomci_xsub(self.accounts[acc].balance.as_mut_ptr(), 
                                combined.as_ptr());
                }
            }
        }
    }
    fn deposits(&mut self, msgs: &mut [data::Message]) {
        for m in msgs {
            if m.state == data::State::Withdrawn {
                let acc = self.lookup(m.to);
                //TODO(aey) multiple threads
                unsafe {
                    atomci_xadd(self.accounts[acc].balance.as_mut_ptr(), 
                                m.amount.as_ptr());
                }
                cm.state = data::State::Deposited;
            }
        }
    }
}
