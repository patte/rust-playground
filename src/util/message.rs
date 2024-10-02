use bit_vec::BitVec;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Message {
    pub id: u32,
    pub content: String,
}

pub fn encode_message(message: &Message) -> BitVec {
    BitVec::from_bytes(&bincode::serialize(&message).expect("Failed to serialize"))
}

pub fn decode_message(encoded: &BitVec) -> Message {
    bincode::deserialize(&encoded.to_bytes()).expect("Failed to deserialize")
}
