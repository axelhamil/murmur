use crate::domain::error::DomainError;

#[derive(Debug)]
pub enum AppError {
    Domain(DomainError),
    Infrastructure(std::io::Error),
}
