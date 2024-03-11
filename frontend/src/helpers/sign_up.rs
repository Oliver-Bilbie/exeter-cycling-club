use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::constants::application_endpoints::APPLICATION_API_BASE_URL;

#[derive(Serialize, Deserialize)]
struct SubscribeResponse {
    message: String,
}

pub async fn sign_up(email: String, name: String) -> Result<String, String> {
    let mut map = HashMap::new();
    map.insert("email", email);
    map.insert("name", name);

    let client = Client::new();

    let response = client
        .put(format!("{}/email", APPLICATION_API_BASE_URL))
        .json(&map)
        .send()
        .await
        .expect("An error occurred while attempting to sign up. Please try again later or contact us via Facebook.");

    if response.status() != StatusCode::OK {
        return Err("An error occurred while signing up. Please try again later or contact us via Facebook.".to_string());
    }

    let json_response: SubscribeResponse = response.json().await.expect(
        "Unexpected response from server. Please try again later or contact us via Facebook.",
    );

    Ok(json_response.message)
}
