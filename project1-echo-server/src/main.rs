use crate::tcp_server::{Server, ServerConfig};
use std::net::{IpAddr, Ipv4Addr};

mod tcp_client;
mod tcp_server;
mod message_protocol;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    if args.get(1).map(|s| s == "server").unwrap_or(false) {
        println!("Starting server...");
        let localhost = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
        let server = Server::new(ServerConfig { ip_addr: localhost, port: 8080 });
        server.start()?;
    } else if args.get(1).map(|s| s == "client").unwrap_or(false) {
        println!("Starting client...");
        let client = tcp_client::TcpClient::new("127.0.0.1:8080");
        client.start()?;
    } else {
        println!("Usage: cargo run -- [server|client]");
    }

    Ok(())
}
