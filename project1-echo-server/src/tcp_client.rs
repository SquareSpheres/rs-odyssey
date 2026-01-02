use std::io;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;

pub struct TcpClient {
    host: String,
}

impl TcpClient {
    pub fn new(host: &str) -> Self {
        Self { host: host.to_string() }
    }
    pub fn start(&self)-> std::io::Result<()> {

        let mut stream = TcpStream::connect(&self.host)?;

        let mut buffer_reader = BufReader::new(&mut stream);

        loop {
            let stdin = io::stdin();
            let mut line_to_server = String::new();
            let mut line_from_server = String::new();

            println!("Please enter command:");
            stdin.read_line(&mut line_to_server)?;

            if(line_to_server.trim().is_empty()){
                println!("Exiting");
                break;
            }

            println!("String to server : {}", line_to_server);
            buffer_reader.get_mut().write_all(line_to_server.as_bytes())?;
            buffer_reader.get_mut().flush()?;

            let number_of_bytes = buffer_reader.read_line(&mut line_from_server)?;

            // echo server always returns something, for 0 bytes it will exit
            assert!(number_of_bytes > 0);

            print!("String from server : {}", line_from_server);
        }

        Ok(())

    }
}
