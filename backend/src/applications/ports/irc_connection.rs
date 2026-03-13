use crate::domain::{irc_command::IrcCommand, message::Message};

pub trait IrcConnection: Sized {
    async fn read_line(&mut self) -> Result<Message, std::io::Error>;
    async fn send(&mut self, command: &IrcCommand) -> Result<(), std::io::Error>;
}
