use serde_json::Value;

pub struct SendMsgResults {
    pub message_id: String,
    pub time: u64,
    pub data: Value,
}
