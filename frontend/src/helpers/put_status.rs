use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::constants::application_endpoints::APPLICATION_API_BASE_URL;
use crate::helpers::form_state::RequestState;

#[derive(Serialize, Deserialize, Debug)]
struct PutStatusResponse {
    message: String,
}

pub async fn put_status(id: String, status: String) -> RequestState {
    let mut body = HashMap::new();
    body.insert("id", id);
    body.insert("status", status);

    let client = Client::new();

    let response = client
        .put(format!("{}/status", APPLICATION_API_BASE_URL))
        .json(&body)
        .send()
        .await;

    let response = match response {
        Ok(response) => response,
        Err(_) => return RequestState::Failure,
    };

    if response.status() != StatusCode::OK {
        return RequestState::Failure;
    }

    let json_response: Result<PutStatusResponse, _> = response.json().await;
    match json_response {
        Ok(_) => RequestState::Success,
        Err(_) => RequestState::Failure,
    }
}
