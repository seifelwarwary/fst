mod files;

use axum::{
    routing::post,
    Router,
};

#[tokio::main]
async fn main() {

    let app = Router::new().route("/get", post(files::read_file))
        .route("/", post(files::create_file));


    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}