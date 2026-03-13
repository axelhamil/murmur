use crate::domain::{channel::Channel, message::Message};

pub trait IrcConnection: Sized {
    async fn join_channel(&mut self, channel: Channel) -> Result<(), std::io::Error>;
    async fn read_line(&mut self) -> Result<Message, std::io::Error>;
}
