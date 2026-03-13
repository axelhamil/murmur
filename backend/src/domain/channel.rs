use crate::domain::error::DomainError;

#[allow(dead_code)]
#[derive(Clone)]
pub struct Channel {
    name: String,
}

#[allow(dead_code)]
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
