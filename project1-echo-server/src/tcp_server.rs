use std::io::{BufRead, BufReader, Read, Write};
use std::net::{IpAddr, Shutdown, TcpListener};
use crate::message_protocol::{Command, ParseError};

pub struct ServerConfig {
    pub ip_addr: IpAddr,
    pub port: u16,
}

pub struct Server {
    config: ServerConfig,
}



impl Server {
    pub fn start(&self) -> std::io::Result<()> {
        let tcp_listener = TcpListener::bind((self.config.ip_addr, self.config.port))?;

        for stream in tcp_listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    let mut buffer_reader = BufReader::new(&mut stream);

                    loop {
                        let mut line = String::new();
                        let number_of_bytes = buffer_reader.read_line(&mut line)?;

                        match Command::parse(line.as_str()) {
                            Ok(command) => match command {
                                Command::Exit => {
                                    println!("Received exit command... Exiting...");
                                    stream.shutdown(Shutdown::Both)?;
                                    break;
                                }
                                Command::Message(message) => {
                                    buffer_reader.get_mut().write_all(message.as_bytes())?;
                                }
                            }
                            Err(e) => {

                            }
                        }


                    }


                }
                Err(e) => {
                    println!("Error: {}", e);
                }
            }
        }

        Ok(())
    }

    pub fn new(config: ServerConfig) -> Self{
        Self { config }
    }
}
