use crate::{applications::ports::irc_connection::IrcConnection, domain::channel::Channel};

pub trait IrcConnector {
    type Connection: IrcConnection;
    async fn connect(channel: &Channel) -> Result<Self::Connection, std::io::Error>;
}
