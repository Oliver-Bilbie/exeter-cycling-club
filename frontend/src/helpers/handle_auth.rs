use reqwest::{get, StatusCode};
use serde::{Deserialize, Serialize};

use crate::constants::application_endpoints::APPLICATION_API_BASE_URL;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct UserData {
    pub id: String,
    pub name: String,
    pub access_token: String,
    pub admin: bool,
}

pub async fn handle_auth(auth_code: String) -> Result<UserData, String> {
    let response = get(format!("{}/auth/{}", APPLICATION_API_BASE_URL, auth_code)).await;

    let response = match response {
        Ok(response) => response,
        Err(_) => {
            return Err(String::from(
                "An error occurred while requesting authentication.",
            ))
        }
    };

    if response.status() != StatusCode::OK {
        return Err(String::from("An error occurred while authenticating."));
    }

    let json_response: Result<UserData, _> = response.json().await;

    match json_response {
        Ok(user_data) => Ok(user_data),
        Err(_) => Err(String::from("Authentication request unsuccessful.")),
    }
}
