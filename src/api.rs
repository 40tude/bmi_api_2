use axum::{Json, http::StatusCode};
use serde::{Deserialize, Serialize};

// Structure for request body
#[derive(Deserialize)]
pub struct BmiRequest {
    pub height: f32,
    pub weight: f32,
}

// Structure for response body
#[derive(Serialize, Deserialize)]
pub struct BmiResponse {
    pub bmi: f32,
}

// Handler for the /bmi endpoint
pub async fn calculate_bmi(Json(payload): Json<BmiRequest>) -> Result<Json<BmiResponse>, (StatusCode, String)> {
    if payload.height <= 0.0 {
        return Err((StatusCode::BAD_REQUEST, "Height must be > 0".into()));
    }

    if payload.weight <= 0.0 {
        return Err((StatusCode::BAD_REQUEST, "Weight must be > 0".into()));
    }

    let bmi = payload.weight / (payload.height * payload.height);
    Ok(Json(BmiResponse { bmi: (bmi * 100.0).round() / 100.0 }))
}
