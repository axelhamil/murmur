use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpStream,
};
use tokio_rustls::{client::TlsStream, rustls};

use crate::{
    adapters::services::{irc_command::IrcCommand, irc_frame::IrcFrame},
    applications::ports::{
        chat_connection::{ChatConnection, ChatConnectionError},
        chat_connector::ChatConnector,
    },
    domain::{channel::Channel, chat_notification::ChatNotification, message::Message},
};

const TWITCH_IRC_ADDR: &str = "irc.chat.twitch.tv";
const TWITCH_IRC_PORT: u16 = 6697;
const ANONYMOUS_NICK: &str = "justinfan12345";

pub struct TwitchIrcConnector {}
pub struct TwitchIrcConnection {
    stream: BufReader<TlsStream<TcpStream>>,
}

impl ChatConnector for TwitchIrcConnector {
    type Connection = TwitchIrcConnection;
    async fn get_client() -> Result<Self::Connection, ChatConnectionError> {
        let root_store =
            rustls::RootCertStore::from_iter(webpki_roots::TLS_SERVER_ROOTS.iter().cloned());
        let config = rustls::ClientConfig::builder()
            .with_root_certificates(root_store)
            .with_no_client_auth();

        let connector = tokio_rustls::TlsConnector::from(std::sync::Arc::new(config));
        let server_name = TWITCH_IRC_ADDR.try_into().unwrap();

        let tcp_stream = TcpStream::connect((TWITCH_IRC_ADDR, TWITCH_IRC_PORT))
            .await
            .map_err(ChatConnectionError::Io)?;

        let tls_stream = connector
            .connect(server_name, tcp_stream)
            .await
            .map_err(ChatConnectionError::Io)?;

        let mut connection = TwitchIrcConnection {
            stream: BufReader::new(tls_stream),
        };

        connection
            .send(&IrcCommand::Nick(ANONYMOUS_NICK.into()))
            .await?;
        connection.send(&IrcCommand::Cap).await?;

        Ok(connection)
    }
}

impl TwitchIrcConnection {
    async fn send(&mut self, command: &IrcCommand) -> Result<(), ChatConnectionError> {
        let raw = match command {
            IrcCommand::Nick(name) => format!("NICK {}\r\n", name),
            IrcCommand::Join(channel) => format!("JOIN {}\r\n", channel.name()),
            IrcCommand::Cap => format!("CAP REQ :twitch.tv/tags twitch.tv/commands\r\n"),
            IrcCommand::Privmsg { channel, content } => {
                format!("PRIVMSG {} :{}\r\n", channel.name(), content)
            }
            IrcCommand::Pong(token) => format!("PONG :{}\r\n", token),
        };

        self.stream
            .get_mut()
            .write_all(raw.as_bytes())
            .await
            .map_err(ChatConnectionError::Io)?;

        Ok(())
    }
}

impl ChatConnection for TwitchIrcConnection {
    async fn join_channel(&mut self, channel: Channel) -> Result<(), ChatConnectionError> {
        self.send(&IrcCommand::Join(channel)).await
    }

    async fn next_notification(&mut self) -> Result<ChatNotification, ChatConnectionError> {
        loop {
            let mut line = String::new();
            match self.stream.read_line(&mut line).await {
                Ok(0) => return Err(ChatConnectionError::ConnectionClosed),
                Ok(_) => {}
                Err(err) => return Err(ChatConnectionError::Io(err)),
            }

            let irc_frame = match IrcFrame::parse(&line) {
                Ok(v) => v,
                Err(err) => return Err(ChatConnectionError::InvalidData(format!("{:?}", err))),
            };

            match irc_frame.command.as_str() {
                "PING" => match &irc_frame.trailing {
                    Some(token) => match self.send(&IrcCommand::Pong(token.clone())).await {
                        Ok(_) => {}
                        Err(err) => return Err(err),
                    },
                    None => eprintln!("Warning: PING received without token"),
                },

                "PRIVMSG" => {
                    let message = match Message::new(
                        irc_frame.get_tag("id").cloned().unwrap_or_default(),
                        irc_frame.trailing.clone().unwrap_or_default(),
                        irc_frame
                            .params
                            .as_ref()
                            .and_then(|p| p.first())
                            .cloned()
                            .unwrap_or_default(),
                        irc_frame
                            .get_tag("display-name")
                            .cloned()
                            .unwrap_or_default(),
                        irc_frame
                            .get_tag("tmi-sent-ts")
                            .and_then(|ts| ts.parse::<u64>().ok())
                            .unwrap_or(0),
                    ) {
                        Ok(v) => v,
                        Err(err) => {
                            return Err(ChatConnectionError::InvalidData(format!("{:?}", err)));
                        }
                    };

                    return Ok(ChatNotification::NewMessage(message));
                }
                _ => continue,
            };
        }
    }
}
