use axum::{Router, routing::post};
mod handlers;
mod models;
mod errors;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/keypair", post(handlers::keypair::generate))
        .route("/token/create", post(handlers::token::create))
        .route("/token/mint", post(handlers::token::mint))
        .route("/message/sign", post(handlers::message::sign))
        .route("/message/verify", post(handlers::message::verify)); 

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    
    axum::serve(listener, app).await.unwrap();
}