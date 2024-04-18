mod handshake;
mod raw_header;

pub use raw_header::RawHeader;
use std::{
    io::{Error as IoError, Read, Write},
    net::TcpStream,
};
use thiserror::Error;

#[derive(Debug)]
pub struct Msg {
    msg_type: MsgType,
    msg_data: MsgData,
}

#[derive(Debug)]
pub enum MsgType {
    Notification,
    Request,
    Response { return_code: u32 },
    Unknown { expect_response: u8, return_code: u32, flags: u32 },
}

#[derive(Debug)]
pub enum MsgData {
    Unknown { command: u32, data: Vec<u8> },
}

#[derive(Debug, Error)]
pub enum MsgError {
    #[error("Error receiving message: {0}")]
    RecvError(IoError),
    #[error("Error sending message: {0}")]
    SendError(IoError),
    #[error("Invalid header")]
    InvalidHeader(RawHeader),
    #[error("Message exceeds maximum allowed length")]
    TooLong(RawHeader),
    #[error("Invalid request")]
    InvalidRequest(RawHeader),
    #[error("Invalid response")]
    InvalidResponse(RawHeader),
}

type Result<T> = std::result::Result<T, MsgError>;

const MAX_PAYLOAD_LENGTH: usize = 256 * 1024;

impl Msg {
    pub fn recv(stream: &mut TcpStream) -> Result<Self> {
        let mut header = [0; raw_header::LENGTH];
        stream.read_exact(&mut header).map_err(MsgError::RecvError)?;
        let header = RawHeader::from(header);

        if header.signature != raw_header::SIGNATURE ||
           header.version != raw_header::VERSION {
            return Err(MsgError::InvalidHeader(header));
        }
        if header.length > MAX_PAYLOAD_LENGTH {
            return Err(MsgError::TooLong(header));
        }

        let mut data = vec![0; header.length];
        stream.read_exact(&mut data).map_err(MsgError::RecvError)?;

        Self::from_raw_parts(header, data)
    }

    pub fn send(&self, stream: &mut TcpStream) -> Result<()> {
        stream.write_all(&self.to_bytes()).map_err(MsgError::SendError)
    }

    fn from_raw_parts(header: RawHeader, data: Vec<u8>) -> Result<Self> {
        let msg_type = match (header.expect_response, header.return_code, header.flags) {
            (0, 0, 1) => MsgType::Notification,
            (_, 0, 1) => MsgType::Request,
            (_, _, 1) => return Err(MsgError::InvalidRequest(header)),
            (0, return_code, 2) => MsgType::Response { return_code },
            (_, _, 2) => return Err(MsgError::InvalidResponse(header)),
            (expect_response, return_code, flags) => {
                MsgType::Unknown { expect_response, return_code, flags }
            },
        };

        Ok(Self {
            msg_type,
            msg_data: MsgData::Unknown {
                command: header.command,
                data,
            },
        })
    }

    fn to_bytes(&self) -> Vec<u8> {
        let (command, data) = match &self.msg_data {
            MsgData::Unknown { command, data } => (*command, data.clone()),
        };

        let (expect_response, return_code, flags) = match self.msg_type {
            MsgType::Notification => (0, 0, 1),
            MsgType::Request => (1, 0, 1),
            MsgType::Response { return_code } => (0, return_code, 2),
            MsgType::Unknown { expect_response, return_code, flags } => {
                (expect_response, return_code, flags)
            },
        };

        let header = RawHeader {
            signature: raw_header::SIGNATURE,
            length: data.len(),
            expect_response,
            command,
            return_code,
            flags,
            version: raw_header::VERSION,
        };
        let header: [u8; raw_header::LENGTH] = header.into();

        [header.into(), data].concat()
    }
}
