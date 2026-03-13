use crate::domain::error::DomainError;

#[allow(dead_code)]
#[derive(Debug)]
pub enum AppError {
    Domain(DomainError),
    Infrastructure(std::io::Error),
}
