use std::sync::Arc;
use axum::{
    routing::{get, post},
    Router,
};
use axum::routing::{delete, patch};
use tokio::sync::Mutex;
use crate::network::{
    model::Server,
    handler::*
};
use crate::network::handler::table_create_handler;

pub fn create_router() -> Router {

    let state = Arc::new(Mutex::new(Server::default()));

    Router::new()
        .route("/api/health", get(health_checker_handler))
        .route("/api/database", post(database_create_handler))
        .route("/api/database", get(database_get_handler))
        .route("/api/database/{database_name}/table", post(table_create_handler))
        .route("/api/database/{database_name}/table", get(table_get_handler))
        .route("/api/database/{database_name}/table/{table_name}/row", post(row_create_handler))
        .route("/api/database/{database_name}/table/{table_name}/row", get(row_get_handler))
        .route("/api/database/{database_name}/table/{table_name}/row", patch(row_update_handler))
        .route("/api/database/{database_name}/table/{table_name}/row", delete(row_delete_handler))
        .with_state(state)
}