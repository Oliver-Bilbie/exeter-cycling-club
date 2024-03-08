use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::constants::application_endpoints::APPLICATION_API_BASE_URL;

#[derive(Serialize, Deserialize, Debug)]
struct SetRouteResponse {
    message: String,
}

#[derive(PartialEq, Clone)]
pub struct CancelRouteData {
    pub message: String,
    pub access_token: String,
}

pub async fn cancel_route(cancel_route_data: CancelRouteData) -> Result<String, String> {
    let formatted_message = cancel_route_data.message.replace("\n", "$NEWLINE");

    let mut body = HashMap::new();
    body.insert("access_token", cancel_route_data.access_token);
    body.insert("message", formatted_message);

    let client = Client::new();

    let response = client
        .delete(format!("{}/route", APPLICATION_API_BASE_URL))
        .json(&body)
        .send()
        .await;

    let response = match response {
        Ok(response) => response,
        Err(_) => return Err("Error sending request".to_string()),
    };

    if response.status() != StatusCode::OK {
        return Err("Error cancelling route".to_string());
    }

    let json_response: Result<SetRouteResponse, _> = response.json().await;
    match json_response {
        Ok(resp) => Ok(resp.message),
        Err(_) => Err("Unexpected response from server".to_string()),
    }
}
