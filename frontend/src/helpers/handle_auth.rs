use reqwest::get;
use serde::{Deserialize, Serialize};

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
    // TODO: Refactor this to use environment variables
    const AUTH_ENDPOINT: &str = "https://3u7ify9w39.execute-api.eu-west-1.amazonaws.com/auth";

    let response = get(format!("{}/{}", AUTH_ENDPOINT, auth_code)).await;

    let response = match response {
        Ok(response) => response,
        Err(_) => return Err(String::from("An error occurred while authenticating.")),
    };

    let json_response: Result<AuthResponse, _> = response.json().await;
    match json_response {
        Ok(auth_response) => {
            match auth_response.status {
                200 => Ok(auth_response.body),
                _ => return Err(String::from("Authentication failed.")),
            }
        },
        Err(_) => Err(String::from("Unexpected response from server.")),
    }
}
