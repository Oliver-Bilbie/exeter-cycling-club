use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::constants::application_endpoints::APPLICATION_API_BASE_URL;

#[derive(Serialize, Deserialize)]
struct SendEmailResponse {
    message: String,
}

pub async fn send_email(contact_email: String, message: String) -> Result<String, String> {
    let mut map = HashMap::new();
    map.insert("contact_email", contact_email);
    map.insert("message", message);

    let client = Client::new();

    let response = client
        .post(format!("{}/contact", APPLICATION_API_BASE_URL))
        .json(&map)
        .send()
        .await
        .expect("An error occurred while sending your message. Please try again later or contact us via Facebook.");

    if response.status() != StatusCode::OK {
        return Err("An error occurred while delivering your message. Please try again later or contact us via Facebook.".to_string());
    }

    let json_response: Result<SendEmailResponse, _> = response.json().await;
    match json_response {
        Ok(json_response) => Ok(json_response.message),
        Err(_) => Err(
            "Unexpected response from server. Please try again later or contact us via Facebook."
                .to_string(),
        ),
    }
}
