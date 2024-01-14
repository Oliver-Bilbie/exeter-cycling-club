use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
struct SendEmailResponse {
    status: u16,
    body: String,
}

pub async fn sign_up(email: String, name: String) -> Result<String, String> {
    let mut map = HashMap::new();
    map.insert("email", email);
    map.insert("name", name);

    let client = Client::new();
    let response = client
        // TODO: Refactor this to use environment variables
        .put("https://3u7ify9w39.execute-api.eu-west-1.amazonaws.com/email")
        .json(&map)
        .send()
        .await
        .expect("An error occurred while sending your message. Please try again later or contact us via Facebook.");

    let json_response: SendEmailResponse = response.json().await.expect(
        "Unexpected response from server. Please try again later or contact us via Facebook.",
    );

    match json_response.status {
        200 => Ok(json_response.body),
        _ => return Err(json_response.body),
    }
}
