use super::{Msg, MsgData, MsgType};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct SupportFlags {
    #[serde(skip_deserializing)]
    support_flags: u32,
}

impl Msg {
    pub fn cmd_support_flags(return_code: Option<u32>) -> Self {
        let msg_type = match return_code {
            Some(return_code) => MsgType::Response { return_code },
            None => MsgType::Request,
        };

        let data = SupportFlags {
            support_flags: 0,
        };

        Self {
            msg_type,
            msg_data: MsgData::SupportFlags(data),
        }
    }
}
