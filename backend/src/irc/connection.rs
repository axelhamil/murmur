use tokio::net::TcpStream;
use tokio_rustls::{client::TlsStream, rustls};

const TWITCH_IRC_ADDR: &str = "irc.chat.twitch.tv";
const TWITCH_IRC_PORT: u16 = 6697;
const ANONYMOUS_NICK: &str = "justinfan12345";

pub async fn connect(channel: &str) -> Result<TlsStream<TcpStream>, std::io::Error> {
    let root_store =
        rustls::RootCertStore::from_iter(webpki_roots::TLS_SERVER_ROOTS.iter().cloned());
    let config = rustls::ClientConfig::builder()
        .with_root_certificates(root_store)
        .with_no_client_auth();

    let connector = tokio_rustls::TlsConnector::from(std::sync::Arc::new(config));
    let server_name = TWITCH_IRC_ADDR.try_into().unwrap();

    let tcp_stream = TcpStream::connect((TWITCH_IRC_ADDR, TWITCH_IRC_PORT)).await?;

    connector.connect(server_name, tcp_stream).await
}
