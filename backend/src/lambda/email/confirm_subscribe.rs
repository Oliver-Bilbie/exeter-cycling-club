use aws_config::{load_defaults, BehaviorVersion};
use aws_sdk_dynamodb as ddb;
use ecc_lib::http::{json_message, parse_body};
use lambda_http::{run, service_fn, Body, Error, Request, Response};
use serde::Deserialize;
use std::env;

#[derive(Deserialize)]
struct ConfirmSubscribeRequest {
    id: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .without_time()
        .init();

    let aws_config = load_defaults(BehaviorVersion::latest()).await;
    let ddb_client = ddb::Client::new(&aws_config);

    run(service_fn(move |event| {
        let ddb_client = ddb_client.clone();
        async move { confirm_subscribe(event, &ddb_client).await }
    }))
    .await
}

async fn confirm_subscribe(
    event: Request,
    ddb_client: &ddb::Client,
) -> Result<Response<Body>, Error> {
    let body = parse_body::<ConfirmSubscribeRequest>(&event)?;

    if !check_id_exists(ddb_client, &body.id).await? {
        return json_message(404, "User does not exist. Please try subscribing again.");
    }

    update_ddb_item(ddb_client, &body.id).await?;
    json_message(200, "Subscribed successfully")
}

async fn check_id_exists(ddb_client: &ddb::Client, id: &str) -> Result<bool, Error> {
    let table_name = env::var("MAILING_LIST_TABLE_NAME")?;

    let response = ddb_client
        .get_item()
        .table_name(table_name)
        .key("id", ddb::types::AttributeValue::S(id.to_string()))
        .send()
        .await?;

    Ok(response.item.is_some())
}

async fn update_ddb_item(ddb_client: &ddb::Client, id: &str) -> Result<(), Error> {
    let table_name = env::var("MAILING_LIST_TABLE_NAME")?;

    ddb_client
        .update_item()
        .table_name(table_name)
        .key("id", ddb::types::AttributeValue::S(id.to_string()))
        .update_expression("SET verified = :verified")
        .expression_attribute_values(":verified", ddb::types::AttributeValue::Bool(true))
        .send()
        .await?;

    Ok(())
}
