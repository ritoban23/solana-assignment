use axum::Json;
use solana_sdk::signature::{Keypair, Signer};
use bs58;
use serde::Serialize;
use crate::models::response::SuccessResponse;

#[derive(Serialize)]
pub struct KeypairResponse {
    pub pubkey: String,
    pub secret: String,
}

pub async fn generate_keypair() -> Json<SuccessResponse<KeypairResponse>> {
    let keypair = Keypair::new();
    let response = KeypairResponse {
        pubkey: bs58::encode(keypair.pubkey()).into_string(),
        secret: bs58::encode(&keypair.to_bytes()).into_string(),
    };
    Json(SuccessResponse { success: true, data: response })
}


