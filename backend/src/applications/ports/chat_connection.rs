use crate::domain::{channel::Channel, message::Message};

pub trait ChatConnection: Sized {
    async fn join_channel(&mut self, channel: Channel) -> Result<(), std::io::Error>;
    async fn next_message(&mut self) -> Result<Message, std::io::Error>;
}
