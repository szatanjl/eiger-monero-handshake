use super::{Msg, MsgData, MsgType};
use serde::{Deserialize, Serialize};
use serde_epee::section::{Section, SectionEntry};
use thiserror::Error;

#[derive(Debug, Deserialize, Serialize)]
pub struct Handshake {
    node_data: NodeData,
    payload_data: PayloadData,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(try_from = "Section")]
struct NodeData {
    #[serde(with = "serde_bytes")]
    network_id: [u8; 16],
    peer_id: u64,
    my_port: u32,
    #[serde(skip_serializing)]
    #[allow(dead_code)]
    other: Section,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(try_from = "Section")]
struct PayloadData {
    current_height: u64,
    cumulative_difficulty: u64,
    #[serde(with = "serde_bytes")]
    top_id: [u8; 32],
    #[serde(skip_serializing)]
    #[allow(dead_code)]
    other: Section,
}

#[derive(Debug, Error)]
#[error("Invalid handshake command")]
pub struct InvalidHandshake;

const MAINNET: [u8; 16] = [
    0x12, 0x30, 0xf1, 0x71, 0x61, 0x04, 0x41, 0x61,
    0x17, 0x31, 0x00, 0x82, 0x16, 0xa1, 0xa1, 0x10,
];
const MONERO_GENESIS_HASH: [u8; 32] = [
    0x41, 0x80, 0x15, 0xbb, 0x9a, 0xe9, 0x82, 0xa1,
    0x97, 0x5d, 0xa7, 0xd7, 0x92, 0x77, 0xc2, 0x70,
    0x57, 0x27, 0xa5, 0x68, 0x94, 0xba, 0x0f, 0xb2,
    0x46, 0xad, 0xaa, 0xbb, 0x1f, 0x46, 0x32, 0xe3,
];
const PEER_ID: u64 = 0xDAADB00D54AFF1EE;

impl TryFrom<Section> for NodeData {
    type Error = InvalidHandshake;

    fn try_from(mut section: Section) -> Result<Self, Self::Error> {
        Ok(Self {
            network_id: to_array(section.remove("network_id"))?,
            peer_id: to_u64(section.remove("peer_id"))?,
            my_port: to_u32(section.remove("my_port"))?,
            other: section,
        })
    }
}

impl TryFrom<Section> for PayloadData {
    type Error = InvalidHandshake;

    fn try_from(mut section: Section) -> Result<Self, Self::Error> {
        Ok(Self {
            current_height: to_u64(section.remove("current_height"))?,
            cumulative_difficulty: to_u64(section.remove("cumulative_difficulty"))?,
            top_id: to_array(section.remove("top_id"))?,
            other: section,
        })
    }
}

impl Msg {
    pub fn cmd_handshake(return_code: Option<u32>) -> Self {
        let msg_type = match return_code {
            Some(return_code) => MsgType::Response { return_code },
            None => MsgType::Request,
        };

        let data = Handshake {
            node_data: NodeData {
                network_id: MAINNET,
                peer_id: PEER_ID,
                my_port: 0,
                other: Section::new(),
            },
            payload_data: PayloadData {
                current_height: 1,
                cumulative_difficulty: 1,
                top_id: MONERO_GENESIS_HASH,
                other: Section::new(),
            },
        };

        Self {
            msg_type,
            msg_data: MsgData::Handshake(data),
        }
    }
}

fn to_u32(entry: Option<SectionEntry>) -> Result<u32, InvalidHandshake> {
    match entry {
        Some(SectionEntry::Int64(v)) => v.try_into().map_err(|_| InvalidHandshake),
        Some(SectionEntry::Int32(v)) => v.try_into().map_err(|_| InvalidHandshake),
        Some(SectionEntry::Int16(v)) => v.try_into().map_err(|_| InvalidHandshake),
        Some(SectionEntry::Int8(v)) => v.try_into().map_err(|_| InvalidHandshake),
        Some(SectionEntry::UInt64(v)) => v.try_into().map_err(|_| InvalidHandshake),
        Some(SectionEntry::UInt32(v)) => Ok(v),
        Some(SectionEntry::UInt16(v)) => Ok(v.into()),
        Some(SectionEntry::UInt8(v)) => Ok(v.into()),
        _ => Err(InvalidHandshake),
    }
}

fn to_u64(entry: Option<SectionEntry>) -> Result<u64, InvalidHandshake> {
    match entry {
        Some(SectionEntry::Int64(v)) => v.try_into().map_err(|_| InvalidHandshake),
        Some(SectionEntry::Int32(v)) => v.try_into().map_err(|_| InvalidHandshake),
        Some(SectionEntry::Int16(v)) => v.try_into().map_err(|_| InvalidHandshake),
        Some(SectionEntry::Int8(v)) => v.try_into().map_err(|_| InvalidHandshake),
        Some(SectionEntry::UInt64(v)) => Ok(v),
        Some(SectionEntry::UInt32(v)) => Ok(v.into()),
        Some(SectionEntry::UInt16(v)) => Ok(v.into()),
        Some(SectionEntry::UInt8(v)) => Ok(v.into()),
        _ => Err(InvalidHandshake),
    }
}

fn to_array<const N: usize>(
    entry: Option<SectionEntry>,
) -> Result<[u8; N], InvalidHandshake> {
    match entry {
        Some(SectionEntry::Blob(v)) => {
            v.into_vec().try_into().map_err(|_| InvalidHandshake)
        },
        _ => Err(InvalidHandshake),
    }
}
