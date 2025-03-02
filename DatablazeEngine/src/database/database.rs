use crate::database::table::Table;
use crate::DatastoreVariants;
use crate::datastore::datastore::Datastore;
use crate::datastore::file_based_datastore::FileBasedDatastore;
use crate::datastore::in_memory_datastore::InMemoryDatastore;

pub struct Database {
    pub name: String,
    pub datastore: Box<dyn Datastore + Send>,
    pub tables: Vec<Table>,
}

impl Database {
    pub fn new(name: String, datastore_variant: DatastoreVariants) -> Database {
        let datastore: Box<dyn Datastore + Send> = match datastore_variant {
            DatastoreVariants::InMemory => Box::new(InMemoryDatastore { }),
            DatastoreVariants::FileBased => Box::new(FileBasedDatastore { })
        };
        Database { name, datastore, tables: Vec::new() }
    }
}