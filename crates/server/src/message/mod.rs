use chrono::{DateTime, Utc};
use crate::user::{Author};

/// Value that identifies a message.
pub struct MessageId(String);

impl MessageId {
    pub fn new() -> Self {
        MessageId(String::new())
    }
}

/// A message sent from one person to another.
pub struct Message {
    pub content: MessageContent,
    pub sender: Author,
}

impl Message {
    pub fn new(sender: Author, content: Vec<MessageFragment>) -> Self {
        Self {
            content: MessageContent(content),
            sender
        }
    }
}

pub struct MessageEntry {
    pub id: MessageId,
    pub payload: Message,
    pub creation_time: DateTime<Utc>,
}

/// The content of the message, several distinct message fragments.
pub struct MessageContent(pub Vec<MessageFragment>);

/// A fragment of a message.
pub enum MessageFragment {
    Text(String),
}