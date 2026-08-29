use aws_config::{load_defaults, BehaviorVersion};
use aws_sdk_dynamodb as ddb;
use aws_sdk_sesv2 as ses;
use ecc_lib::email as email_util;
use ecc_lib::http::{json_message, parse_body};
use lambda_http::{run, service_fn, Body, Error, Request, Response};
use serde::Deserialize;
use std::env;
use uuid::Uuid;

#[derive(Deserialize)]
struct SubscribeRequest {
    name: String,
    email: String,
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
        async move { subscribe(event, &ddb_client, &ses_client).await }
    }))
    .await
}

async fn subscribe(
    event: Request,
    ddb_client: &ddb::Client,
    ses_client: &ses::Client,
) -> Result<Response<Body>, Error> {
    let body = parse_body::<SubscribeRequest>(&event)?;

    // Return an identical response to a success to avoid leaking email addresses
    if check_email_exists(ddb_client, &body.email).await? {
        return json_message(200, "Subscribed successfully");
    }

    let id = Uuid::new_v4().to_string();
    send_verification_email(ses_client, &body.email, &id).await?;
    write_to_ddb(ddb_client, &body.name, &body.email, &id).await?;

    json_message(200, "Subscribed successfully")
}

async fn check_email_exists(ddb_client: &ddb::Client, email: &str) -> Result<bool, Error> {
    let table_name = env::var("MAILING_LIST_TABLE_NAME")?;

    let count = ddb_client
        .query()
        .table_name(table_name)
        .index_name("EmailIndex")
        .key_condition_expression("#email = :email")
        .expression_attribute_names("#email", "email")
        .expression_attribute_values(":email", ddb::types::AttributeValue::S(email.to_string()))
        .send()
        .await?
        .count;

    Ok(count > 0)
}

async fn send_verification_email(
    ses_client: &ses::Client,
    email: &str,
    id: &str,
) -> Result<(), Error> {
    email_util::send_html(
        ses_client,
        email.to_string(),
        "Confirm your subscription",
        build_email_body(id),
    )
    .await
}

fn build_email_body(id: &str) -> String {
    include_str!("../../templates/confirm.html").replace("%RECIPIENT_ID%", id)
}

async fn write_to_ddb(
    ddb_client: &ddb::Client,
    name: &str,
    email: &str,
    id: &str,
) -> Result<(), Error> {
    let table_name = env::var("MAILING_LIST_TABLE_NAME")?;

    ddb_client
        .put_item()
        .table_name(table_name)
        .item("id", ddb::types::AttributeValue::S(id.to_string()))
        .item("name", ddb::types::AttributeValue::S(name.to_string()))
        .item("email", ddb::types::AttributeValue::S(email.to_string()))
        .item("verified", ddb::types::AttributeValue::Bool(false))
        .item("rideStatus", ddb::types::AttributeValue::S("N".to_string()))
        .send()
        .await?;

    Ok(())
}
