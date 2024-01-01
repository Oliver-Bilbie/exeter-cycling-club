use aws_sdk_ses as ses;
use aws_config::{load_defaults, BehaviorVersion};
use lambda_http::{run, service_fn, Body, Error, Request, Response};
use serde_json::Value as JsonValue;

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .without_time()
        .init();

    run(service_fn(function_handler)).await
}

async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    let body = read_event(event)?;
    let contact_email = body["contact_email"].as_str().expect("No email provided");
    let message = body["message"].as_str().expect("No message provided");

    println!("Email: {}", contact_email);
    println!("Message: {}", message);

    let aws_config = load_defaults(BehaviorVersion::latest()).await;
    let ses_client = ses::Client::new(&aws_config);
    let mailing_list = "mrweenie@tuta.io";


    let resp = Response::builder()
        .status(200)
        .header("content-type", "text/html")
        .body(format!("Hello my dude, this is an AWS Lambda HTTP request").into())
        .map_err(Box::new)?;
    Ok(resp)
}

fn read_event(event: Request) -> Result<JsonValue, Error> {
    let body: JsonValue = match event.body() {
        Body::Text(text) => serde_json::from_str(text).expect("Unable to parse body"),
        Body::Binary(items) => {
            let mut body = String::new();
            for item in items {
                body.push_str(&format!("{:?}", item));
            }
            serde_json::from_str(&body).expect("Unable to parse body")
        }
        _ => panic!("No event body was provided"),
    };
    Ok(body)
}
