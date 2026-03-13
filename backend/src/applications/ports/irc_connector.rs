use crate::applications::ports::irc_connection::IrcConnection;

pub trait IrcConnector {
    type Connection: IrcConnection;
    async fn get_client() -> Result<Self::Connection, std::io::Error>;
}
