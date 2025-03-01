use std::io::{Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use crate::database::database::Database;
use crate::DatastoreVariants;
use crate::parser::parser::parse_command;

pub struct Server {
    pub stop: bool,
    pub address: String,
    pub port: String,
    pub databases: Vec<Database>,
}

impl Server {
    pub fn default() -> Server {
        Server { 
            stop: false,
            databases: Vec::new(), 
            address: "0.0.0.0".to_string(),
            port: "3333".to_string(),
        }
    }
    
    pub fn use_address(&mut self, address: &str) -> &mut Self {
        self.address = address.to_string();
        self
    }
    
    pub fn use_port(&mut self, port: &str) -> &mut Self {
        self.port = port.to_string();
        self
    }
    
    pub fn run_server(&mut self) {
        
        let full_addr = format!("{}:{}", self.address, self.port);
        let listener = TcpListener::bind(&full_addr).unwrap();
        listener.set_nonblocking(true).unwrap();
        println!("Server listening on port {}", full_addr);
        
        while !self.stop {
            
            if self.stop {
                break;
            }
            if let Ok((stream, _addr)) = listener.accept() {
                println!("New connection: {}", stream.peer_addr().unwrap());
                handle_client(stream, self);
            }
        }
        drop(listener);
    }
    
    pub fn create_database(&mut self, name: String, datastore_variant: DatastoreVariants) {
        let database = Database::new(name, datastore_variant);
        self.databases.push(database);
    }
}


fn handle_client(mut stream: TcpStream, server: &mut Server) {
    
    let mut data = String::new();

    match stream.read_to_string(&mut data) {
        Ok(_size) => {
            println!("Read data from client {}", data);
            let maybe_command = parse_command(&mut data);
            match maybe_command {
                Ok(command) => {
                    let response = command.execute(server);
                    if server.stop { return }
                    if let Err(e) = stream.write_all(response.as_bytes()) {
                        println!("Error: {}", e);
                    }
                }
                Err(e) => {
                    println!("Error: {}", e);
                }
            }
            true
        },
        Err(_) => {
            println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    };
}