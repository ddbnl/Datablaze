use std::io::{Read, Write};
use std::net::TcpStream;
use crate::commands::command::Command;
use crate::client::errors::*;


pub struct DatabaseClient {
    server_address: String,
    server_port: String,
}

impl Default for DatabaseClient {
    fn default() -> Self {
       DatabaseClient { server_address: "127.0.0.1".to_string(), server_port: "3333".to_string() } 
    }
}

impl DatabaseClient {

    pub fn use_address(&mut self, address: &str) -> &mut Self {
        self.server_address = address.to_string();
        self
    }

    pub fn use_port(&mut self, port: &str) -> &mut Self {
        self.server_port = port.to_string();
        self
    }

    pub fn send_command<T>(&self, command: T) -> Result<String, DatabaseClientError> where T: Command {
        // The server's address and port
        let server_address = "127.0.0.1:40001";

        // Connect to the server
        match TcpStream::connect(server_address) {
            Ok(mut stream) => {
                if let Err(e) = stream.write_all(command.to_command_string().as_bytes()) {
                    return Err(DatabaseClientError { message: format!("Error writing to stream: {}", e) })
                };
                let mut result = String::new();
                if command.receive_reply() {
                    if let Err(e) = stream.read_to_string(&mut result) {
                        return Err(DatabaseClientError { message: format!("Error reading response from stream: {}", e) })
                    }
                }
                Ok(result)
            }
            Err(e) => Err(DatabaseClientError { message: format!("Error writing to stream: {}", e)})
        }
    }

}