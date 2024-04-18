use super::{Msg, MsgData, MsgType, MONERO_GENESIS_HASH};
use serde::{Deserialize, Serialize};
use serde_epee::section::{Section, SectionEntry};
use thiserror::Error;

#[derive(Debug, Deserialize, Serialize)]
pub struct TimedSync {
    payload_data: PayloadData,
    #[serde(skip_deserializing)]
    // TODO: respond with valid peerlist
    local_peerlist_new: Vec<Section>,
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
pub struct InvalidTimedSync;

impl TryFrom<Section> for PayloadData {
    type Error = InvalidTimedSync;

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
    pub fn cmd_timed_sync(return_code: Option<u32>) -> Self {
        let msg_type = match return_code {
            Some(return_code) => MsgType::Response { return_code },
            None => MsgType::Request,
        };

        let data = TimedSync {
            payload_data: PayloadData {
                current_height: 1,
                cumulative_difficulty: 1,
                top_id: MONERO_GENESIS_HASH,
                other: Section::new(),
            },
            local_peerlist_new: Vec::new(),
        };

        Self {
            msg_type,
            msg_data: MsgData::TimedSync(data),
        }
    }
}

fn to_u64(entry: Option<SectionEntry>) -> Result<u64, InvalidTimedSync> {
    match entry {
        Some(SectionEntry::Int64(v)) => v.try_into().map_err(|_| InvalidTimedSync),
        Some(SectionEntry::Int32(v)) => v.try_into().map_err(|_| InvalidTimedSync),
        Some(SectionEntry::Int16(v)) => v.try_into().map_err(|_| InvalidTimedSync),
        Some(SectionEntry::Int8(v)) => v.try_into().map_err(|_| InvalidTimedSync),
        Some(SectionEntry::UInt64(v)) => Ok(v),
        Some(SectionEntry::UInt32(v)) => Ok(v.into()),
        Some(SectionEntry::UInt16(v)) => Ok(v.into()),
        Some(SectionEntry::UInt8(v)) => Ok(v.into()),
        _ => Err(InvalidTimedSync),
    }
}

fn to_array<const N: usize>(
    entry: Option<SectionEntry>,
) -> Result<[u8; N], InvalidTimedSync> {
    match entry {
        Some(SectionEntry::Blob(v)) => {
            v.into_vec().try_into().map_err(|_| InvalidTimedSync)
        },
        _ => Err(InvalidTimedSync),
    }
}
