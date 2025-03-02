use strum_macros::{Display, EnumString};


#[derive(Copy, Clone, Display, EnumString, Debug)]
pub enum DatastoreVariants {
    #[strum(serialize = "in_memory")]
    InMemory,
    #[strum(serialize = "file_based")]
    FileBased
}

#[derive(Copy, Clone, Display, EnumString, Debug)]
pub enum ColumnTypes {
    #[strum(serialize = "string")]
    String,
    #[strum(serialize = "int")]
    Int
}

#[derive(Clone, Display, EnumString, Debug)]
pub enum ColumnData {
    String(String),
    Int(u64)
}
impl ColumnData {
    pub fn validate(&self, data: ColumnTypes) -> bool {
        match data {
            ColumnTypes::String => matches!(self, ColumnData::String(_)),
            ColumnTypes::Int => matches!(self, ColumnData::Int(_)),
        }
    }
}
impl From<String> for ColumnData {
    fn from(data: String) -> Self {
        ColumnData::String(data)
    }
}

impl From<u64> for ColumnData {
    fn from(data: u64) -> Self {
        ColumnData::Int(data)
    }
}