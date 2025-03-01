use strum_macros::Display;


#[derive(Copy, Clone, Display)]
pub enum DatastoreVariants {
    #[strum(serialize = "in_memory")]
    InMemory,
    #[strum(serialize = "file_based")]
    FileBased
}
