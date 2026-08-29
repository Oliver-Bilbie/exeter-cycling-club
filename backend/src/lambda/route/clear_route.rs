use aws_config::{load_defaults, BehaviorVersion};
use aws_sdk_ssm as ssm;
use ecc_lib::ssm as ssm_util;
use ecc_lib::Route;
use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use serde::Deserialize;
use std::env;

#[derive(Deserialize)]
struct Request {}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .without_time()
        .init();

    let aws_config = load_defaults(BehaviorVersion::latest()).await;
    let ssm_client = ssm::Client::new(&aws_config);

    run(service_fn(move |event| {
        let ssm_client = ssm_client.clone();
        async move { clear_route(event, &ssm_client).await }
    }))
    .await
}

async fn clear_route(_event: LambdaEvent<Request>, ssm_client: &ssm::Client) -> Result<(), Error> {
    let route = Route::unavailable();
    let route_data_ssm_id = env::var("ROUTE_DATA_SSM")?;
    let route_json = serde_json::to_string(&route)?;
    ssm_util::put_parameter(ssm_client, route_data_ssm_id, route_json).await?;
    Ok(())
}
