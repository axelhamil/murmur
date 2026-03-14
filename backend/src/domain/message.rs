use crate::domain::error::DomainError;

#[derive(Debug)]
pub struct Message {
    id: String,
    content: String,
    channel_id: String,
    author: String,
    timestamp: u64,
}

impl Message {
    pub fn content(&self) -> &str {
        &self.content
    }
    pub fn author(&self) -> &str {
        &self.author
    }

    pub fn new(
        id: String,
        content: String,
        channel_id: String,
        author: String,
        timestamp: u64,
    ) -> Result<Self, DomainError> {
        if content.is_empty() || author.is_empty() {
            return Err(DomainError::Validation(
                "content or author can't be empty".into(),
            ));
        }

        Ok(Self {
            id,
            content,
            channel_id,
            author,
            timestamp,
        })
    }
}
