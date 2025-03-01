use crate::network::server::Server;
pub use datablaze_types::enums::*;

mod commands;
mod datastore;
mod network;
mod parser;
mod database;

fn main() {
    let mut server = Server::default();
    server.run_server();

}
