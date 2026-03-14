use crate::domain::error::DomainError;

#[derive(Clone)]
pub struct Channel {
    name: String,
}

impl Channel {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn new(name: String) -> Result<Self, DomainError> {
        if !name.starts_with('#') {
            return Err(DomainError::Validation(
                "channel name must start with: #".into(),
            ));
        }

        Ok(Self { name })
    }
}
