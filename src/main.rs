mod files;

use axum::{
    routing::{get, post},
    Router,
};

#[tokio::main]
async fn main() {

    let app = Router::new().route("/file/:file_name", get(files::read_file))
        .route("/upload", post(files::create_file));


    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}