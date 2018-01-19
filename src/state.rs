use data;
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
        //TODO(aey): *self == [0u8; 32] should work
        for i in self.iter() {
            if *i != 0 {
                return false;
            }
        }
        return true;
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

impl State {
    pub fn new(size: usize) -> State {
        let mut v = Vec::new();
        v.clear();
        v.resize(size, Account::default());
        for i in v.iter() {
            if i.from.unused() {
                assert!(i.balance == 0);
            }
        }
        let t = Vec::new();
        let rv = State{accounts: v, tmp: t, used: 0};
        for v in rv.accounts.iter() {
            if v.from.unused() {
                assert!(v.balance == 0);
            }
        }
        return rv;
    }
    pub fn double(&mut self) -> Result<()> {
        let mut v = Vec::new();
        let size = self.accounts.len()*2;
        v.resize(size, Account::default());
        AccountT::migrate(&self.accounts, &mut v)?;
        self.accounts = v;
        return Ok(());
    }
    pub fn execute(&mut self, msgs: &mut [data::Message]) -> Result<()> {
        let mut num_new = 0;
        self.tmp.clear();
        self.tmp.resize(msgs.len()*4, Account::default());
        Self::populate(&self.accounts, msgs, &mut self.tmp)?;
        Self::charges(&mut self.tmp, msgs)?;
        Self::new_accounts(&mut self.tmp, msgs, &mut num_new)?;
        Self::deposits(&mut self.tmp, msgs)?;
        if ((4*(num_new + self.used))/3) > self.accounts.len() {
            self.double()?
        }
        return Self::apply(&self.tmp, &mut self.accounts);
    }
    fn populate(accounts: &[Account], msgs: &[data::Message], tmp: &mut [Account]) -> Result<()> {
        for m in msgs.iter() {
            unsafe {
                let sf = AccountT::find(accounts, &m.data.tx.from)?;
                let df = AccountT::find(tmp, &m.data.tx.from)?;
                *tmp.get_unchecked_mut(df) = *accounts.get_unchecked(sf);

                let st = AccountT::find(accounts, &m.data.tx.to)?;
                let dt = AccountT::find(tmp, &m.data.tx.to)?;
                *tmp.get_unchecked_mut(dt) = *accounts.get_unchecked(st);
            }
        }
        return Ok(());
    }
    fn charges(state: &mut [Account], msgs: &mut [data::Message]) -> Result<()> {
        for m in msgs {
            if m.kind != data::Kind::Transaction {
                continue;
            }
            //TODO(aey) multiple threads
            unsafe {
                let fp = AccountT::find(state, &m.data.tx.from)?;
                let combined = m.data.tx.amount + m.data.tx.fee;
                let mut acc = state.get_unchecked_mut(fp);
                if acc.from != m.data.tx.from {
                    continue;
                }
                if acc.balance >= combined {
                    m.state = data::State::Withdrawn;
                    acc.balance = acc.balance - combined;
                }
            }
        }
        return Ok(());
    }
    fn new_accounts(state: &mut [Account], msgs: &mut [data::Message], num: &mut usize) -> Result<()> {
        for m in msgs {
            if m.kind != data::Kind::Transaction {
                continue;
            }
            //TODO(aey) multiple threads
            unsafe {
                if m.state == data::State::Withdrawn {
                    let tp = AccountT::find(state, &m.data.tx.to)?;
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
                    let pos = AccountT::find(state, &m.data.tx.to)?;
                    let mut acc = state.get_unchecked_mut(pos);
                    //TODO(aey) multiple threads
                    acc.balance = acc.balance + m.data.tx.amount;
                    if acc.from.unused() {
                        acc.from = m.data.tx.to;
                        assert!(m.data.tx.to.unused() == false);
                        assert!(acc.from.unused() == false);
                    }
                }
                m.state = data::State::Deposited;
            }
        }
        return Ok(());
    }
    fn apply(state: &[Account], accounts: &mut [Account]) -> Result<()> {
        //TODO(aey): multiple threads
        for t in state.iter() {
            unsafe {
                if t.from.unused() {
                    assert!(t.balance == 0);
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
    s.execute(&mut msgs).expect("e");
}

#[test]
fn populate_test() {
    const NUM: usize = 2usize;
    let mut s: State = State::new(NUM);
    let mut msgs = [data::Message::default(); NUM];
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
    s.tmp.clear();
    s.tmp.resize(msgs.len()*4, Account::default());
    State::populate(&s.accounts, &msgs, &mut s.tmp).expect("p");
    for i in s.tmp {
        assert!(i.balance == 0);
        assert!(i.key().unused());
    }
}

#[test]
fn populate_test2() {
    const NUM: usize = 2usize;
    let mut s: State = State::new(NUM*2);
    let mut msgs = [data::Message::default(); NUM];
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
    s.tmp.clear();
    s.tmp.resize(msgs.len()*4, Account::default());
    State::populate(&s.accounts, &msgs, &mut s.tmp).expect("p");
    for m in msgs.iter() {
        unsafe {
            let p = AccountT::find(&s.accounts, &m.data.tx.to).expect("f");
            s.accounts[p].from = m.data.tx.to;
            let np = AccountT::find(&s.accounts, &m.data.tx.to).expect("f");
            assert_eq!(np, p);
        }
    }
    State::populate(&s.accounts, &msgs, &mut s.tmp).expect("p");
    for m in msgs.iter() {
        unsafe {
            let p = AccountT::find(&s.tmp, &m.data.tx.to).expect("f");
            assert_eq!(s.tmp[p].from, m.data.tx.to);
        }
    }
}

#[test]
fn charge_test() {
    const NUM: usize = 2usize;
    let mut s: State = State::new(NUM*2);
    let mut msgs = [data::Message::default(); NUM];
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
    s.tmp.clear();
    s.tmp.resize(msgs.len()*4, Account::default());

    let p = AccountT::find(&s.accounts, &[255u8;32]).expect("f");
    s.accounts[p].from = [255u8;32];
    s.accounts[p].balance = (NUM*2) as u64;

    for m in msgs.iter() {
        unsafe {
            let p = AccountT::find(&s.accounts, &m.data.tx.to).expect("f");
            s.accounts[p].from = m.data.tx.to;
        }
    }

    State::populate(&s.accounts, &msgs, &mut s.tmp).expect("p");
    for m in msgs.iter() {
        unsafe {
            let p = AccountT::find(&s.tmp, &m.data.tx.to).expect("f");
            assert_eq!(s.tmp[p].from, m.data.tx.to);
        }
    }
    State::charges(&mut s.tmp, &mut msgs).expect("c");
    for m in msgs.iter() {
        assert!(m.state == data::State::Withdrawn);
    }
}

#[test]
fn new_accounts_test() {
    const NUM: usize = 2usize;
    let mut s: State = State::new(NUM*2);
    let mut msgs = [data::Message::default(); NUM];
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
    s.tmp.clear();
    s.tmp.resize(msgs.len()*4, Account::default());

    let p = AccountT::find(&s.accounts, &[255u8;32]).expect("f");
    s.accounts[p].from = [255u8;32];
    s.accounts[p].balance = (NUM*2) as u64;

    State::populate(&s.accounts, &msgs, &mut s.tmp).expect("p");
    let mut num = 0usize;
    for m in msgs.iter_mut() {
        m.state = data::State::Withdrawn;
    }
    State::new_accounts(&mut s.tmp, &mut msgs, &mut num).expect("c");
    assert_eq!(num, NUM); 
}



#[bench]
fn state_test2(b: &mut Bencher) {
    const NUM: usize = 2usize;
    let mut s: State = State::new(NUM*2);
    let mut msgs = [data::Message::default(); NUM];
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
    let fp = AccountT::find(&s.accounts, &[255u8; 32]).expect("f");
    s.accounts[fp].from = [255u8;32];
    b.iter(|| {
        s.accounts[fp].balance = 128;
        s.execute(&mut msgs).expect("execute");
        assert_eq!(s.accounts[fp].balance,0);
    })
}
