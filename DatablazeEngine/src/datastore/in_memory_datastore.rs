use datablaze_types::enums::DatastoreVariants;
use crate::datastore::datastore::Datastore;

pub struct InMemoryDatastore {
    
}

impl Datastore for InMemoryDatastore {
    fn insert(&mut self) {
        todo!()
    }

    fn delete(&mut self) {
        todo!()
    }

    fn create(&mut self) {
        todo!()
    }

    fn update(&mut self) {
        todo!()
    }

    fn select(&self) {
        todo!()
    }
    fn get_type(&self) -> DatastoreVariants {
        DatastoreVariants::InMemory
    }
}