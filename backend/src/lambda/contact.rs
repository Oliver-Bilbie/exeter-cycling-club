use aws_config::{load_defaults, BehaviorVersion};
use aws_sdk_sesv2 as ses;
use aws_sdk_ssm as ssm;
use ecc_lib::email;
use ecc_lib::http::{json_message, parse_body};
use ecc_lib::ssm as ssm_util;
use lambda_http::{run, service_fn, Body, Error, Request, Response};
use serde::Deserialize;

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

    let aws_config = load_defaults(BehaviorVersion::latest()).await;
    let ssm_client = ssm::Client::new(&aws_config);
    let ses_client = ses::Client::new(&aws_config);

    run(service_fn(move |event| {
        let ssm_client = ssm_client.clone();
        let ses_client = ses_client.clone();
        async move { contact_us(event, &ssm_client, &ses_client).await }
    }))
    .await
}

async fn contact_us(
    event: Request,
    ssm_client: &ssm::Client,
    ses_client: &ses::Client,
) -> Result<Response<Body>, Error> {
    let body = parse_body::<ContactUsRequest>(&event)?;
    let recipients = ssm_util::get_csv_list(ssm_client, "ecc-admin-emails").await?;
    let message = format!("Message from: {}\n\n{}", body.contact_email, body.message);

    for recipient in &recipients {
        email::send_text(
            ses_client,
            recipient.clone(),
            "Message from website",
            &message,
        )
        .await?;
    }

    json_message(200, "Message sent successfully")
}
