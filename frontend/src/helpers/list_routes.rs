use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};

use crate::constants::strava_endpoints::STRAVA_API_BASE_URL;

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct RouteData {
    pub id_str: String,
    pub name: String,
    pub distance: f32,
    pub elevation_gain: f32,
    pub updated_at: String,
    pub map_urls: MapUrls,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct MapUrls {
    pub url: String,
}

pub async fn list_routes(user_id: String, user_token: String) -> Result<Vec<RouteData>, String> {
    let mut full_route_data: Vec<RouteData> = Vec::new();
    let mut page_num: u16 = 1;

    loop {
        let endpoint = format!(
            "{}/athletes/{}/routes?page={}",
            STRAVA_API_BASE_URL, user_id, page_num
        );

        let response = match Client::new()
            .get(endpoint)
            .header("Authorization", format!("Bearer {}", user_token))
            .send()
            .await
        {
            Ok(response) => response,
            Err(_) => {
                return Err(
                    "An error occurred while requesting the route.\nPlease try again later."
                        .to_string(),
                )
            }
        };

        if response.status() != StatusCode::OK {
            return Err(
                "An error occurred while loading the route.\nPlease try again later.".to_string(),
            );
        }

        let json_response: Result<Vec<RouteData>, _> = response.json().await;
        let mut route_data: Vec<RouteData> = match json_response {
            Ok(route_data_response) => route_data_response,
            Err(_) => {
                return Err("Unexpected response from server.\nPlease try again later.".to_string())
            }
        };

        if route_data.len() == 0 {
            return Ok(full_route_data);
        } else {
            full_route_data.append(&mut route_data);
            page_num += 1;
        };
    }
}
