mod irc;

#[tokio::main]
async fn main() {
    let tls_stream = match irc::connection::connect("").await {
        Ok(v) => v,
        Err(e) => {
            println!("{:?}", e);
            return;
        }
    };

    println!("{}", "Connecté !")
}
