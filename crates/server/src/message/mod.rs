use crate::user::Author;
use chrono::{DateTime, Utc};
use rocket::serde::{Deserialize, Serialize};

/// Value that identifies a message.
pub struct MessageId(String);

impl MessageId {
    pub fn new() -> Self {
        MessageId(String::new())
    }
}

/// A message sent from one person to another.
#[derive(Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct MessagePayload {
    pub content: MessageContent,
    pub sender: Author,
}

impl MessagePayload {
    pub fn new(sender: Author, content: Vec<MessageFragment>) -> Self {
        Self {
            content: MessageContent(content),
            sender,
        }
    }
}

pub struct MessageEntry {
    pub id: MessageId,
    pub payload: MessagePayload,
    pub creation_time: DateTime<Utc>,
}

/// The content of the message, several distinct message fragments.
#[derive(Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct MessageContent(pub Vec<MessageFragment>);

/// A fragment of a message.
#[derive(Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
pub enum MessageFragment {
    Text(String),
}
