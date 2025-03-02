use std::collections::HashMap;
use serde_json::Value;
use datablaze_types::enums::{ColumnData, ColumnTypes};
use crate::database::column::Column;
use crate::database::exceptions::DataError;

pub struct Table {
    pub name: String,
    pub columns: HashMap<String, Column>,
}

impl Table {
    pub fn new(name: String, columns: HashMap<String, Column>) -> Table {
        Table { name, columns }
    }

    pub fn get_row(&self, row: u64) -> Result<HashMap<String, Value>, DataError> {
        let mut result = HashMap::new();
        for (column_name, column) in self.columns.iter() {
            if row >= column.data.len() as u64 { return Err(DataError) };
            let data = match column.column_type {
                ColumnTypes::String => {
                    if let ColumnData::String(data) = &column.data[row as usize] {
                        Value::String(data.to_string())
                    } else {
                        return Err(DataError)
                    }
                }
                ColumnTypes::Int => {
                    if let ColumnData::Int(number) = column.data[row as usize] {
                        Value::Number(number.into())
                    } else {
                        return Err(DataError)
                    }
                }
            };
            result.insert(column_name.to_string(), data);
        }
        Ok(result)

    }
    pub fn add_row(&mut self, row: HashMap<String, Value>) -> Result<(), DataError> {

        if row.len() != self.columns.len() { return Err(DataError) };
        for (column_name, column_data) in row {
            let column = self.columns.get_mut(&column_name);
            if let Some(column) = column {
                let converted_data = convert_data(&column.column_type, column_data)?;
                column.add_data(converted_data)?;
            } else {
                return Err(DataError);
            }
        }
        Ok(())
    }

    pub fn update_row(&mut self, row: HashMap<String, Value>, index: u64) -> Result<(), DataError> {

        if row.len() != self.columns.len() { return Err(DataError) };
        for (column_name, column_data) in row {
            let column = self.columns.get_mut(&column_name);
            if let Some(column) = column {
                let converted_data = convert_data(&column.column_type, column_data)?;
                column.data[index as usize] = converted_data;
            } else {
                return Err(DataError);
            }
        }
        Ok(())
    }

    pub fn delete_row(&mut self, index: u64) -> Result<(), DataError> {
        for column in self.columns.values_mut() {
            if index >= column.data.len() as u64 { return Err(DataError) };
            column.data.remove(index as usize);
        }
        Ok(())
    }
}


fn convert_data(column_type: &ColumnTypes, data: Value) -> Result<ColumnData, DataError> {
    match column_type {
        ColumnTypes::String => {
            let maybe_data = data.as_str();
            if let Some(data) = maybe_data {
                Ok(ColumnData::String(data.to_string()))
            } else {
                Err(DataError)
            }
        }
        ColumnTypes::Int => {
            let maybe_data = data.as_u64();
            if let Some(data) = maybe_data {
                Ok(ColumnData::Int(data))
            } else {
                Err(DataError)
            }
        }
    }
}
