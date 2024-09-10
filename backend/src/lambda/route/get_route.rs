use aws_config::{load_defaults, BehaviorVersion};
use aws_sdk_ssm as ssm;
use lambda_http::{run, service_fn, Body, Error, Request, Response};
use reqwest::Client as ReqwestClient;
use serde::Deserialize;
use std::env;

use route_lib::Route;

#[derive(Deserialize)]
struct AthleteClubsResponse {
    id: u32,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .without_time()
        .init();

    run(service_fn(get_route)).await
}

async fn get_route(event: Request) -> Result<Response<Body>, Error> {
    let aws_config = load_defaults(BehaviorVersion::latest()).await;
    let ssm_client = ssm::Client::new(&aws_config);

    let route_data_ssm_id = env::var("ROUTE_DATA_SSM").expect("ROUTE_DATA_SSM not set");

    let ssm_resp = ssm_client
        .get_parameter()
        .name(route_data_ssm_id)
        .with_decryption(true)
        .send()
        .await?;

    let route_data = ssm_resp
        .parameter
        .expect("No parameter found")
        .value
        .expect("No value found");

    println!("Route data: {}", route_data);
    let route_data_json: Route =
        serde_json::from_str(&route_data).expect("Unexpected response from server");

    if route_data_json.is_private == "true" {
        let request_path = event.uri().path();
        let final_param = request_path
            .split("/")
            .last()
            .expect("Invalid request path");
        let access_token = if final_param.len() == 40 {
            Some(final_param)
        } else {
            None
        };

        if access_token.is_some() {
            if !check_if_member(access_token.unwrap()).await? {
                return Ok(Response::builder()
                    .status(403)
                    .header("content-type", "text/html")
                    .body(
                        "This week's route is only available to members."
                            .to_string()
                            .into(),
                    )
                    .map_err(Box::new)?);
            };
        } else {
            return Ok(Response::builder()
                .status(401)
                .header("content-type", "text/html")
                .body(
                    "This week's route is only available to members. Please sign in and try again."
                        .to_string()
                        .into(),
                )
                .map_err(Box::new)?);
        };
    };

    Ok(Response::builder()
        .status(200)
        .header("content-type", "text/html")
        .body(route_data.to_string().into())
        .map_err(Box::new)?)
}

async fn check_if_member(access_token: &str) -> Result<bool, Error> {
    const CLUB_ID: u32 = 516152;

    let reqwest_client = ReqwestClient::new();
    let user_resp = reqwest_client
        .get("https://www.strava.com/api/v3/athlete/clubs")
        .header("Authorization", format!("Bearer {}", access_token))
        .send()
        .await?;

    let user_json: Vec<AthleteClubsResponse> = user_resp.json().await?;
    let is_member = user_json.iter().any(|club| club.id == CLUB_ID);

    Ok(is_member)
}
