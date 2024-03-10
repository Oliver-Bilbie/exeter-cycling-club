use aws_config::{load_defaults, BehaviorVersion};
use aws_sdk_sesv2 as ses;
use aws_sdk_ssm as ssm;
use lambda_http::{run, service_fn, Body, Error, Request, Response};
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
struct ContactUsRequest {
    contact_email: String,
    message: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .without_time()
        .init();

    run(service_fn(contact_us)).await
}

async fn contact_us(event: Request) -> Result<Response<Body>, Error> {
    let body = read_event_body(event)?;
    let contact_email = body.contact_email;
    let message = body.message;

    let aws_config = load_defaults(BehaviorVersion::latest()).await;
    let ssm_client = ssm::Client::new(&aws_config);
    let ses_client = ses::Client::new(&aws_config);

    let recipients = get_admin_list(&ssm_client).await?;

    let message = format!("Message from: {}\n\n{}", contact_email, message);
    send_emails(&ses_client, &recipients, &message).await?;

    Ok(Response::builder()
        .status(200)
        .header("content-type", "text/html")
        .body(
            json!({ "message": "Message sent successfully" })
                .to_string()
                .into(),
        )
        .map_err(Box::new)?)
}

async fn get_admin_list(ssm_client: &ssm::Client) -> Result<Vec<String>, Error> {
    let ssm_resp = ssm_client
        .get_parameter()
        .name("ecc-admin-emails")
        .with_decryption(true)
        .send()
        .await?;

    let ssm_value = ssm_resp
        .parameter
        .expect("No parameter found")
        .value
        .expect("No value found");

    let admin_list: Vec<String> = ssm_value.split(",").map(|s| s.to_string()).collect();

    Ok(admin_list)
}

async fn send_emails(
    ses_client: &ses::Client,
    recipients: &Vec<String>,
    message: &String,
) -> Result<(), Error> {
    for recipient in recipients {
        send_email(ses_client, recipient.to_string(), message).await?;
    }
    Ok(())
}

async fn send_email(
    ses_client: &ses::Client,
    recipient: String,
    message: &String,
) -> Result<(), Error> {
    let mut destination: ses::types::Destination = ses::types::Destination::builder().build();
    destination.to_addresses = Some(vec![recipient]);

    let subject_content = ses::types::Content::builder()
        .data("Message from website")
        .charset("UTF-8")
        .build()
        .expect("Unable to build subject content");

    let body_content = ses::types::Content::builder()
        .data(message)
        .charset("UTF-8")
        .build()
        .expect("Unable to build body content");

    let body = ses::types::Body::builder().text(body_content).build();

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

fn read_event_body(event: Request) -> Result<ContactUsRequest, Error> {
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
