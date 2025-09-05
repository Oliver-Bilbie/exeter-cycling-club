use aws_config::{load_defaults, BehaviorVersion};
use aws_sdk_dynamodb as ddb;
use aws_sdk_sesv2 as ses;
use lambda_http::{run, service_fn, Body, Error, Request, Response};
use serde::Deserialize;
use serde_json::json;
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

    run(service_fn(unsubscribe)).await
}

async fn unsubscribe(event: Request) -> Result<Response<Body>, Error> {
    let body = read_event_body(event)?;
    let id = body.id;

    let aws_config = load_defaults(BehaviorVersion::latest()).await;
    let ddb_client = ddb::Client::new(&aws_config);
    let ses_client = ses::Client::new(&aws_config);

    let email = get_email_address(&ddb_client, &id).await?;

    let mailing_list_ddb_id =
        env::var("MAILING_LIST_TABLE_NAME").expect("MAILING_LIST_TABLE_NAME not set");

    ddb_client
        .delete_item()
        .table_name(mailing_list_ddb_id)
        .key("id", ddb::types::AttributeValue::S(id.to_string()))
        .send()
        .await?;

    match ses_client
        .delete_email_identity()
        .email_identity(email)
        .send()
        .await
    {
        Ok(_) => {}
        Err(_) => {
            println!("[INFO] Failed to delete email identity");
        }
    }

    Ok(Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(
            json!({ "message": "Unsubscribed successfully" })
                .to_string()
                .into(),
        )
        .map_err(Box::new)?)
}

async fn get_email_address(ddb_client: &ddb::Client, id: &str) -> Result<String, Error> {
    let mailing_list_ddb_id =
        env::var("MAILING_LIST_TABLE_NAME").expect("MAILING_LIST_TABLE_NAME not set");

    let ddb_resp = ddb_client
        .get_item()
        .table_name(mailing_list_ddb_id)
        .key("id", ddb::types::AttributeValue::S(id.to_string()))
        .send()
        .await?;

    let user_item = ddb_resp.item.expect("No item exists for the given id");

    let email = user_item
        .get("email")
        .expect("No email exists for the given item")
        .as_s()
        .expect("The corresponding email address is not a string")
        .to_string();

    Ok(email)
}

fn read_event_body(event: Request) -> Result<UnsubscribeRequest, Error> {
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
