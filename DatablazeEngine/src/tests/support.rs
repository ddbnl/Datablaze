use axum::{body::Body, http::{Request}, Router};
use axum::response::Response;
use axum::routing::RouterIntoService;
use tower::{Service, ServiceExt};
use crate::network::model::{DatabaseCreateRequestModel, RowCreateRequestModel, TableCreateRequestModel};
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