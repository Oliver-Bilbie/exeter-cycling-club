use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::constants::application_endpoints::APPLICATION_API_BASE_URL;

#[derive(PartialEq, Clone)]
pub enum UnsubscribeStatus {
    Loading,
    Success,
    Failure,
}

#[derive(Serialize, Deserialize, Debug)]
struct UnsubscribeResponse {
    message: String,
}

pub async fn request_unsubscribe(id: String) -> UnsubscribeStatus {
    let mut body = HashMap::new();
    body.insert("id", id);

    let client = Client::new();

    let response = client
        .delete(format!("{}/email", APPLICATION_API_BASE_URL))
        .json(&body)
        .send()
        .await;

    let response = match response {
        Ok(response) => response,
        Err(_) => return UnsubscribeStatus::Failure,
    };

    if response.status() != StatusCode::OK {
        return UnsubscribeStatus::Failure;
    }

    let json_response: Result<UnsubscribeResponse, _> = response.json().await;
    match json_response {
        Ok(_) => UnsubscribeStatus::Success,
        Err(_) => UnsubscribeStatus::Failure,
    }
}
