use data;
use core::intrinsics::{atomic_xadd, atomic_xsub};
use result::{Result};
use hasht::{HashT, KeyT};

#[derive(Default)]
#[repr(C)]
struct Account {
    from: [u8; 32],
    balance: u64,
}

impl KeyT<[u8;32]> {
    fn unused(&self) {
        return self == [0u8; 32];
    }
    fn start(&self) -> usize {
        let st = ((self[0] as u64) << ((7 - 0) * 8)) |
                 ((self[1] as u64) << ((7 - 1) * 8)) |
                 ((self[2] as u64) << ((7 - 2) * 8)) |
                 ((self[3] as u64) << ((7 - 3) * 8)) |
                 ((self[4] as u64) << ((7 - 4) * 8)) |
                 ((self[5] as u64) << ((7 - 5) * 8)) |
                 ((self[6] as u64) << ((7 - 6) * 8)) |
                 ((self[7] as u64) << ((7 - 7) * 8)) ;
        return st as usize;
    }
}
impl HashT<Account,Key=[u8;32]> {
    type Key = [u8; 32];
    fn key(&self) -> &Self::Key {
        return self.from;
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
        HashT::<Account>::migrate(self.accounts, &mut v)?;
        self.accounts = v;
        return Ok(());
    }
    pub fn execute(&mut self, msgs: &mut [data::Message]) -> Result<()> {
        let num_new = 0;
        let mut tmp = Vec::new();
        tmp.resize(msgs.len()*4, Account::default());
        Self::populate(self.accounts, msgs, &mut tmp)?;
        Self::withdrawals(tmp, msgs, &mut num_new)?;
        Self::deposits(tmp, msgs)?;
        if ((4*(num_new + self.used))/3) > self.accounts.len() {
            self.double()?
        }
        Self::apply(tmp, &mut self.accounts);
    }
    fn populate(accounts: &[Account], msgs: &[data::Message], tmp: &mut [Account]) -> Result<()> {
        for m in msgs.iter() {
            unsafe {
                let sf = HashT::<Account>::find(accounts, msgs.from)?;
                let st = HashT::<Account>::find(accounts, msgs.to)?;
                let df = HashT::<Account>::find(tmp, msgs.from)?;
                let dt = HashT::<Account>::find(tmp, msgs.to)?;
                tmp.get_unchecked(df) = accounts.get_unchecked(sf);
                tmp.get_unchecked(dt) = accounts.get_unchecked(st);
            }
        }
        return Ok(());
    }
    fn withdrawals(state: &mut [Account], msgs: &mut [data::Message], num: &mut usize) -> Result<()> {
        for m in msgs {
            if m.kind != data::Kind::Transaction {
                continue;
            }
            //TODO(aey) multiple threads
            unsafe {
                let fp = HashT::<Account>::find(state, m.data.tx.from)?;
                let combined = m.data.tx.amount + m.data.tx.fee;
                let acc = state.get_unchecked(fp);
                if acc.balance >= combined {
                    m.state = data::State::Withdrawn;
                        atomic_xsub((&mut acc.balance) as *mut u64,
                                    combined as u64);
                    let tp = HashT::<Account>::find(state, m.data.tx.to)?;
                    if state.get_unchecked(tp).from.unused() {
                        *num = *num + 1;
                    }
                }
            }
        }
        return Ok(());
    }
    fn deposits(state: &mut [Account], msgs: &mut [data::Message]) -> Result<()> {
        for m in msgs {
            if m.kind != data::Kind::Transaction {
                continue;
            }
            if m.state == data::State::Withdrawn {
                unsafe {
                    let pos = HashT::<Account>::find(state, m.data.tx.to)?;
                    let acc = state.get_unchecked(pos);
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
    fn apply(state: &[Account], accounts: &mut [Account]) -> Result<()> {
        //TODO(aey): multiple threads
        for t in state.iter() {
            unsafe {
                if t.from.unsued() {
                    continue;
                }
                let ap = HashT::<Account>::find(accounts, t.from)?;
                let acc = accounts.get_unchecked(ap);
                acc.balance = t.balance;
            }
        }
        return Ok(());
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
        s.execute(&mut msgs).expect("execute");
        assert_eq!(s.accounts[0].balance,0);
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
    s.execute(&mut msgs)?;
    //assert_eq!(ErrorKind::Other, rv.kind());
    //assert_eq!("no space", rv.description());
}
