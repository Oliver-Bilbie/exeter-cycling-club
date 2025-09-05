use aws_config::{load_defaults, BehaviorVersion};
use aws_sdk_dynamodb as ddb;
use lambda_http::{run, service_fn, Body, Error, Request, Response};
use serde::Deserialize;
use serde_json::json;
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

    run(service_fn(set_attendance)).await
}

async fn set_attendance(event: Request) -> Result<Response<Body>, Error> {
    let body = read_event_body(event)?;
    let id = body.id;
    let status = body.status.to_uppercase();

    let aws_config = load_defaults(BehaviorVersion::latest()).await;
    let ddb_client = ddb::Client::new(&aws_config);

    if !validate_status(&status) {
        return Ok(Response::builder()
            .status(400)
            .header("content-type", "application/json")
            .body(json!({ "message": "Invalid status" }).to_string().into())
            .map_err(Box::new)?);
    }

    if !check_user_exists(&ddb_client, &id).await? {
        return Ok(Response::builder()
            .status(404)
            .header("content-type", "application/json")
            .body(json!({ "message": "User not found" }).to_string().into())
            .map_err(Box::new)?);
    }

    update_user_status(&ddb_client, &id, &status).await?;

    Ok(Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(
            json!({ "message": "Attendance set successfully" })
                .to_string()
                .into(),
        )
        .map_err(Box::new)?)
}

fn read_event_body(event: Request) -> Result<SetAttendanceRequest, Error> {
    let body = match event.body() {
        Body::Text(text) => serde_json::from_str(text).expect("Unable to parse body"),
        Body::Binary(input) => {
            let text = String::from_utf8(input.to_vec()).expect("Unable to parse binary body");
            serde_json::from_str(&text).expect("Unable to parse body")
        }
        _ => panic!("No event body was provided"),
    };
    Ok(body)
}

fn validate_status(status: &str) -> bool {
    match status {
        "Y" | "N" | "M" => true,
        _ => false,
    }
}

async fn check_user_exists(ddb_client: &ddb::Client, id: &str) -> Result<bool, Error> {
    let mailing_list_ddb_id =
        env::var("MAILING_LIST_TABLE_NAME").expect("MAILING_LIST_TABLE_NAME not set");

    let ddb_resp = ddb_client
        .get_item()
        .table_name(mailing_list_ddb_id)
        .key("id", ddb::types::AttributeValue::S(id.to_string()))
        .send()
        .await?;

    Ok(ddb_resp.item.is_some())
}

async fn update_user_status(ddb_client: &ddb::Client, id: &str, status: &str) -> Result<(), Error> {
    let mailing_list_ddb_id =
        env::var("MAILING_LIST_TABLE_NAME").expect("MAILING_LIST_TABLE_NAME not set");

    ddb_client
        .update_item()
        .table_name(mailing_list_ddb_id)
        .key("id", ddb::types::AttributeValue::S(id.to_string()))
        .update_expression("SET #rideStatus = :status")
        .expression_attribute_names("#rideStatus", "rideStatus")
        .expression_attribute_values(":status", ddb::types::AttributeValue::S(status.to_string()))
        .send()
        .await?;
    Ok(())
}
