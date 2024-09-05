use reqwest::{get, StatusCode};
use serde::{Deserialize, Serialize};

use crate::constants::application_endpoints::APPLICATION_API_BASE_URL;

#[derive(PartialEq, Clone, Debug)]
pub enum RouteStatus {
    Ready(RouteData),
    Unavailable(String),
    Cancelled(String),
    Error(String),
    Loading,
}

#[derive(PartialEq, Clone, Debug)]
pub struct RouteData {
    pub id: String,
    pub name: String,
    pub message: String,
}

impl RouteStatus {
    fn from_route_data_response(response: RouteDataResponse) -> RouteStatus {
        match response.status.as_str() {
            "ready" => RouteStatus::Ready(RouteData {
                id: response.id,
                name: response.name,
                message: response.message,
            }),
            "unavailable" => RouteStatus::Unavailable(response.message),
            "cancelled" => RouteStatus::Cancelled(response.message),
            _ => RouteStatus::Error(
                "Unexpected response from server.\nPlease try again later.".to_string(),
            ),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct RouteDataResponse {
    status: String,
    id: String,
    name: String,
    message: String,
}

pub async fn get_route(auth_token: Option<String>) -> RouteStatus {
    let endpoint = match auth_token {
        Some(token) => format!("{}/route/{}", APPLICATION_API_BASE_URL, token),
        None => format!("{}/route", APPLICATION_API_BASE_URL),
    };
    let response = match get(endpoint).await {
        Ok(response) => response,
        Err(_) => {
            return RouteStatus::Error(
                "An error occurred while requesting the route.\nPlease try again later."
                    .to_string(),
            )
        }
    };

    match response.status() {
        StatusCode::OK => {
            let json_response: Result<RouteDataResponse, _> = response.json().await;
            match json_response {
                Ok(route_data_response) => {
                    RouteStatus::from_route_data_response(route_data_response)
                }
                Err(_) => RouteStatus::Error(
                    "Unexpected response from server.\nPlease try again later.".to_string(),
                ),
            }
        }
        StatusCode::UNAUTHORIZED => {
            return RouteStatus::Error(
                "This week's route is only available to members. \nPlease sign in and try again."
                    .to_string(),
            );
        }
        StatusCode::FORBIDDEN => {
            return RouteStatus::Error(
                "This week's route is only available to members.".to_string(),
            );
        }
        _ => {
            return RouteStatus::Error(
                "An error occurred while loading the route.\nPlease try again later.".to_string(),
            );
        }
    }
}
