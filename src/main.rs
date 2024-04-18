mod msg;

use msg::Msg;
use std::{
    net::{SocketAddr, TcpStream},
    str::FromStr,
    time::Duration,
};

fn main() {
    let addr = SocketAddr::from_str("127.0.0.1:18080").unwrap();
    let timeout = Duration::from_secs(30);

    let mut stream = TcpStream::connect_timeout(&addr, timeout).unwrap();
    stream.set_write_timeout(Some(timeout)).unwrap();

    Msg::cmd_handshake().send(&mut stream).unwrap();

    loop {
        let msg = Msg::recv(&mut stream).unwrap();
        println!("{:?}", msg);
    }
}
