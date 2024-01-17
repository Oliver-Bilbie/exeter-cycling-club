use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(PartialEq, Clone)]
pub enum AttendanceStatus {
    Loading,
    Success,
    Failure,
}

#[derive(Serialize, Deserialize, Debug)]
struct PutStatusResponse {
    status: u16,
    body: String,
}

pub async fn put_status(id: String, status: String) -> AttendanceStatus {
    // TODO: Refactor this to use environment variables
    const STATUS_ENDPOINT: &str = "https://3u7ify9w39.execute-api.eu-west-1.amazonaws.com/status";

    let mut body = HashMap::new();
    body.insert("id", id);
    body.insert("status", status);

    let client = Client::new();
    let response = client
        .put(STATUS_ENDPOINT)
        .json(&body)
        .send()
        .await;
    let response = match response {
        Ok(response) => response,
        Err(_) => return AttendanceStatus::Failure,
    };

    let json_response: Result<PutStatusResponse, _> = response.json().await;
    match json_response {
        Ok(resp) => {
            match resp.status {
                200 => AttendanceStatus::Success,
                _ => AttendanceStatus::Failure,
            }},
        Err(_) => AttendanceStatus::Failure,
    }
}
