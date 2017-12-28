use data;
use core::intrinsics::{atomic_xadd, atomic_xsub};
use result::{Result};
use hasht::{HashT, Key, Val};

#[derive(Default, Copy, Clone)]
#[repr(C)]
struct Account {
    from: [u8; 32],
    balance: u64,
}

impl Key for [u8; 32] {
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
 
    fn unused(&self) -> bool {
        *self == [0u8; 32]
    }
}

impl Val<[u8;32]> for Account {
    fn key(&self) -> &[u8;32] {
        return &self.from;
    }
}
type AccountT = HashT<[u8;32], Account>;

#[repr(C)]
pub struct State {
    accounts: Vec<Account>,
    tmp: Vec<Account>,
    used: usize,
}

struct Loc {
    tacc: usize,
    facc: usize,
    ttmp: usize,
    ftmp: usize,
}


impl State {
    pub fn new(size: usize) -> State {
        let mut v = Vec::new();
        v.resize(size, Account::default());
        let t = Vec::new();
        return State{accounts: v, tmp: t, used: 0};
    }
    fn double(&mut self) -> Result<()> {
        let mut v = Vec::new();
        let size = self.accounts.len()*2;
        v.resize(size, Account::default());
        AccountT::migrate(&self.accounts, &mut v)?;
        self.accounts = v;
        return Ok(());
    }
    pub fn execute(&mut self, msgs: &mut [data::Message]) -> Result<()> {
        //TODO(aey): this is slow, make it fast, we should just do one scan
        let mut num_new = 0;
        self.tmp.clear();
        self.tmp.resize(msgs.len()*4, Account::default());
        for m in msgs.iter_mut() {
            //TODO(aey) multiple threads
            if m.kind != data::Kind::Transaction {
                continue;
            }
            unsafe {
                let fa = AccountT::find(&self.accounts, &m.data.tx.from)?;
                let ta = AccountT::find(&self.accounts, &m.data.tx.to)?;
                let ft = AccountT::find(&self.tmp, &m.data.tx.from)?;
                let tt = AccountT::find(&self.tmp, &m.data.tx.to)?;
                let l = Loc{facc:fa, tacc:ta, ftmp:ft, ttmp:tt};
                Self::populate(&self.accounts, &l, &mut self.tmp);
                Self::withdrawals(&mut self.tmp, &mut *m, &l);
                Self::new_accounts(&mut self.tmp, &mut *m, &l, &mut num_new);
                Self::deposits(&mut self.tmp, &mut *m, &l);
            }
        }
        if ((4*(num_new + self.used))/3) > self.accounts.len() {
            self.double()?
        }
        return Self::apply(&self.tmp, &mut self.accounts);
    }
    unsafe fn populate(accounts: &[Account], l: &Loc, tmp: &mut [Account]) {
        *tmp.get_unchecked_mut(l.ftmp) = *accounts.get_unchecked(l.facc);
        *tmp.get_unchecked_mut(l.ttmp) = *accounts.get_unchecked(l.tacc);
    }
    unsafe fn withdrawals(state: &mut [Account], m: &mut data::Message, l: &Loc) {
       let combined = m.data.tx.amount + m.data.tx.fee;
       let acc = state.get_unchecked_mut(l.ftmp);
       if acc.balance >= combined {
           m.state = data::State::Withdrawn;
               atomic_xsub((&mut acc.balance) as *mut u64,
                           combined as u64);
       }
    }
    fn new_accounts(state: &mut [Account], m: &mut data::Message, l: &Loc, num: &mut usize) {
        unsafe {
            if m.state == data::State::Withdrawn {
                if state.get_unchecked(l.ttmp).from.unused() {
                    *num = *num + 1;
                }
            }
        }
    }

    fn deposits(state: &mut [Account], m: &mut data::Message, l: &Loc) {
        if m.state == data::State::Withdrawn {
            unsafe {
                let acc = state.get_unchecked_mut(l.ttmp);
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
    fn apply(state: &[Account], accounts: &mut [Account]) -> Result<()> {
        //TODO(aey): multiple threads
        for t in state.iter() {
            unsafe {
                if t.from.unused() {
                    continue;
                }
                let ap = AccountT::find(accounts, &t.from)?;
                let mut acc = accounts.get_unchecked_mut(ap);
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
    s.execute(&mut msgs).expect("execute");
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
