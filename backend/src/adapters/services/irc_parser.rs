use std::collections::HashMap;

use crate::adapters::services::error::IrcParserError;

pub struct IrcMessage {
    tags: Option<HashMap<String, Option<String>>>,
    prefix: Option<String>,
    command: String,
    params: Option<Vec<String>>,
    trailing: Option<String>,
}

impl IrcMessage {
    pub fn parse(line: &str) -> Result<IrcMessage, IrcParserError> {
        let (tags, rest) = Self::extract_tags(&line)?;

        todo!()
    }

    fn extract_tags(
        line: &str,
    ) -> Result<(Option<HashMap<String, Option<String>>>, &str), IrcParserError> {
        if line.starts_with("@") {
            match line.find(" ") {
                Some(space_index) => {
                    let tags = line[1..space_index]
                        .split(";")
                        .map(|el| match el.split_once("=") {
                            Some((key, "")) => (key.to_string(), None),
                            Some((key, val)) => (key.to_string(), Some(val.to_string())),
                            None => (el.to_string(), None),
                        });

                    return Ok((Some(tags.collect()), &line[space_index + 1..]));
                }
                None => return Err(IrcParserError::MissingCommand),
            };
        };

        Ok((None, line))
    }
}
