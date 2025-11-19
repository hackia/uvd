use crate::uvd::data::Manifest;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SearchResponse {
    pub results: Vec<Manifest>,
    pub total_count: usize,
}

#[derive(Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub token: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoginResponse {
    pub success: bool,
    pub session_id: Option<String>,
    pub message: String,
}
