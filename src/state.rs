use data;
use result::Result;
use hasht::Key;

#[repr(C)]
pub struct State {
    accounts: Vec<data::Account>,
    used: usize,
}

impl State {
    pub fn new(size: usize) -> State {
        State {
            accounts: vec![data::Account::default(); size],
            used: 0,
        }
    }
    fn double(&mut self) -> Result<()> {
        let size = self.accounts.len() * 2;
        let mut v = vec![data::Account::default(); size];
        data::AccountT::migrate(&self.accounts, &mut v)?;
        self.accounts = v;
        Ok(())
    }
    unsafe fn find_accounts(
        state: &[data::Account],
        fk: &[u8; 32],
        tk: &[u8; 32],
    ) -> Result<(usize, usize)> {
        let sf = data::AccountT::find(&state, fk)?;
        let st = data::AccountT::find(&state, tk)?;
        Ok((sf, st))
    }
    unsafe fn load_accounts<'a>(
        state: &'a mut [data::Account],
        (sf, st): (usize, usize),
    ) -> (&'a mut data::Account, &'a mut data::Account) {
        let ptr = state.as_mut_ptr();
        let from = ptr.offset(sf as isize).as_mut().unwrap();
        let to = ptr.offset(st as isize).as_mut().unwrap();
        (from, to)
    }

    unsafe fn exec(
        state: &mut [data::Account],
        m: &mut data::Message,
        num_new: &mut usize,
    ) -> Result<()> {
        if m.pld.kind != data::Kind::Transaction {
            return Ok(());
        }
        let pos = Self::find_accounts(state, &m.pld.data.tx.from, &m.pld.data.tx.to)?;
        let (mut from, mut to) = Self::load_accounts(state, pos);
        if from.from != m.pld.data.tx.from {
            return Ok(());
        }
        if !to.from.unused() && to.from != m.pld.data.tx.to {
            return Ok(());
        }
        Self::charge(&mut from, m);
        if m.pld.state != data::State::Withdrawn {
            return Ok(());
        }
        Self::new_account(&to, num_new);
        Self::deposit(&mut to, m);
        Ok(())
    }
    pub fn execute(&mut self, msgs: &mut [data::Message]) -> Result<()> {
        let mut num_new = 0;
        for mut m in msgs.iter_mut() {
            unsafe {
                Self::exec(&mut self.accounts, &mut m, &mut num_new)?;
            }
            self.used = num_new + self.used;
            if ((4 * (self.used)) / 3) > self.accounts.len() {
                self.double()?
            }
            num_new = 0;
        }
        Ok(())
    }
    unsafe fn charge(acc: &mut data::Account, m: &mut data::Message) -> () {
        let combined = m.pld.data.tx.amount + m.pld.data.tx.fee;
        if acc.balance >= combined {
            m.pld.state = data::State::Withdrawn;
            acc.balance = acc.balance - combined;
        }
    }
    fn new_account(to: &data::Account, num: &mut usize) -> () {
        if to.from.unused() {
            *num = *num + 1;
        }
    }
    unsafe fn deposit(to: &mut data::Account, m: &mut data::Message) -> () {
        to.balance = to.balance + m.pld.data.tx.amount;
        if to.from.unused() {
            to.from = m.pld.data.tx.to;
            assert!(!m.pld.data.tx.to.unused());
            assert!(!to.from.unused());
        }
        m.pld.state = data::State::Deposited;
    }
}

#[cfg(test)]
mod tests {
    use state::State;

    #[test]
    fn state_test() {
        let mut s: State = State::new(64);
        let mut msgs = [];
        s.execute(&mut msgs).expect("e");
    }
}

#[cfg(all(feature = "unstable", test))]
mod bench {
    extern crate test;
    use self::test::Bencher;
    use data;
    use state::State;
    use hasht::Key;

    fn init_msgs(msgs: &mut [data::Message]) {
        for (i, m) in msgs.iter_mut().enumerate() {
            m.pld.kind = data::Kind::Transaction;
            unsafe {
                m.pld.data.tx.to = [255u8; 32];
                m.pld.data.tx.to[0] = i as u8;
                m.pld.data.tx.from = [255u8; 32];
                m.pld.data.tx.fee = 1;
                m.pld.data.tx.amount = 1;
                assert!(!m.pld.data.tx.to.unused());
                assert!(!m.pld.data.tx.from.unused());
            }
        }
    }

    #[bench]
    fn state_bench(b: &mut Bencher) {
        const NUM: usize = 128usize;
        let mut s: State = State::new(NUM * 2);
        let mut msgs = [data::Message::default(); NUM];
        init_msgs(&mut msgs);
        let fp = data::AccountT::find(&s.accounts, &[255u8; 32]).expect("f");
        s.accounts[fp].from = [255u8; 32];
        b.iter(|| {
            s.accounts[fp].balance = NUM as u64 * 2u64;
            s.execute(&mut msgs).expect("execute");
            assert_eq!(s.accounts[fp].balance, 0);
        })
    }
}
