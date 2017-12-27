use net;
use state;
use data;
use result::Result;
use std::mem::uninitialized;

pub fn run() -> Result<()> {
    let srv = net::server()?;
    let mut s = state::State::new(1024);
    let mut m: [data::Message; 1024] = unsafe { uninitialized() };
    let mut num = 0;
    loop {
        //TODO(aey): read/execute on separate threads
        let start = num;
        net::read(&srv, &mut m[start .. ], &mut num).expect("read");
        let end = num;
        s.execute(&mut m[start .. end]).expect("execute");
    }
}
