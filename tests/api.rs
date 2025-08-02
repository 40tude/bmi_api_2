use axum::{
    Router,
    body::Body,
    http::{Request, StatusCode},
    routing::post,
};
use bmi_api::api::{BmiRequest, BmiResponse, calculate_bmi};
use tower::ServiceExt; // now imported cleanly

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
