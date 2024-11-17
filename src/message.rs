use std::sync::Arc;

use serde::{Deserialize, Serialize};
use strum::{EnumDiscriminants, EnumString};

#[derive(Debug, Clone, strum::Display, EnumDiscriminants, Serialize, Deserialize)]
#[strum_discriminants(name(MessageRole))]
#[strum_discriminants(derive(EnumString))]
#[strum_discriminants(strum(serialize_all = "lowercase"))]
pub enum Message {
    #[strum(serialize = "assistant: {0}")]
    Assistant(Arc<str>),
    #[strum(serialize = "user: {0}")]
    User(Arc<str>),
    #[strum(serialize = "system: {0}")]
    System(Arc<str>),
}

impl Message {
    pub fn role(&self) -> &'static str {
        match self {
            Message::Assistant(_) => "assistant",
            Message::User(_) => "user",
            Message::System(_) => "system",
        }
    }

    pub fn content(&self) -> Arc<str> {
        match self {
            Message::Assistant(arc) | Message::User(arc) | Message::System(arc) => arc.clone(),
        }
    }
}

impl<'a> From<(MessageRole, &'a str)> for Message {
    fn from(value: (MessageRole, &'a str)) -> Self {
        let (role, message) = value;
        let message: Arc<str> = message.into();
        match role {
            MessageRole::Assistant => Message::Assistant(message),
            MessageRole::User => Message::User(message),
            MessageRole::System => Message::System(message),
        }
    }
}
