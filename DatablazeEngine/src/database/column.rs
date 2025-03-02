use datablaze_types::enums::{ColumnData, ColumnTypes};
use crate::database::exceptions::{DataError, IndexError};

#[derive(Debug)]
pub struct Column {
    pub name: String,
    pub column_type: ColumnTypes,
    pub data: Vec<ColumnData>,
}

impl Column {
    pub fn new(name: String, column_type: ColumnTypes) -> Column {
        Column { name, column_type, data: Vec::new() }
    }
    
    pub fn add_data(&mut self, data: ColumnData) -> Result<usize, DataError> {
        if !data.validate(self.column_type) {
            return Err(DataError);
        }
        self.data.push(data);
        Ok(self.data.len() - 1)
    }
}