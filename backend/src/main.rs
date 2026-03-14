use crate::{
    adapters::services::twitch_irc::TwitchIrcConnector,
    applications::usecases::listen_chat_usecase::ListenChatUseCase,
};

mod adapters;
mod applications;
mod domain;

#[tokio::main]
async fn main() {
    if let Err(e) = ListenChatUseCase::execute::<TwitchIrcConnector>("#alphacast").await {
        println!("{:?}", e);
    }
}
