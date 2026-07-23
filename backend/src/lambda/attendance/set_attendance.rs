use aws_config::{load_defaults, BehaviorVersion};
use aws_sdk_dynamodb as ddb;
use ecc_lib::http::{json_message, parse_body};
use lambda_http::{run, service_fn, Body, Error, Request, Response};
use serde::Deserialize;
use std::env;

#[derive(Deserialize)]
struct SetAttendanceRequest {
    id: String,
    status: String,
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
        async move { set_attendance(event, &ddb_client).await }
    }))
    .await
}

async fn set_attendance(event: Request, ddb_client: &ddb::Client) -> Result<Response<Body>, Error> {
    let body = parse_body::<SetAttendanceRequest>(&event)?;
    let status = body.status.to_uppercase();

    if !matches!(status.as_str(), "Y" | "N" | "M") {
        return json_message(400, "Invalid status");
    }

    if !check_user_exists(ddb_client, &body.id).await? {
        return json_message(404, "User not found");
    }

    update_user_status(ddb_client, &body.id, &status).await?;
    json_message(200, "Attendance set successfully")
}

async fn check_user_exists(ddb_client: &ddb::Client, id: &str) -> Result<bool, Error> {
    let table_name = env::var("MAILING_LIST_TABLE_NAME")?;

    let resp = ddb_client
        .get_item()
        .table_name(table_name)
        .key("id", ddb::types::AttributeValue::S(id.to_string()))
        .send()
        .await?;

    Ok(resp.item.is_some())
}

async fn update_user_status(ddb_client: &ddb::Client, id: &str, status: &str) -> Result<(), Error> {
    let table_name = env::var("MAILING_LIST_TABLE_NAME")?;

    ddb_client
        .update_item()
        .table_name(table_name)
        .key("id", ddb::types::AttributeValue::S(id.to_string()))
        .update_expression("SET #rideStatus = :status")
        .expression_attribute_names("#rideStatus", "rideStatus")
        .expression_attribute_values(":status", ddb::types::AttributeValue::S(status.to_string()))
        .send()
        .await?;

    Ok(())
}
