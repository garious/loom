use std::sync::{Arc, Mutex};
use std::collections::VecDeque;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, UdpSocket};
use result::{from_option, Result};
use result::Error::IO;
use std::time::Duration;
use data;
use net;

pub struct Messages {
    msgs: Vec<data::Message>,
    data: Vec<(usize, SocketAddr)>,
}

impl Messages {
    fn new() -> Messages {
        let mut m = Vec::new();
        m.resize(1024, data::Message::default());
        let mut d = Vec::new();
        d.resize(1024, Self::def_data());
        Messages { msgs: m, data: d }
    }
    fn def_data() -> (usize, SocketAddr) {
        let ipv4 = Ipv4Addr::new(0, 0, 0, 0);
        let addr = SocketAddr::new(IpAddr::V4(ipv4), 0);
        let df = (0, addr);
        return df;
    }
}

pub type SharedMessages = Arc<Messages>;

struct Data {
    pending: VecDeque<SharedMessages>,
    gc: Vec<SharedMessages>,
}
pub struct Reader {
    lock: Mutex<Data>,
    sock: UdpSocket,
}
impl Reader {
    pub fn new(port: u16) -> Result<Reader> {
        let d = Data {
            gc: Vec::new(),
            pending: VecDeque::new(),
        };

        let ipv4 = Ipv4Addr::new(0, 0, 0, 0);
        let addr = SocketAddr::new(IpAddr::V4(ipv4), port);
        let srv = UdpSocket::bind(&addr)?;
        let timer = Duration::new(0, 500000000);
        srv.set_read_timeout(Some(timer))?;
        let rv = Reader {
            lock: Mutex::new(d),
            sock: srv,
        };
        return Ok(rv);
    }
    pub fn next(&self) -> Result<SharedMessages> {
        let mut d = self.lock.lock().expect("lock");
        let o = d.pending.pop_front();
        return from_option(o);
    }
    pub fn recycle(&self, m: SharedMessages) {
        let mut d = self.lock.lock().expect("lock");
        d.gc.push(m);
    }
    pub fn run(&self, exit: Arc<Mutex<bool>>) -> Result<()> {
        loop {
            let mut m = self.allocate();
            {
                let v = Arc::get_mut(&mut m).expect("only ref");
                v.msgs.resize(1024, data::Message::default());
                v.data.resize(1024, Messages::def_data());
                let mut total = 0usize;
                while total == 0usize {
                    let r = net::read_from(&self.sock, &mut v.msgs, &mut v.data);
                    match r {
                        Err(IO(e)) => {
                            warn!("failed with IO error {:?}", e);
                        }
                        Err(e) => {
                            warn!("read failed error {:?}", e);
                        }
                        Ok(0) => {
                            info!("read returned 0");
                        }
                        Ok(num) => {
                            let s: usize = v.data.iter_mut().map(|v| v.0).sum();
                            total += s;
                            v.msgs.resize(s, data::Message::default());
                            v.data.resize(num, Messages::def_data());
                        }
                    }
                    let e = exit.lock().expect("lock");
                    if *e == true {
                        return Ok(());
                    }
                }
            }
            let c = Arc::clone(&m);
            self.enqueue(c);
            self.notify();
        }
    }
    fn notify(&self) {
        //TODO(anatoly), hard code other threads to notify
    }
    fn allocate(&self) -> SharedMessages {
        let mut s = self.lock.lock().expect("lock");
        return match s.gc.pop() {
            Some(v) => v,
            _ => Arc::new(Messages::new()),
        };
    }
    fn enqueue(&self, m: SharedMessages) {
        let mut s = self.lock.lock().expect("lock");
        s.pending.push_back(m);
    }
}

#[cfg(test)]
use std::thread::spawn;
#[cfg(test)]
use std::thread::sleep;

#[test]
fn reader_test() {
    let reader = Arc::new(Reader::new(12001).expect("reader"));
    let c_reader = reader.clone();
    let exit = Arc::new(Mutex::new(false));
    let c_exit = exit.clone();
    let t = spawn(move || {
        return c_reader.run(c_exit);
    });
    let cli = net::client("127.0.0.1:12001").expect("client");
    let timer = Duration::new(0, 500000000);
    cli.set_write_timeout(Some(timer)).expect("write timer");
    let m = [data::Message::default(); 64];
    let mut num = 0;
    while num < 64 {
        match net::write(&cli, &m[0..num + 1], &mut num) {
            Err(_) => sleep(Duration::new(0, 500000000)),
            _ => (),
        }
    }
    let mut rvs = 0usize;
    let mut tries = 0;
    while rvs < 64 && tries < 100 {
        match reader.next() {
            Err(_) => {
                sleep(Duration::new(0, 500000000));
            }
            Ok(msgs) => {
                rvs += msgs.data.len();
            }
        }
        tries = tries + 1;
    }
    *exit.lock().expect("lock") = true;
    let o = t.join().expect("thread join");
    o.expect("thread output");
    assert_eq!(rvs, 64);
}
