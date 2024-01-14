use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
struct SendEmailResponse {
    status: u16,
    body: String,
}

pub async fn send_email(contact_email: String, message: String) -> Result<String, String> {
    let mut map = HashMap::new();
    map.insert("contact_email", contact_email);
    map.insert("message", message);

    let client = Client::new();
    let response = client
        .post("https://nyxlz4uure.execute-api.eu-west-1.amazonaws.com/contact")
        .json(&map)
        .send()
        .await
        .expect("An error occurred while sending your message. Please try again later or contact us via Facebook.");

    let json_response: SendEmailResponse = response
        .json()
        .await
        .expect("Unexpected response from server. Please try again later or contact us via Facebook.");

    match json_response.status {
        200 => Ok(json_response.body),
        _ => return Err(json_response.body),
    }
}
