use reqwest::get;
use serde::{Deserialize, Serialize};

use crate::constants::application_endpoints::APPLICATION_API_BASE_URL;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct UserData {
    pub id: String,
    pub name: String,
    pub access_token: String,
    pub admin: bool,
}

#[derive(Serialize, Deserialize)]
struct AuthResponse {
    status: u16,
    body: UserData,
}

pub async fn handle_auth(auth_code: String) -> Result<UserData, String> {
    let response = get(format!("{}/auth/{}", APPLICATION_API_BASE_URL, auth_code)).await;

    let response = match response {
        Ok(response) => response,
        Err(_) => return Err(String::from("An error occurred while authenticating.")),
    };

    let json_response: Result<AuthResponse, _> = response.json().await;
    match json_response {
        Ok(auth_response) => match auth_response.status {
            200 => Ok(auth_response.body),
            _ => return Err(String::from("Authentication failed.")),
        },
        Err(_) => Err(String::from("Unexpected response from server.")),
    }
}
