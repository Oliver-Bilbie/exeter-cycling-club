use aws_config::{load_defaults, BehaviorVersion};
use aws_sdk_ssm as ssm;
use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Deserialize)]
struct Request {}

#[derive(Serialize)]
struct Route {
    status: String,
    id: String,
    name: String,
    message: String,
    distance: String,
    elevation_gain: String,
    map_url: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .without_time()
        .init();

    run(service_fn(clear_route)).await
}

async fn clear_route(_event: LambdaEvent<Request>) -> Result<(), Error> {
    let aws_config = load_defaults(BehaviorVersion::latest()).await;
    let ssm_client = ssm::Client::new(&aws_config);

    let route = Route {
        status: "unavailable".to_string(),
        id: "".to_string(),
        name: "".to_string(),
        message: "Subscribe to email updates to find out when a route is announced".to_string(),
        distance: "".to_string(),
        elevation_gain: "".to_string(),
        map_url: "".to_string(),
    };

    let route_data_ssm_id = env::var("ROUTE_DATA_SSM").expect("ROUTE_DATA_SSM not set");
    let route_json = serde_json::to_string(&route).expect("Unable to serialize route data");

    ssm_client
        .put_parameter()
        .name(route_data_ssm_id)
        .value(route_json)
        .overwrite(true)
        .send()
        .await?;

    Ok(())
}
