use axum::body::to_bytes;
use axum::{
    Router,
    body::Body,
    http::{Request, StatusCode},
    routing::post,
};
// use bmi_api::api::{BmiRequest, BmiResponse, calculate_bmi};
use bmi_api::api::{BmiResponse, calculate_bmi};
use tower::ServiceExt;

fn app() -> Router {
    Router::new().route("/bmi", post(calculate_bmi))
}

#[tokio::test]
async fn test_zero_weight_should_fail() {
    let app = app();

    let payload = r#"{ "height": 1.75, "weight": 0.0 }"#;
    let request = Request::builder()
        .method("POST")
        .uri("/bmi")
        .header("Content-Type", "application/json")
        .body(Body::from(payload))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_valid_bmi_should_succeed() {
    let app = app();

    let payload = r#"{ "height": 1.75, "weight": 70.0 }"#;
    let request = Request::builder()
        .method("POST")
        .uri("/bmi")
        .header("Content-Type", "application/json")
        .body(Body::from(payload))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    // Check HTTP 200
    assert_eq!(response.status(), StatusCode::OK);

    // Extract JSON body
    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let parsed: BmiResponse = serde_json::from_slice(&body).unwrap();

    // Check BMI value (rounded to 2 decimals)
    assert!((parsed.bmi - 22.86).abs() < 0.01, "Expected ≈22.86, got {}", parsed.bmi);
}
