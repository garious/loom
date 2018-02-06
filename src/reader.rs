use std::sync::{Arc, Mutex};
use std::collections::VecDeque;
use std::net::{SocketAddr, Ipv4Addr, IpAddr};
use result::{Result, from_option};
use std::time::Duration;
use mio;
use data;
use net;

pub struct Messages {
    msgs: Vec<data::Message>,
    data: Vec<(usize,SocketAddr)>,
}

impl Messages {
    fn new() -> Messages {
        let mut m = Vec::new();
        m.resize(1024, data::Message::default());
        let mut d = Vec::new();
        d.resize(1024, Self::def_data());
        Messages{msgs:m, data:d}
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
    port: u16,
}
impl Reader {
    pub fn new(port: u16) -> Reader {
        let d = Data { gc: Vec::new(),
                       pending: VecDeque::new() };

        return Reader{lock: Mutex::new(d), port: port};
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
        let ipv4 = Ipv4Addr::new(0, 0, 0, 0);
        let addr = SocketAddr::new(IpAddr::V4(ipv4), self.port);
        const READABLE: mio::Token = mio::Token(0);
        let poll = mio::Poll::new()?;
        let srv = mio::net::UdpSocket::bind(&addr)?;
        poll.register(&srv, READABLE, mio::Ready::readable(),
                       mio::PollOpt::edge())?;
        let mut events = mio::Events::with_capacity(8);
        
        loop {
            {
                let e = exit.lock().expect("lock");
                if *e == true {
                    return Ok(());
                }
            }
            let timeout = Duration::new(1, 0);
            println!("HERE polling");
            match poll.poll(&mut events, Some(timeout)) {
                Err(_) => {
                    println!("HERE error");
                    continue;
                }
                Ok(_) => {
                    println!("HERE ok");
                    let mut m =  self.allocate();
                    {
                        let v = Arc::get_mut(&mut m).expect("only ref");
                        match net::read_from(&srv, &mut v.msgs, &mut v.data) {
                            Err(e) => {
                                println!("read failed error {:?}", e);
                                continue;
                            }
                            Ok(0) => {
                                println!("read returned 0");
                                continue;
                            }
                            Ok(num) => {
                                println!("HERE read {:?}", num);
                                let total = v.data.iter_mut()
                                                  .map(|v| v.0)
                                                  .sum();
                                v.msgs.resize(total, data::Message::default());
                                v.data.resize(num, Messages::def_data());
                            }
                        }
                    }
                    let c = Arc::clone(&m);
                    println!("HERE enqueue");
                    self.enqueue(c);
                    self.notify();
                }
            }
        }
    }
    fn notify(&self) {
        //TODO(anatoly), hard code other threads to notify
    }
    fn allocate(&self) -> SharedMessages {
        let mut s = self.lock.lock().expect("lock");
        return match s.gc.pop() {
                Some(v) => v,
                _ => Arc::new(Messages::new())
        }
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
    let reader = Arc::new(Reader::new(12001));
    let c_reader = reader.clone();
    let exit = Arc::new(Mutex::new(false));
    let c_exit = exit.clone();
    let t = spawn(move || {
        return c_reader.run(c_exit);
    });
    let cli = net::client("127.0.0.1:12001").expect("client");
    let m = [data::Message::default(); 64];
    let mut num = 0;
    for n in 0 .. 64 { 
        match net::write(&cli, &m[n..n+1], &mut num) {
            Err(_) => break,
            _ => continue
        }
    }
    sleep(Duration::new(2, 0));
    *exit.lock().expect("lock") = true;
    let o = t.join().expect("thread join");
    o.expect("thread output");
    let r = reader.next().expect("messages");
    assert_eq!(r.msgs.len(), 64);
}
