use axum::Json;
use serde::Serialize;
use crate::models::response::SuccessResponse;
use crate::models::request::{SendSolRequest, SendTokenRequest};

#[derive(Serialize)]
pub struct InstructionResponse {
    pub program_id: String,
    pub accounts: Vec<AccountMetaResponse>,
    pub instruction_data: String,
}

#[derive(Serialize)]
pub struct AccountMetaResponse {
    pub pubkey: String,
    pub is_signer: bool,
    pub is_writable: bool,
}

pub async fn send_sol(Json(_request): Json<SendSolRequest>) -> Json<SuccessResponse<InstructionResponse>> {
    // Simplified response without actual transfer logic
    let response = InstructionResponse {
        program_id: "system_program".to_string(),
        accounts: vec![],
        instruction_data: "simulated_instruction_data".to_string(),
    };

    Json(SuccessResponse { success: true, data: response })
}

pub async fn send_token(Json(_request): Json<SendTokenRequest>) -> Json<SuccessResponse<InstructionResponse>> {
    // Simplified response without actual transfer logic
    let response = InstructionResponse {
        program_id: "token_program".to_string(),
        accounts: vec![],
        instruction_data: "simulated_instruction_data".to_string(),
    };

    Json(SuccessResponse { success: true, data: response })
}




