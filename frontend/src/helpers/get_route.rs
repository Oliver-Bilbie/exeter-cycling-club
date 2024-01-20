use reqwest::get;
use serde::{Deserialize, Serialize};

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

pub async fn get_route() -> RouteStatus {
    const ROUTE_DATA_ENDPOINT: &str =
        "https://s3.eu-west-1.amazonaws.com/eccv2.oliver-bilbie.co.uk/routeData.json";
    let response = match get(ROUTE_DATA_ENDPOINT).await {
        Ok(response) => response,
        Err(_) => {
            return RouteStatus::Error(
                "An error occurred while loading the route.\nPlease try again later.".to_string(),
            )
        }
    };

    let json_response: Result<RouteDataResponse, _> = response.json().await;
    match json_response {
        Ok(route_data_response) => RouteStatus::from_route_data_response(route_data_response),
        Err(_) => RouteStatus::Error(
            "Unexpected response from server.\nPlease try again later.".to_string(),
        ),
    }
}
