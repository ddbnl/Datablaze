use std::collections::HashMap;
use std::str::FromStr;
use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use axum::extract::Path;
use axum_macros::debug_handler;
use datablaze_types::enums::{ColumnData, ColumnTypes, DatastoreVariants};
use crate::database::column::Column;
use crate::database::database::Database;
use crate::database::table::Table;
use crate::network::{
    model::{ServerState},
};
use crate::network::model::{ColumnResponseModel, DatabaseCreateRequestModel, DatabaseGetQueryModel, DatabaseResponseModel, RowCreateRequestModel, RowGetQueryModel, TableCreateRequestModel, TableGetQueryModel, TableResponseModel};

pub async fn health_checker_handler() -> impl IntoResponse {
    StatusCode::OK
}

#[debug_handler]
pub async fn database_get_handler(
    opts: Query<DatabaseGetQueryModel>,
    State(db): State<ServerState>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    
    let state = db.lock().await;
    let maybe_result = state.databases.iter().find(|db| db.name == opts.name);
    let db = match maybe_result {
        Some(db) => {db}
        None => {return Err((StatusCode::NOT_FOUND, Json(serde_json::json!({ "error": "Database not found" }))))}
    };
    let json_response = DatabaseResponseModel { name: db.name.to_string(), datastore: db.datastore.get_type().to_string() };
    Ok((StatusCode::OK, Json(json_response)))
}

#[debug_handler]
pub async fn database_create_handler(
    State(db): State<ServerState>,
    Json(body): Json<DatabaseCreateRequestModel>,
) -> impl IntoResponse {
    
    let mut state = db.lock().await;
    let datastore = match DatastoreVariants::from_str(&body.datastore) {
        Ok(datastore) => {datastore}
        Err(_) => {return StatusCode::BAD_REQUEST}
    };
    let db = Database::new(body.name.clone(), datastore);
    state.databases.push(db);

    StatusCode::CREATED
}

pub async fn table_create_handler(
    Path(db_name): Path<String>,
    State(db): State<ServerState>,
    Json(body): Json<TableCreateRequestModel>,
) -> impl IntoResponse {
    
    let mut state = db.lock().await;
    
    let maybe_database = state
        .databases
        .iter_mut()
        .find(|db| db.name == db_name);
    let database = match maybe_database {
        Some(db) => {db}
        None => {return StatusCode::NOT_FOUND}
    };
    
    let mut columns = HashMap::new();
    for request_column in body.columns.iter() {
        let maybe_column_type = ColumnTypes::from_str(request_column.column_type.as_str());
        let column_type = match maybe_column_type {
            Ok(column_type) => {column_type}
            Err(_) => {return StatusCode::BAD_REQUEST}
        };
        columns.insert(request_column.name.to_string(), Column::new(request_column.name.clone(), column_type));
    }
    let table = Table::new(body.name, columns);
    
    database.tables.push(table);
    
    StatusCode::CREATED
}

#[debug_handler]
pub async fn table_get_handler(
    opts: Query<TableGetQueryModel>,
    Path(database_name): Path<String>,
    State(db): State<ServerState>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {

    let state = db.lock().await;
    
    let maybe_database = state.databases.iter().find(|db| db.name == database_name);
    if let Some(database) = maybe_database {
        let maybe_table = database.tables.iter().find(|table| table.name == opts.name);
        if let Some(table) = maybe_table {
            let columns: Vec<ColumnResponseModel> = table
                .columns
                .values()
                .map(|column| ColumnResponseModel { name: column.name.clone(), column_type: column.column_type.to_string() })
                .collect();
            let json_response = TableResponseModel { name: table.name.to_string(), columns };
            Ok((StatusCode::OK, Json(json_response)))
        } else {
            Err((StatusCode::NOT_FOUND, Json(serde_json::json!({ "error": "Table not found" }))))
        }
    } else {
        Err((StatusCode::NOT_FOUND, Json(serde_json::json!({ "error": "Database not found" }))))
    }
}

pub async fn row_create_handler(
    Path((database_name, table_name)): Path<(String, String)>,
    State(db): State<ServerState>,
    Json(body): Json<RowCreateRequestModel>,
) -> impl IntoResponse {

    let mut state = db.lock().await;

    let maybe_database = state
        .databases
        .iter_mut()
        .find(|db| db.name == database_name);
    let database = match maybe_database {
        Some(db) => {db}
        None => {return StatusCode::NOT_FOUND}
    };
    
    let maybe_database = state.databases.iter_mut().find(|db| db.name == database_name);
    if let Some(database) = maybe_database {
        let maybe_table = database.tables.iter_mut().find(|table| table.name == table_name);
        if let Some(table) = maybe_table {
            if let Err(e) = table.add_row(body.data) {
                return StatusCode::BAD_REQUEST;
            }
        }
    }
    StatusCode::CREATED
}

#[debug_handler]
pub async fn row_get_handler(
    opts: Query<RowGetQueryModel>,
    Path((database_name, table_name)): Path<(String, String)>,
    State(db): State<ServerState>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {

    let state = db.lock().await;

    let maybe_database = state.databases.iter().find(|db| db.name == database_name);
    if let Some(database) = maybe_database {
        let maybe_table = database.tables.iter().find(|table| table.name == table_name);
        if let Some(table) = maybe_table {
            if let Ok(row) = table.get_row(opts.index) {
                Ok((StatusCode::OK, Json(row)))
            } else {
                Err((StatusCode::NOT_FOUND, Json(serde_json::json!({ "error": "Row not found" }))))
            }
        } else {
            Err((StatusCode::NOT_FOUND, Json(serde_json::json!({ "error": "Table not found" }))))
        }
    } else {
        Err((StatusCode::NOT_FOUND, Json(serde_json::json!({ "error": "Database not found" }))))
    }
}
