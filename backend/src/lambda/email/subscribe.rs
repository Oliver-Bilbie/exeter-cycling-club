use aws_config::{load_defaults, BehaviorVersion};
use aws_sdk_dynamodb as ddb;
use aws_sdk_sesv2 as ses;
use lambda_http::{run, service_fn, Body, Error, Request, Response};
use serde::Deserialize;
use serde_json::json;
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

    run(service_fn(subscribe)).await
}

async fn subscribe(event: Request) -> Result<Response<Body>, Error> {
    let body = read_event_body(event)?;
    let name = body.name;
    let email = body.email;

    let aws_config = load_defaults(BehaviorVersion::latest()).await;
    let ddb_client = ddb::Client::new(&aws_config);
    let ses_client = ses::Client::new(&aws_config);

    let email_exists = check_email_exists(&ddb_client, &email).await?;
    if email_exists {
        return Ok(Response::builder()
            .status(400)
            .header("content-type", "text/html")
            .body(
                json!({ "message": "Email is already subscribed" })
                    .to_string()
                    .into(),
            )
            .map_err(Box::new)?);
    }

    let id = Uuid::new_v4().to_string();

    send_verification_email(&ses_client, &email, &id).await?;

    write_to_ddb(&ddb_client, &name, &email, &id).await?;

    Ok(Response::builder()
        .status(200)
        .header("content-type", "text/html")
        .body(
            json!({ "message": "Subscribed successfully" })
                .to_string()
                .into(),
        )
        .map_err(Box::new)?)
}

fn read_event_body(event: Request) -> Result<SubscribeRequest, Error> {
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

async fn check_email_exists(ddb_client: &ddb::Client, email: &str) -> Result<bool, Error> {
    let mailing_list_ddb_id =
        env::var("MAILING_LIST_TABLE_NAME").expect("MAILING_LIST_TABLE_NAME not set");

    let email_exists = ddb_client
        .query()
        .table_name(mailing_list_ddb_id)
        .index_name("EmailIndex")
        .key_condition_expression("#email = :email")
        .expression_attribute_names("#email", "email")
        .expression_attribute_values(":email", ddb::types::AttributeValue::S(email.to_string()))
        .send()
        .await?
        .count
        > 0;
    Ok(email_exists)
}

async fn send_verification_email(
    ses_client: &ses::Client,
    email: &str,
    id: &String,
) -> Result<(), Error> {
    let mut destination: ses::types::Destination = ses::types::Destination::builder().build();
    destination.to_addresses = Some(vec![email.to_string()]);

    let subject_content = ses::types::Content::builder()
        .data("Confirm your subscription")
        .charset("UTF-8")
        .build()
        .expect("Unable to build subject content");

    let body_content = ses::types::Content::builder()
        .data(build_email_body(id))
        .charset("UTF-8")
        .build()
        .expect("Unable to build body content");

    let body = ses::types::Body::builder().html(body_content).build();

    let message = ses::types::Message::builder()
        .subject(subject_content)
        .body(body)
        .build();

    let email_content = ses::types::EmailContent::builder().simple(message).build();

    ses_client
        .send_email()
        // TODO: Replace with production domain
        .from_email_address("Exeter Cycling Club <ecc@oliver-bilbie.co.uk>")
        .destination(destination)
        .content(email_content)
        .send()
        .await?;

    Ok(())
}

fn build_email_body(id: &String) -> String {
    let template_body = include_str!("../../templates/confirm.html");

    let email_body = template_body.replace("%RECIPIENT_ID%", id);

    return email_body;
}

async fn write_to_ddb(
    ddb_client: &ddb::Client,
    name: &str,
    email: &str,
    id: &String,
) -> Result<(), Error> {
    let mailing_list_ddb_id =
        env::var("MAILING_LIST_TABLE_NAME").expect("MAILING_LIST_TABLE_NAME not set");

    ddb_client
        .put_item()
        .table_name(mailing_list_ddb_id)
        .item("id", ddb::types::AttributeValue::S(id.to_string()))
        .item("name", ddb::types::AttributeValue::S(name.to_string()))
        .item("email", ddb::types::AttributeValue::S(email.to_string()))
        .item("verified", ddb::types::AttributeValue::Bool(false))
        .item("rideStatus", ddb::types::AttributeValue::S("N".to_string()))
        .send()
        .await?;

    Ok(())
}
