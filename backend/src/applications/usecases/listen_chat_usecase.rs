use crate::{
    applications::{
        error::AppError,
        ports::{irc_connection::IrcConnection, irc_connector::IrcConnector},
    },
    domain::channel::Channel,
};

pub struct ListenChatUseCase;

impl ListenChatUseCase {
    pub async fn execute<T: IrcConnector>(channel_name: &str) -> Result<(), AppError> {
        let channel = match Channel::new(channel_name.to_owned()) {
            Ok(channel) => channel,
            Err(err) => return Err(AppError::Domain(err)),
        };

        let mut client = match T::get_client().await {
            Ok(c) => c,
            Err(err) => return Err(AppError::Infrastructure(err)),
        };

        client
            .join_channel(channel.into())
            .await
            .map_err(AppError::Infrastructure)?;

        loop {
            match client.read_line().await {
                Ok(message) => println!("{:?}", message),
                Err(err) => return Err(AppError::Infrastructure(err)),
            }
        }
    }
}
