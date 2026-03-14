use std::collections::HashMap;

#[derive(Debug)]
pub enum IrcParseError {
    MissingCommand,
    InvalidFormat(String),
}

#[derive(Debug)]
pub struct IrcFrame {
    pub tags: Option<HashMap<String, Option<String>>>,
    prefix: Option<String>,
    pub command: String,
    pub params: Option<Vec<String>>,
    pub trailing: Option<String>,
}

impl IrcFrame {
    pub fn parse(line: &str) -> Result<IrcFrame, IrcParseError> {
        let trimmed_line = line.trim_end();
        let (tags, rest) = Self::extract_tags(&trimmed_line)?;
        let (prefix, rest) = Self::extract_prefix(rest)?;
        let (command, rest) = Self::extract_command(rest)?;
        let (params, trailing) = Self::extract_params_and_trailing(rest)?;

        Ok(Self {
            tags,
            prefix,
            command,
            params,
            trailing,
        })
    }

    pub fn get_tag(&self, key: &str) -> Option<&String> {
        self.tags
            .as_ref()
            .and_then(|t| t.get(key))
            .and_then(|v| v.as_ref())
    }

    fn extract_tags(
        line: &str,
    ) -> Result<(Option<HashMap<String, Option<String>>>, &str), IrcParseError> {
        if line.starts_with('@') {
            let space_index = line.find(' ').ok_or(IrcParseError::InvalidFormat(
                "tags section is not followed by any content".to_owned(),
            ))?;

            let tags = line[1..space_index]
                .split(';')
                .map(|el| match el.split_once('=') {
                    Some((key, "")) => (key.to_owned(), None),
                    Some((key, val)) => (key.to_owned(), Some(val.to_owned())),
                    None => (el.to_owned(), None),
                })
                .collect();

            return Ok((Some(tags), &line[space_index + 1..]));
        };

        Ok((None, line))
    }

    fn extract_prefix(line: &str) -> Result<(Option<String>, &str), IrcParseError> {
        if line.starts_with(':') {
            let space_index = line.find(' ').ok_or(IrcParseError::InvalidFormat(
                "prefix section is not followed by any content".to_owned(),
            ))?;

            return Ok((
                Some((&line[1..space_index]).to_owned()),
                &line[space_index + 1..],
            ));
        }

        Ok((None, line))
    }

    fn extract_command(line: &str) -> Result<(String, &str), IrcParseError> {
        if line.is_empty() {
            return Err(IrcParseError::MissingCommand);
        }

        match line.find(' ') {
            Some(space_index) => Ok(((&line[..space_index]).to_owned(), &line[space_index + 1..])),
            None => Ok((line.to_owned(), "")),
        }
    }

    fn extract_params_and_trailing(
        line: &str,
    ) -> Result<(Option<Vec<String>>, Option<String>), IrcParseError> {
        if line.is_empty() {
            return Ok((None, None));
        }

        if line.starts_with(':') {
            return Ok((None, Some((&line[1..]).to_owned())));
        }

        if let Some((raw_params, trailing)) = line.split_once(" :") {
            let params = raw_params.split(' ').map(|el| el.to_owned()).collect();

            return Ok((Some(params), Some(trailing.to_owned())));
        }

        let params = line.split(' ').map(|el| el.to_owned()).collect();

        Ok((Some(params), None))
    }
}
