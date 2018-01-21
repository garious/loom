use data;
use result::{Result};
use hasht::{HashT, Key, Val};

#[derive(Default, Copy, Clone)]
#[repr(C)]
struct Account {
    from: [u8; 32],
    balance: u64,
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
    used: usize,
}

impl State {
    pub fn new(size: usize) -> State {
        let mut v = Vec::new();
        v.clear();
        v.resize(size, Account::default());
        return State{accounts: v, used: 0};
    }
    fn double(&mut self) -> Result<()> {
        let mut v = Vec::new();
        let size = self.accounts.len()*2;
        v.resize(size, Account::default());
        AccountT::migrate(&self.accounts, &mut v)?;
        self.accounts = v;
        return Ok(());
    }
    unsafe fn find_accounts(state: &[Account],
                            fk: &[u8;32], tk: &[u8;32]) 
        -> Result<(usize, usize)> 
    {
        let sf = AccountT::find(&state, fk)?;
        let st = AccountT::find(&state, tk)?;
        return Ok((sf, st));
    }
    unsafe fn load_accounts<'a>(state: &'a mut [Account],
                                (sf, st): (usize, usize))
        -> (&'a mut Account, &'a mut Account) 
    {
        let ptr = state.as_mut_ptr();
        let from = ptr.offset(sf as isize).as_mut().unwrap();
        let to = ptr.offset(st as isize).as_mut().unwrap();
        return (from, to);
    }

    unsafe fn exec(state: &mut [Account],
                   m: &mut data::Message,
                   num_new: &mut usize)
        -> Result<()>
   {
        if m.kind != data::Kind::Transaction {
            return Ok(());
        }
        let pos = Self::find_accounts(state,
                                      &m.data.tx.from,
                                      &m.data.tx.to)?;
        let (mut from, mut to) = Self::load_accounts(state, pos);
        if from.from != m.data.tx.from {
            return Ok(());
        }
        if to.from.unused() != true && to.from != m.data.tx.to {
            return Ok(());
        }
        Self::charge(&mut from, m);
        if m.state != data::State::Withdrawn {
            return Ok(());
        }
        Self::new_account(&to, num_new);
        Self::deposit(&mut to, m);
        return Ok(());
    }
    pub fn execute(&mut self, msgs: &mut [data::Message]) -> Result<()> {
        let mut num_new = 0;
        for mut m in msgs.iter_mut() {
            unsafe {
                Self::exec(&mut self.accounts, &mut m, &mut num_new)?;
            }
            if ((4*(num_new + self.used))/3) > self.accounts.len() {
                self.double()?
            }
            self.used = num_new + self.used;
            num_new = 0;
        }
        return Ok(());
    }
    unsafe fn charge(acc: &mut Account,
                     m: &mut data::Message) -> () {
            let combined = m.data.tx.amount + m.data.tx.fee;
            if acc.balance >= combined {
                m.state = data::State::Withdrawn;
                acc.balance = acc.balance - combined;
            }
    }
    fn new_account(to: &Account,
                   num: &mut usize) -> () {
        if to.from.unused() {
            *num = *num + 1;
        }
    }
    unsafe fn deposit(to: &mut Account, m: &mut data::Message) -> () {
        to.balance = to.balance + m.data.tx.amount;
        if to.from.unused() {
            to.from = m.data.tx.to;
            assert!(m.data.tx.to.unused() == false);
            assert!(to.from.unused() == false);
        }
        m.state = data::State::Deposited;
    }

}

#[cfg(test)]
use test::Bencher;

#[test]
fn state_test() {
    let mut s: State = State::new(64);
    let mut msgs = [];
    s.execute(&mut msgs).expect("e");
}

#[cfg(test)]
fn init_msgs(msgs: &mut [data::Message]) {
    for (i,m) in msgs.iter_mut().enumerate() {
        m.kind = data::Kind::Transaction;
        unsafe {
            m.data.tx.to = [255u8; 32];
            m.data.tx.to[0] = i as u8;
            m.data.tx.from = [255u8; 32];
            m.data.tx.fee = 1;
            m.data.tx.amount = 1;
            assert!(m.data.tx.to.unused() == false);
            assert!(m.data.tx.from.unused() == false);
        }
    }
}
#[bench]
fn state_test2(b: &mut Bencher) {
    const NUM: usize = 128usize;
    let mut s: State = State::new(NUM*2);
    let mut msgs = [data::Message::default(); NUM];
    init_msgs(&mut msgs);
    let fp = AccountT::find(&s.accounts, &[255u8; 32]).expect("f");
    s.accounts[fp].from = [255u8;32];
    b.iter(|| {
        s.accounts[fp].balance = NUM as u64 * 2u64;
        s.execute(&mut msgs).expect("execute");
        assert_eq!(s.accounts[fp].balance,0);
    })
}
