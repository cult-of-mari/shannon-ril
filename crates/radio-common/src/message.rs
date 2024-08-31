//! Message structure.

/// A message.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Message {
    pub message_sequence: u8,
    pub acknowledge_sequence: u8,
    pub category: u8,
    pub which: u8,
    pub kind: u8,
    pub data: Vec<u8>,
}
