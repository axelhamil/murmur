use crate::{
    applications::ports::chat_connection::ChatConnectionError, domain::error::DomainError,
};

#[derive(Debug)]
pub enum AppError {
    Domain(DomainError),
    Infrastructure(std::io::Error),
    Connection(ChatConnectionError),
}
