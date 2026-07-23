use aws_sdk_sesv2 as ses;

type Error = Box<dyn std::error::Error + Send + Sync>;

// TODO: Replace with production domain
const FROM_ADDRESS: &str = "Exeter Cycling Club <ecc@oliver-bilbie.co.uk>";

pub async fn send_html(
    ses_client: &ses::Client,
    recipient: impl Into<String>,
    subject: &str,
    html_body: String,
) -> Result<(), Error> {
    send(ses_client, recipient, subject, EmailBody::Html(html_body)).await
}

pub async fn send_text(
    ses_client: &ses::Client,
    recipient: impl Into<String>,
    subject: &str,
    text_body: &str,
) -> Result<(), Error> {
    send(
        ses_client,
        recipient,
        subject,
        EmailBody::Text(text_body.to_string()),
    )
    .await
}

enum EmailBody {
    Html(String),
    Text(String),
}

async fn send(
    ses_client: &ses::Client,
    recipient: impl Into<String>,
    subject: &str,
    body: EmailBody,
) -> Result<(), Error> {
    let destination = ses::types::Destination::builder()
        .to_addresses(recipient)
        .build();

    let subject_content = ses::types::Content::builder()
        .data(subject)
        .charset("UTF-8")
        .build()?;

    let body_content = match &body {
        EmailBody::Html(html) => ses::types::Content::builder()
            .data(html)
            .charset("UTF-8")
            .build()?,
        EmailBody::Text(text) => ses::types::Content::builder()
            .data(text)
            .charset("UTF-8")
            .build()?,
    };

    let body = match body {
        EmailBody::Html(_) => ses::types::Body::builder().html(body_content).build(),
        EmailBody::Text(_) => ses::types::Body::builder().text(body_content).build(),
    };

    let message = ses::types::Message::builder()
        .subject(subject_content)
        .body(body)
        .build();

    let email_content = ses::types::EmailContent::builder().simple(message).build();

    ses_client
        .send_email()
        .from_email_address(FROM_ADDRESS)
        .destination(destination)
        .content(email_content)
        .send()
        .await?;

    Ok(())
}
