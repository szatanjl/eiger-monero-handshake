mod msg;

use msg::{Msg, MsgData, MsgType, Result};
use std::{
    net::{SocketAddr, TcpStream},
    str::FromStr,
    time::Duration,
};

fn handle_request(stream: &mut TcpStream, req: MsgData) -> Result<()> {
    match req {
        MsgData::Handshake(_data) => Msg::cmd_handshake(Some(1)).send(stream),
        MsgData::TimedSync(_data) => Msg::cmd_timed_sync(Some(1)).send(stream),
        MsgData::SupportFlags(_data) => Msg::cmd_support_flags(Some(1)).send(stream),
        MsgData::Unknown { command, .. } => {
            eprintln!("WARN: Unknown request command: {}.  Not responding.", command);
            Ok(())
        },
    }
}

fn main() {
    let addr = SocketAddr::from_str("127.0.0.1:18080").unwrap();
    let timeout = Duration::from_secs(30);

    let mut stream = TcpStream::connect_timeout(&addr, timeout).unwrap();
    stream.set_write_timeout(Some(timeout)).unwrap();

    Msg::cmd_handshake(None).send(&mut stream).unwrap();

    loop {
        let msg = Msg::recv(&mut stream).unwrap();
        println!("{:?}", msg);

        if matches!(msg.msg_type, MsgType::Request) {
            handle_request(&mut stream, msg.msg_data).unwrap();
        }
    }
}
