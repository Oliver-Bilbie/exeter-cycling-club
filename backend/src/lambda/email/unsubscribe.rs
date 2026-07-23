use aws_config::{load_defaults, BehaviorVersion};
use aws_sdk_dynamodb as ddb;
use aws_sdk_sesv2 as ses;
use ecc_lib::http::{json_message, parse_body};
use lambda_http::{run, service_fn, Body, Error, Request, Response};
use serde::Deserialize;
use std::env;

#[derive(Deserialize)]
struct UnsubscribeRequest {
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
    let ses_client = ses::Client::new(&aws_config);

    run(service_fn(move |event| {
        let ddb_client = ddb_client.clone();
        let ses_client = ses_client.clone();
        async move { unsubscribe(event, &ddb_client, &ses_client).await }
    }))
    .await
}

async fn unsubscribe(
    event: Request,
    ddb_client: &ddb::Client,
    ses_client: &ses::Client,
) -> Result<Response<Body>, Error> {
    let body = parse_body::<UnsubscribeRequest>(&event)?;
    let email = get_email_address(ddb_client, &body.id).await?;
    let table_name = env::var("MAILING_LIST_TABLE_NAME")?;

    ddb_client
        .delete_item()
        .table_name(table_name)
        .key("id", ddb::types::AttributeValue::S(body.id))
        .send()
        .await?;

    if ses_client
        .delete_email_identity()
        .email_identity(email)
        .send()
        .await
        .is_err()
    {
        tracing::info!("Failed to delete email identity");
    }

    json_message(200, "Unsubscribed successfully")
}

async fn get_email_address(ddb_client: &ddb::Client, id: &str) -> Result<String, Error> {
    let table_name = env::var("MAILING_LIST_TABLE_NAME")?;

    let item = ddb_client
        .get_item()
        .table_name(table_name)
        .key("id", ddb::types::AttributeValue::S(id.to_string()))
        .send()
        .await?
        .item
        .ok_or("No item exists for the given id")?;

    item.get("email")
        .ok_or("No email exists for the given item")?
        .as_s()
        .map(|s| s.to_string())
        .map_err(|_| Error::from("The corresponding email address is not a string"))
}
