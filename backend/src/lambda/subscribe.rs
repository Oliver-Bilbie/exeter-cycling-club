use aws_config::{load_defaults, BehaviorVersion};
use aws_sdk_dynamodb as ddb;
use aws_sdk_sesv2 as ses;
use lambda_http::{run, service_fn, Body, Error, Request, Response};
use serde_json::json;
use serde::Deserialize;
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

    let email_exists = ddb_client
        .query()
        .table_name("ecc-mailing-list")
        .index_name("EmailIndex")
        .key_condition_expression("#email = :email")
        .expression_attribute_names("#email", "email")
        .expression_attribute_values(":email", ddb::types::AttributeValue::S(email.to_string()))
        .send()
        .await?
        .count
        > 0;

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

    // TODO: Send a custom verification email once SES is out of sandbox
    // Once this is done, move the dynamodb write to the email verification lambda
    ses_client
        .create_email_identity()
        .email_identity(&email)
        .send()
        .await?;

    let id = Uuid::new_v4().to_string();
    ddb_client
        .put_item()
        .table_name("ecc-mailing-list")
        .item("id", ddb::types::AttributeValue::S(id))
        .item("name", ddb::types::AttributeValue::S(name.to_string()))
        .item("email", ddb::types::AttributeValue::S(email.to_string()))
        .item("verified", ddb::types::AttributeValue::Bool(false))
        .item("rideStatus", ddb::types::AttributeValue::S("N".to_string()))
        .send()
        .await?;

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
