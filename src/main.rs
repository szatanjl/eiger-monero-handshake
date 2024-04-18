mod msg;

use msg::{Msg, MsgData, MsgType, Result};
use std::{
    net::{TcpStream, ToSocketAddrs},
    time::Duration,
};

const DEFAULT_NODE: &str = "node.moneroworld.com:18080";

fn handle_request(stream: &mut TcpStream, req: MsgData) -> Result<()> {
    match req {
        MsgData::Handshake(_data) => Msg::cmd_handshake(Some(1)).send(stream),
        MsgData::TimedSync(_data) => Msg::cmd_timed_sync(Some(1)).send(stream),
        MsgData::SupportFlags(_data) => Msg::cmd_support_flags(Some(1)).send(stream),
        MsgData::NewBlock => {
            eprintln!("ERR: NewBlock command sent as Request, it should be a Notification.  Not responding.");
            Ok(())
        },
        MsgData::NewTransactions => {
            eprintln!("ERR: NewTransactions command sent as Request, it should be a Notification.  Not responding.");
            Ok(())
        },
        MsgData::Unknown { command, .. } => {
            eprintln!("WARN: Unknown request command: {}.  Not responding.", command);
            Ok(())
        },
    }
}

fn main() {
    let mut args = std::env::args();
    args.next();
    let addr = args.next();
    let addr = addr.as_deref().unwrap_or(DEFAULT_NODE);
    let addr_resolved = addr.to_socket_addrs().unwrap().next().unwrap();

    let timeout = Duration::from_secs(30);

    println!("Connecting to {} ({})", addr, addr_resolved);
    let mut stream = TcpStream::connect_timeout(&addr_resolved, timeout).unwrap();
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
