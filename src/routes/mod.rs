pub mod keypair;
pub mod token;
pub mod message;
pub mod send;

use axum::{Router, routing::post};
use keypair::generate_keypair;
use token::{create_token, mint_token};
use message::{sign_message, verify_message};
use send::{send_sol, send_token};

pub fn create_routes() -> Router {
    Router::new()
        .route("/keypair", post(generate_keypair))
        .route("/token/create", post(create_token))
        .route("/token/mint", post(mint_token))
        .route("/message/sign", post(sign_message))
        .route("/message/verify", post(verify_message))
        .route("/send/sol", post(send_sol))
        .route("/send/token", post(send_token))
}






