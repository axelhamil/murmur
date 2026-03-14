use crate::applications::ports::chat_connection::{ChatConnection, ChatConnectionError};

pub trait ChatConnector {
    type Connection: ChatConnection;
    async fn get_client() -> Result<Self::Connection, ChatConnectionError>;
}
