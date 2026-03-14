use crate::domain::channel::Channel;

pub enum IrcCommand {
    Join(Channel),
    Nick(String),
    Cap,
    Privmsg { channel: Channel, content: String },
    Pong(String),
}
