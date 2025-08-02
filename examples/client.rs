// cargo run --example client

use serde::{Deserialize, Serialize};

// Structure for request payload
#[derive(Serialize)]
struct BmiRequest {
    height: f32,
    weight: f32,
}

// Structure for response payload
#[derive(Deserialize)]
struct BmiResponse {
    bmi: f32,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://rust-bmi-api-b4fd519caa8f.herokuapp.com/bmi";

    // Example data
    let request_body = BmiRequest { height: 1.69, weight: 69.0 };

    // Create the HTTP client
    let client = reqwest::Client::new();

    // Send the POST request with JSON body
    let response = client.post(url).json(&request_body).send().await?;

    // Ensure status is OK
    if !response.status().is_success() {
        eprintln!("Request failed with status: {}", response.status());
        return Ok(());
    }

    // Deserialize JSON response
    let bmi_response: BmiResponse = response.json().await?;

    println!("BMI = {:.2}", bmi_response.bmi);

    Ok(())
}
