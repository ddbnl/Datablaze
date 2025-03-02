use std::collections::HashMap;
use axum::{body::Body, http::{Request}, Router};
use axum::response::Response;
use axum::routing::RouterIntoService;
use serde_json::Value;
use tower::{Service, ServiceExt};
use crate::network::model::{DatabaseCreateRequestModel, RowCreateRequestModel, RowUpdateRequestModel, TableCreateRequestModel};

pub async fn parse_body<T>(response: Response) -> T where T: serde::de::DeserializeOwned {
    
    let body_bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .map_err(|e| format!("Failed to read body: {}", e)).unwrap();
    serde_json::from_slice(&body_bytes).unwrap()
}

pub async fn request_create_database(body: DatabaseCreateRequestModel, app: &mut RouterIntoService<Body>) -> Response {
    
    let request = Request::builder()
        .uri("/api/database")
        .method("POST")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_string(&body).unwrap())).unwrap();
    ServiceExt::<Request<Body>>::ready(app)
        .await
        .unwrap()
        .call(request)
        .await
        .unwrap()
}

pub async fn request_get_database(database_name: &str, app: &mut RouterIntoService<Body>) -> Response {

    let request = Request::builder()
        .uri(format!("/api/database?name={}", database_name))
        .method("GET")
        .body(Body::default()).unwrap();
    ServiceExt::<Request<Body>>::ready(app)
        .await
        .unwrap()
        .call(request)
        .await
        .unwrap()
}

pub async fn request_create_table(
    database_name: String,
    body: TableCreateRequestModel,
    app: &mut RouterIntoService<Body>) -> Response {

    let request = Request::builder()
        .uri(format!("/api/database/{}/table", database_name))
        .method("POST")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_string(&body).unwrap())).unwrap();
    ServiceExt::<Request<Body>>::ready(app)
        .await
        .unwrap()
        .call(request)
        .await
        .unwrap()
}

pub async fn request_get_table(database_name: String, table_name: &str, app: &mut RouterIntoService<Body>) -> Response {
    
    let request = Request::builder()
        .uri(format!("/api/database/{}/table?name={}", database_name, table_name))
        .method("GET")
        .body(Body::default()).unwrap();
    ServiceExt::<Request<Body>>::ready(app)
        .await
        .unwrap()
        .call(request)
        .await
        .unwrap()
}

pub async fn request_create_row(
    
    database_name: String,
    table_name: String,
    body: RowCreateRequestModel,
    app: &mut RouterIntoService<Body>) -> Response {

    let request = Request::builder()
        .uri(format!("/api/database/{}/table/{}/row", database_name, table_name))
        .method("POST")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_string(&body).unwrap())).unwrap();
    ServiceExt::<Request<Body>>::ready(app)
        .await
        .unwrap()
        .call(request)
        .await
        .unwrap()
}

pub async fn request_update_row(

    database_name: String,
    table_name: String,
    index: u64,
    body: RowUpdateRequestModel,
    app: &mut RouterIntoService<Body>) -> Response {

    let request = Request::builder()
        .uri(format!("/api/database/{}/table/{}/row?index={}", database_name, table_name, index))
        .method("PATCH")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_string(&body).unwrap())).unwrap();
    ServiceExt::<Request<Body>>::ready(app)
        .await
        .unwrap()
        .call(request)
        .await
        .unwrap()
}

pub async fn request_delete_row(

    database_name: String,
    table_name: String,
    index: u64,
    app: &mut RouterIntoService<Body>) -> Response {

    let request = Request::builder()
        .uri(format!("/api/database/{}/table/{}/row?index={}", database_name, table_name, index))
        .method("DELETE")
        .body(Body::default()).unwrap();
    ServiceExt::<Request<Body>>::ready(app)
        .await
        .unwrap()
        .call(request)
        .await
        .unwrap()
}


pub async fn request_get_row(database_name: String, table_name: &str, row: u64, app: &mut RouterIntoService<Body>) -> Response {
    let request = Request::builder()
        .uri(format!("/api/database/{}/table/{}/row?index={}", database_name, table_name, row))
        .method("GET")
        .body(Body::default()).unwrap();
    ServiceExt::<Request<Body>>::ready(app)
        .await
        .unwrap()
        .call(request)
        .await
        .unwrap()
}