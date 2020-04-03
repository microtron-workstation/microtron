use crate::types::{ID, Sample};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Packet {
    pub from: ID,
    pub to:   ID,
    pub message: Message,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Message {
    Connect,
    ConnectionEstablished,

    Buffer {
        data: Vec<Sample>,
    },

    Invalid,
}
