use crate::applications::ports::chat_connection::ChatConnection;

pub trait ChatConnector {
    type Connection: ChatConnection;
    async fn get_client() -> Result<Self::Connection, std::io::Error>;
}
