use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(PartialEq, Clone)]
pub enum UnsubscribeStatus {
    Loading,
    Success,
    Failure,
}

#[derive(Serialize, Deserialize, Debug)]
struct UnsubscribeResponse {
    status: u16,
    body: String,
}

pub async fn request_unsubscribe(id: String) -> UnsubscribeStatus {
    // TODO: Refactor this to use environment variables
    const UNSUBSCRIBE_ENDPOINT: &str = "https://3u7ify9w39.execute-api.eu-west-1.amazonaws.com/email";

    let mut body = HashMap::new();
    body.insert("id", id);

    let client = Client::new();
    let response = client
        .delete(UNSUBSCRIBE_ENDPOINT)
        .json(&body)
        .send()
        .await;
    let response = match response {
        Ok(response) => response,
        Err(_) => return UnsubscribeStatus::Failure,
    };

    let json_response: Result<UnsubscribeResponse, _> = response.json().await;
    match json_response {
        Ok(resp) => {
            match resp.status {
                200 => UnsubscribeStatus::Success,
                _ => UnsubscribeStatus::Failure,
            }},
        Err(_) => UnsubscribeStatus::Failure,
    }
}
