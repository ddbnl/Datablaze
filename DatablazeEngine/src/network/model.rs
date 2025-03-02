use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use serde_json::Value;
use tokio::sync::Mutex;
use crate::database::database::Database;

#[allow(non_snake_case)]
#[derive(Default)]
pub struct Server {
    pub databases: Vec<Database>,
}

pub type ServerState = Arc<Mutex<Server>>;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DatabaseGetQueryModel {
    pub name: String,
}
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DatabaseResponseModel {
    pub name: String,
    pub datastore: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DatabaseCreateRequestModel {
    pub name: String,
    pub datastore: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TableGetQueryModel {
    pub name: String,
}
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TableCreateRequestModel {
    pub name: String,
    pub columns: Vec<ColumnCreateRequestModel>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ColumnCreateRequestModel {
    pub name: String,
    pub column_type: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TableResponseModel {
    pub name: String,
    pub columns: Vec<ColumnResponseModel>,
}


#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ColumnResponseModel {
    pub name: String,
    pub column_type: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RowCreateRequestModel {
    pub data: Vec<HashMap<String, Value>>
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RowUpdateRequestModel {
    pub data: HashMap<String, Value>
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RowGetQueryModel {
    pub index: u64,
}
