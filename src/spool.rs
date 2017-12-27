use net;
use state;
use std::io::Result;

pub fn run() -> Result<()> {
    let srv = net::server()?;
    let s = state::new();
    let mut m: [Message; 1024] = unsafe { uninitialized() };
    let mut num = 0;
    loop {
        //TODO(aey): read/execute on separate threads
        let start = num;
        net::read(srv, &mut m[start .. ], &mut num).expect("read");
        let end = num;
        s.execute(&mut m[start .. end]).expect("execute");
    }
    return Ok(());
}
