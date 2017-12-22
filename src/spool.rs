use net;
use poh;
use std::io::Result;

pub fn run() -> Result<()> {
    let srv = net::server()?;
    let mut m: [Message; 1024] = unsafe { uninitialized() };
    let mut num = 0;
    loop {
        let start = num;
        net::read(srv, &mut m[start .. ], &mut num).expect("read");
        let end = num;
        poh::verify(&mut m[start .. end], &mut num).expect("verify");
        execute(&m[start .. num], &mut num).expect("execute");
    }
    return Ok(());
}

pub fn execute(msgs: &[Message]) -> Result<()> {
    let mut order = [0usize; 1024];
    sort(msgs, &mut order);
    apply(&order, msgs);
}
