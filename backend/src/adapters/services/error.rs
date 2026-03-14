#[derive(Debug)]
pub enum IrcParserError {
    MissingCommand,
    InvalidFormat(String),
}
