use super::{Msg, MsgData, MsgType};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct SupportFlags {
    #[serde(skip_deserializing)]
    support_flags: u32,
}

impl Msg {
    pub fn cmd_support_flags() -> Self {
        let data = SupportFlags {
            support_flags: 0,
        };

        Self {
            msg_type: MsgType::Request,
            msg_data: MsgData::SupportFlags(data),
        }
    }
}
