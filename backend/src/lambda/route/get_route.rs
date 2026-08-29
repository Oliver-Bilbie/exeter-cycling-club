use aws_config::{load_defaults, BehaviorVersion};
use aws_sdk_ssm as ssm;
use ecc_lib::http::{path_param, text_response};
use ecc_lib::ssm as ssm_util;
use ecc_lib::Route;
use lambda_http::{run, service_fn, Body, Error, Request, Response};
use reqwest::Client as ReqwestClient;
use serde::Deserialize;
use std::env;

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

    let aws_config = load_defaults(BehaviorVersion::latest()).await;
    let ssm_client = ssm::Client::new(&aws_config);
    let reqwest_client = ReqwestClient::new();

    run(service_fn(move |event| {
        let ssm_client = ssm_client.clone();
        let reqwest_client = reqwest_client.clone();
        async move { get_route(event, &ssm_client, &reqwest_client).await }
    }))
    .await
}

async fn get_route(
    event: Request,
    ssm_client: &ssm::Client,
    reqwest_client: &ReqwestClient,
) -> Result<Response<Body>, Error> {
    let route_data_ssm_id = env::var("ROUTE_DATA_SSM")?;
    let route_data = ssm_util::get_parameter(ssm_client, route_data_ssm_id).await?;

    tracing::info!("Route data: {route_data}");
    let route: Route = serde_json::from_str(&route_data)?;

    if route.is_private() {
        let access_token = path_param(&event).filter(|p| p.len() == 40);

        match access_token {
            Some(token) if check_if_member(reqwest_client, token).await? => {}
            Some(_) => {
                return text_response(403, "This week's route is only available to members.");
            }
            None => {
                return text_response(
                    401,
                    "This week's route is only available to members. Please sign in and try again.",
                );
            }
        }
    }

    text_response(200, route_data)
}

async fn check_if_member(
    reqwest_client: &ReqwestClient,
    access_token: &str,
) -> Result<bool, Error> {
    const CLUB_ID: u32 = 516152;

    let clubs: Vec<AthleteClubsResponse> = reqwest_client
        .get("https://www.strava.com/api/v3/athlete/clubs")
        .header("Authorization", format!("Bearer {access_token}"))
        .send()
        .await?
        .json()
        .await?;

    Ok(clubs.iter().any(|club| club.id == CLUB_ID))
}
