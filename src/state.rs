use data;
use core::intrinsics::{atomic_xadd, atomic_xsub};
use result::{Result};
use hasht::{HashT, find};

#[derive(Default)]
#[repr(C)]
struct Account {
    from: [u8; 32],
    balance: u64,
}

impl HashT<Account> {
    type Key = [u8; 32];
    fn key(&self) -> &Key {
        return self.from;
    }
    fn unused(&self) {
        return self.from == [0u8; 32];
    }
    fn start(key: &Key) -> usize {
        let st = ((key[0] as u64) << ((7 - 0) * 8)) |
                 ((key[1] as u64) << ((7 - 1) * 8)) |
                 ((key[2] as u64) << ((7 - 2) * 8)) |
                 ((key[3] as u64) << ((7 - 3) * 8)) |
                 ((key[4] as u64) << ((7 - 4) * 8)) |
                 ((key[5] as u64) << ((7 - 5) * 8)) |
                 ((key[6] as u64) << ((7 - 6) * 8)) |
                 ((key[7] as u64) << ((7 - 7) * 8)) ;
        return st as usize;
    }

}

#[repr(C)]
pub struct State {
    accounts: Vec<Account>,
    used: usize,
}

impl State {
    pub fn new(size: usize) -> State {
        let mut v = Vec::new();
        v.resize(size, Account::default());
        return State{accounts: v, used: 0};
    }

    pub fn double(&mut self) -> Result<()> {
        let mut v = Vec::new();
        let size = self.accounts.len()*2;
        v.resize(size, Account::default());
        HashT<Account>::migrate(self.accounts, &mut v)?;
        self.accounts = v;
        return Ok(());
    }

    pub fn populate(&mut self, msgs: &[data::Message], tmp: &mut [Account]) -> Result<()> {
        for m in msgs {
            unsafe {
                let sf = HashT<Account>::find(self.accounts, msgs.from)?;
                let st = HashT<Account>::find(self.accounts, msgs.to)?;
                let df = HashT<Account>::find(tmp, msgs.from)?;
                let dt = HashT<Account>::find(tmp, msgs.to)?;
                tmp.get_unchecked(df) = self.accounts.get_unchecked(sf);
                tmp.get_unchecked(dt) = self.accounts.get_unchecked(st);
            }
        }
    }

    pub fn execute(&mut self, msgs: &mut [data::Message]) -> Result<()> {
        let num_new = 0;
        let mut tmp = Vec::new();
        tmp.resize(msgs.len()*2, Account::default());
        self.populate(state, msgs, &mut tmp);
        self.withdrawals(state, msgs, &mut num_new);
        if ((4*(num_new + self.used))/3) > self.accounts.len() {
            self.double()?
        }
        self.new_dests(state, msgs, &num);

        self.deposits(msgs);
    }

    pub fn withdrawals(state: &mut [Account], msgs: &mut [data::Message]) -> Result<()> {
        for m in msgs {
            if m.kind != data::Kind::Transaction {
                continue;
            }
            //TODO(aey) multiple threads
            unsafe {
                let fp = HashT<Account>::find(m.data.tx.from)?;
                let combined = m.data.tx.amount + m.data.tx.fee;
                let acc = state.get_unchecked(fp);
                if acc.balance >= combined {
                    m.state = data::State::Withdrawn;
                        atomic_xsub((&mut acc.balance) as *mut u64,
                                    combined);
                    let tp = HashT<Account>::find(m.data.tx.to)?;
                    if state.get_unchecked(tp).unused() {
                        *num = *num + 1;
                    }
                }

            }
        }
        return Ok(());
    }
    pub fn deposits(&mut self, msgs: &mut [data::Message]) {
        for m in msgs {
            if m.kind != data::Kind::Transaction {
                continue;
            }
            if m.state == data::State::Withdrawn {
                unsafe {
                    let acc = self.lookup(m.data.tx.to).expect("failed to find to address");
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
    }
}

#[cfg(test)]
use test::Bencher;

#[test]
fn state_test() {
    let mut s: State = State::new(64);
    let mut msgs = [];
    s.withdrawals(&mut msgs).expect("withdrawals");
    s.deposits(&mut msgs).expect("deposits");
}

#[bench]
fn state_test2(b: &mut Bencher) {
    let mut s: State = State::new(64);
    let mut msgs = [data::Message::default(); 64];
    for (i,m) in msgs.iter_mut().enumerate() {
        m.kind = data::Kind::Transaction;
        unsafe {
            m.data.tx.to = [0u8; 32];
            m.data.tx.to[7] = i as u8;
            m.data.tx.from = [0u8; 32];
            m.data.tx.fee = 1;
            m.data.tx.amount = 1;
        }
    }
    b.iter(|| {
        s.accounts[0].balance = 128;
        s.withdrawals(&mut msgs).expect("withdrawals");
        assert_eq!(s.accounts[0].balance,0);
        s.deposits(&mut msgs).expect("deposits");
        assert_eq!(s.accounts[0].balance,1);
    })
}

#[test]
fn state_test3() {
    let mut s: State = State::new(2);
    let mut msgs = [data::Message::default(); 3];
    for (i,m) in msgs.iter_mut().enumerate() {
        m.kind = data::Kind::Transaction;
        unsafe {
            m.data.tx.to = [0u8; 32];
            m.data.tx.to[7] = i as u8;
            m.data.tx.from = [0u8; 32];
            m.data.tx.fee = 1;
            m.data.tx.amount = 1;
        }
    }
    s.accounts[0].balance = 128;
    s.withdrawals(&mut msgs)?;
    s.deposits(&mut msgs)?;
    assert_eq!(ErrorKind::Other, rv.kind());
    //assert_eq!("no space", rv.description());
}
