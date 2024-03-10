use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::constants::application_endpoints::APPLICATION_API_BASE_URL;

#[derive(PartialEq, Clone)]
pub enum ConfirmSubscribeStatus {
    Loading,
    Success,
    Failure,
}

#[derive(Serialize, Deserialize, Debug)]
struct ConfirmSubscribeResponse {
    message: String,
}

pub async fn request_confirm_subscribe(id: String) -> ConfirmSubscribeStatus {
    let mut body = HashMap::new();
    body.insert("id", id);

    let client = Client::new();

    let response = client
        .patch(format!("{}/email", APPLICATION_API_BASE_URL))
        .json(&body)
        .send()
        .await;

    let response = match response {
        Ok(response) => response,
        Err(_) => return ConfirmSubscribeStatus::Failure,
    };

    if response.status() != StatusCode::OK {
        return ConfirmSubscribeStatus::Failure;
    }

    let json_response: Result<ConfirmSubscribeResponse, _> = response.json().await;
    match json_response {
        Ok(_) => ConfirmSubscribeStatus::Success,
        Err(_) => ConfirmSubscribeStatus::Failure,
    }
}
