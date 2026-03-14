#[derive(Debug)]
pub enum IrcParseError {
    MissingCommand,
    InvalidFormat(String),
}
