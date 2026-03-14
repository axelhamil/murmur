use crate::domain::message::Message;

pub enum ChatNotification {
    NewMessage(Message),
}
