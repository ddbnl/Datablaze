pub use datablaze_types::enums::*;

mod datastore;
mod network;
mod database;
mod tests;

use network::route::create_router;


#[tokio::main]
async fn main() {

    let app = create_router();

    println!("🚀 Server started successfully");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
