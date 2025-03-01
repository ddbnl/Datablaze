use crate::commands::command::Command;
use crate::DatastoreVariants;


// Create database
pub struct CreateDatabaseCommand {
    name: String,
    datastore_variant: DatastoreVariants
}

impl Default for CreateDatabaseCommand {
    fn default() -> Self {
        CreateDatabaseCommand { name: String::new(), datastore_variant: DatastoreVariants::InMemory }
    }
}

impl Command for CreateDatabaseCommand {
    fn to_command_string(&self) -> String {
        format!("create database {} {}", self.datastore_variant, self.name)
    }
    fn receive_reply(&self) -> bool {
        true
    }
}

impl CreateDatabaseCommand {
    pub fn use_in_memory(&mut self) -> &mut Self {
        self.datastore_variant = DatastoreVariants::InMemory;
        self
    }
    pub fn use_file_based(&mut self) -> &mut Self {
        self.datastore_variant = DatastoreVariants::FileBased;
        self
    }
    pub fn use_name(&mut self, name: &str) -> &mut Self {
        self.name = name.to_string();
        self
    }
}

// Get database
#[derive(Default)]
pub struct GetDatabaseCommand {
    name: String,
}

impl Command for GetDatabaseCommand {
    fn to_command_string(&self) -> String {
        format!("get database {}", self.name)
    }
    fn receive_reply(&self) -> bool {
        true
    }
}

impl GetDatabaseCommand {
    pub fn use_name(&mut self, name: &str) -> &mut Self {
        self.name = name.to_string();
        self
    }
}


// Stop server
#[derive(Default)]
pub struct StopServerCommand;

impl Command for StopServerCommand {
    fn to_command_string(&self) -> String {
        "stop".to_string()
    }
    fn receive_reply(&self) -> bool {
        false
    }
}