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
	/// A module requests a connection to the relay server
    ConnectionRequested {
    	name: Option<String>,
    	description: Option<String>,
    },

    /// The connection to the relay has been successfully established
    ConnectionEstablished {
    	id: ID,
    },
}
