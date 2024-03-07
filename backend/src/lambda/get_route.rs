use aws_config::{load_defaults, BehaviorVersion};
use aws_sdk_ssm as ssm;
use lambda_http::{run, service_fn, Body, Error, Request, Response};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .without_time()
        .init();

    run(service_fn(get_route)).await
}

async fn get_route(_event: Request) -> Result<Response<Body>, Error> {
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

    Ok(Response::builder()
        .status(200)
        .header("content-type", "text/html")
        .body(route_data.to_string().into())
        .map_err(Box::new)?)
}
