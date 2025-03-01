use crate::network::server::Server;

pub trait Command {
    fn execute(&self, server: &mut Server) -> String;
}