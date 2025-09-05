use aws_config::{load_defaults, BehaviorVersion};
use aws_sdk_dynamodb as ddb;
use lambda_http::{run, service_fn, Body, Error, Request, Response};
use serde::Deserialize;
use serde_json::json;
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

    run(service_fn(confirm_subscribe)).await
}

async fn confirm_subscribe(event: Request) -> Result<Response<Body>, Error> {
    let body = read_event_body(event)?;
    let id = body.id;

    let aws_config = load_defaults(BehaviorVersion::latest()).await;
    let ddb_client = ddb::Client::new(&aws_config);

    let id_exists = check_id_exists(&ddb_client, &id).await?;
    if !id_exists {
        return Ok(Response::builder()
            .status(404)
            .header("content-type", "application/json")
            .body(
                json!({ "message": "User does not exist. Please try subscribing again." })
                    .to_string()
                    .into(),
            )
            .map_err(Box::new)?);
    }

    update_ddb_item(&ddb_client, &id).await?;

    Ok(Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(
            json!({ "message": "Subscribed successfully" })
                .to_string()
                .into(),
        )
        .map_err(Box::new)?)
}

fn read_event_body(event: Request) -> Result<ConfirmSubscribeRequest, Error> {
    let body = match event.body() {
        Body::Text(text) => serde_json::from_str(&text).expect("Unable to parse text body"),
        Body::Binary(input) => {
            let text = String::from_utf8(input.to_vec()).expect("Unable to parse binary body");
            serde_json::from_str(&text).expect("Unable to parse body")
        }
        _ => panic!("No event body was provided"),
    };
    Ok(body)
}

async fn check_id_exists(ddb_client: &ddb::Client, id: &str) -> Result<bool, Error> {
    let mailing_list_ddb_id =
        env::var("MAILING_LIST_TABLE_NAME").expect("MAILING_LIST_TABLE_NAME not set");

    let response = ddb_client
        .get_item()
        .table_name(mailing_list_ddb_id)
        .key("id", ddb::types::AttributeValue::S(id.to_string()))
        .send()
        .await?;

    match response.item {
        Some(_) => Ok(true),
        None => Ok(false),
    }
}

async fn update_ddb_item(ddb_client: &ddb::Client, id: &str) -> Result<(), Error> {
    let mailing_list_ddb_id =
        env::var("MAILING_LIST_TABLE_NAME").expect("MAILING_LIST_TABLE_NAME not set");

    ddb_client
        .update_item()
        .table_name(mailing_list_ddb_id)
        .key("id", ddb::types::AttributeValue::S(id.to_string()))
        .update_expression("SET verified = :verified")
        .expression_attribute_values(":verified", ddb::types::AttributeValue::Bool(true))
        .send()
        .await?;

    Ok(())
}
