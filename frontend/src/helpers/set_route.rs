use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
struct SetRouteResponse {
    status: u16,
    body: String,
}

#[derive(PartialEq, Clone)]
pub struct SetRouteData {
    pub id: String,
    pub name: String,
    pub message: String,
    pub access_token: String,
}

pub async fn set_route(set_route_data: SetRouteData) -> Result<String, String> {
    // TODO: Refactor this to use environment variables
    const ROUTE_ENDPOINT: &str = "https://3u7ify9w39.execute-api.eu-west-1.amazonaws.com/route";

    let formatted_message = set_route_data.message.replace("\n", "$NEWLINE");

    let mut body = HashMap::new();
    body.insert("access_token", set_route_data.access_token);
    body.insert("id", set_route_data.id);
    body.insert("name", set_route_data.name);
    body.insert("message", formatted_message);

    let client = Client::new();
    let response = client.put(ROUTE_ENDPOINT).json(&body).send().await;
    let response = match response {
        Ok(response) => response,
        Err(_) => return Err("Error sending request".to_string()),
    };

    let json_response: Result<SetRouteResponse, _> = response.json().await;
    match json_response {
        Ok(resp) => match resp.status {
            200 => Ok("Route set successfully".to_string()),
            _ => Err("Error setting route".to_string()),
        },
        Err(_) => Err("Unexpected response from server".to_string()),
    }
}