use crate::domain::{channel::Channel, chat_notification::ChatNotification};

#[derive(Debug)]
pub enum ChatConnectionError {
    Io(std::io::Error),
    InvalidData(String),
    ConnectionClosed,
}

pub trait ChatConnection: Sized {
    async fn join_channel(&mut self, channel: Channel) -> Result<(), ChatConnectionError>;
    async fn next_notification(&mut self) -> Result<ChatNotification, ChatConnectionError>;
}
