use super::{Msg, MsgData, MsgType};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct Handshake {
    node_data: NodeData,
    payload_data: PayloadData,
}

#[derive(Debug, Deserialize, Serialize)]
struct NodeData {
    #[serde(with = "serde_bytes")]
    network_id: [u8; 16],
    peer_id: u64,
    my_port: u32,
}

#[derive(Debug, Deserialize, Serialize)]
struct PayloadData {
    current_height: u64,
    cumulative_difficulty: u64,
    #[serde(with = "serde_bytes")]
    top_id: [u8; 32],
}

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

impl Msg {
    pub fn cmd_handshake() -> Self {
        let data = Handshake {
            node_data: NodeData {
                network_id: MAINNET,
                peer_id: PEER_ID,
                my_port: 0,
            },
            payload_data: PayloadData {
                current_height: 1,
                cumulative_difficulty: 1,
                top_id: MONERO_GENESIS_HASH,
            },
        };

        Self {
            msg_type: MsgType::Request,
            msg_data: MsgData::Unknown {
                command: 1001,
                data: serde_epee::to_bytes(&data).unwrap(),
            },
        }
    }
}
