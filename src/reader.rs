use std::sync::{Arc, Mutex};
use std::collections::VecDeque;
use std::net::{SocketAddr, Ipv4Addr, IpAddr};
use result::{Result, from_option};
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
    pub fn run(&self) -> Result<()> {
        let ipv4 = Ipv4Addr::new(0, 0, 0, 0);
        let addr = SocketAddr::new(IpAddr::V4(ipv4), self.port);
        const READABLE: mio::Token = mio::Token(0);
        let poll = mio::Poll::new()?;
        let srv = mio::net::UdpSocket::bind(&addr)?;
        poll.register(&srv, READABLE, mio::Ready::readable(),
                       mio::PollOpt::edge())?;
        let mut events = mio::Events::with_capacity(8);
        
        loop {
            poll.poll(&mut events, None)?;
            let mut m =  self.allocate();
            let c = Arc::clone(&m);
            let v = from_option(Arc::get_mut(&mut m))?;
            let num = net::read_from(&srv, &mut v.msgs, &mut v.data)?;
            let total = v.data.iter_mut().map(|v| v.0).sum();
            v.msgs.resize(total, data::Message::default());
            v.data.resize(num, Messages::def_data());
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
                _ => Arc::new(Messages::new())
        }
    }
    fn enqueue(&self, m: SharedMessages) {
        let mut s = self.lock.lock().expect("lock");
        s.pending.push_back(m);
    }
}
