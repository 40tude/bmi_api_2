// use axum::{Json, Router, routing::post};
// use axum::{http::StatusCode, response::IntoResponse};
// use serde::{Deserialize, Serialize};
// use std::net::SocketAddr;

// // Structure for request body
// #[derive(Deserialize)]
// struct BmiRequest {
//     height: f32, // in meters
//     weight: f32, // in kilograms
// }

// // Structure for response body
// #[derive(Serialize)]
// struct BmiResponse {
//     bmi: f32,
// }

// // Handler for the /bmi endpoint
// async fn calculate_bmi(Json(payload): Json<BmiRequest>) -> Result<Json<BmiResponse>, (StatusCode, String)> {
//     if payload.height <= 0.0 {
//         return Err((StatusCode::BAD_REQUEST, "Height must be > 0".into()));
//     }

//     let bmi = payload.weight / (payload.height * payload.height);
//     Ok(Json(BmiResponse { bmi: (bmi * 100.0).round() / 100.0 }))
// }

// // Handler for the / endpoint
// async fn health_check() -> impl IntoResponse {
//     (StatusCode::OK, "BMI API is up and running")
// }

// #[tokio::main]
// async fn main() {
//     // Define the router and route
//     let app = Router::new().route("/bmi", post(calculate_bmi)).route("/", axum::routing::get(health_check));

//     // Heroku provides $PORT env variable
//     let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string()).parse().expect("PORT must be a number");

//     let addr = SocketAddr::from(([0, 0, 0, 0], port));
//     println!("Listening on {addr}");

//     axum_server::bind(addr).serve(app.into_make_service()).await.unwrap();
// }

use axum::{Router, http::StatusCode, response::IntoResponse, routing::post};
use bmi_api::api::calculate_bmi;
use std::net::SocketAddr;

async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, "BMI API is up and running")
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/bmi", post(calculate_bmi)).route("/", axum::routing::get(health_check));

    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string()).parse().expect("PORT must be a number");
    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    println!("Listening on {addr}");

    axum_server::bind(addr).serve(app.into_make_service()).await.unwrap();
}
