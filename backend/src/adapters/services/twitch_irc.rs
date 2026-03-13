use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpStream,
};
use tokio_rustls::{client::TlsStream, rustls};

use crate::{
    applications::ports::{irc_connection::IrcConnection, irc_connector::IrcConnector},
    domain::{channel::Channel, irc_command::IrcCommand, message::Message},
};

const TWITCH_IRC_ADDR: &str = "irc.chat.twitch.tv";
const TWITCH_IRC_PORT: u16 = 6697;
const ANONYMOUS_NICK: &str = "justinfan12345";

pub struct TwitchIrcConnector {}
pub struct TwitchIrcConnection {
    stream: BufReader<TlsStream<TcpStream>>,
}

impl IrcConnector for TwitchIrcConnector {
    type Connection = TwitchIrcConnection;
    async fn connect(_channel: &Channel) -> Result<Self::Connection, std::io::Error> {
        let root_store =
            rustls::RootCertStore::from_iter(webpki_roots::TLS_SERVER_ROOTS.iter().cloned());
        let config = rustls::ClientConfig::builder()
            .with_root_certificates(root_store)
            .with_no_client_auth();

        let connector = tokio_rustls::TlsConnector::from(std::sync::Arc::new(config));
        let server_name = TWITCH_IRC_ADDR.try_into().unwrap();

        let tcp_stream = TcpStream::connect((TWITCH_IRC_ADDR, TWITCH_IRC_PORT)).await?;

        let tls_stream = connector.connect(server_name, tcp_stream).await?;

        let mut connection = TwitchIrcConnection {
            stream: BufReader::new(tls_stream),
        };

        connection
            .send(&IrcCommand::Nick(ANONYMOUS_NICK.to_owned()))
            .await?;

        Ok(connection)
    }
}

impl IrcConnection for TwitchIrcConnection {
    async fn send(&mut self, command: &IrcCommand) -> Result<(), std::io::Error> {
        let raw = match command {
            IrcCommand::Nick(name) => format!("NICK {}\r\n", name),
            IrcCommand::Join(channel) => format!("JOIN {}\r\n", channel.name()),
            IrcCommand::Privmsg { channel, content } => {
                format!("PRIVMSG {} :{}\r\n", channel.name(), content)
            }
            IrcCommand::Pong(token) => format!("PONG :{}\r\n", token),
        };

        self.stream.get_mut().write_all(raw.as_bytes()).await?;

        Ok(())
    }

    // TODO: parser la ligne IRC en vrai Message
    async fn read_line(&mut self) -> Result<Message, std::io::Error> {
        let mut line = String::new();
        self.stream.read_line(&mut line).await?;

        println!("{}", line);

        Ok(Message::new(
            "0".to_owned(),
            line,
            "unknow".to_owned(),
            "unknow".to_owned(),
            0,
        )
        .unwrap())
    }
}
