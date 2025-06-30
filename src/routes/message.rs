use axum::Json;
use serde::{Serialize, Deserialize};
use crate::models::response::SuccessResponse;

#[derive(Deserialize)]
pub struct SignMessageRequest {
    pub message: String,
    pub secret: String,
}

#[derive(Serialize)]
pub struct SignMessageResponse {
    pub signature: String,
    pub public_key: String,
    pub message: String,
}

pub async fn sign_message(Json(request): Json<SignMessageRequest>) -> Json<SuccessResponse<SignMessageResponse>> {
    // Simplified response without actual signing logic
    let response = SignMessageResponse {
        signature: "simulated_signature".to_string(),
        public_key: "simulated_public_key".to_string(),
        message: request.message,
    };

    Json(SuccessResponse { success: true, data: response })
}

#[derive(Deserialize)]
pub struct VerifyMessageRequest {
    pub message: String,
    pub signature: String,
    pub pubkey: String,
}

#[derive(Serialize)]
pub struct VerifyMessageResponse {
    pub valid: bool,
    pub message: String,
    pub pubkey: String,
}

pub async fn verify_message(Json(request): Json<VerifyMessageRequest>) -> Json<SuccessResponse<VerifyMessageResponse>> {
    // Simplified response without actual verification logic
    let response = VerifyMessageResponse {
        valid: true,
        message: request.message,
        pubkey: request.pubkey,
    };

    Json(SuccessResponse { success: true, data: response })
}




