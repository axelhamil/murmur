use crate::domain::channel::Channel;

#[allow(dead_code)]
pub enum IrcCommand {
    Join(Channel),
    Nick(String),
    Privmsg { channel: Channel, content: String },
    Pong(String),
}
