use crate::domain::error::DomainError;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Message {
    id: String,
    content: String,
    channel_id: String,
    author: String,
    timestamp: u64,
}

#[allow(dead_code)]
impl Message {
    pub fn id(&self) -> &str {
        &self.id
    }
    pub fn content(&self) -> &str {
        &self.content
    }
    pub fn channel_id(&self) -> &str {
        &self.channel_id
    }
    pub fn author(&self) -> &str {
        &self.author
    }
    pub fn timestamp(&self) -> u64 {
        self.timestamp
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
