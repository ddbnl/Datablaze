use crate::commands::command::Command;
use crate::DatastoreVariants;
use crate::network::server::Server;

pub  struct CreateDatabaseCommand {
    pub name: String,
    pub datastore_variant: DatastoreVariants,
}

impl Command for CreateDatabaseCommand {
    fn execute(&self, server: &mut Server) -> String {
        server.create_database(self.name.clone(), self.datastore_variant);
        "Created".to_string()
        
    }
}

pub struct GetDatabaseCommand {
    pub name: String,
}

impl Command for GetDatabaseCommand {
    fn execute(&self, server: &mut Server) -> String {
        let maybe_database = server.databases.iter().find(|x| x.name == self.name);
        if let Some(database) = maybe_database {
            format!("Found: {}", database.name)
        }
        else {
            "Not found".to_string()
        }
    }
}

#[derive(Default)]
pub struct StopServerCommand;

impl Command for StopServerCommand {
    fn execute(&self, server: &mut Server) -> String{
        server.stop = true;
        "Stopped".to_string()
    }
}