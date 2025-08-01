use aws_config::{load_defaults, BehaviorVersion};
use aws_sdk_dynamodb as ddb;
use aws_sdk_sesv2 as ses;
use aws_sdk_ssm as ssm;
use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use serde::Deserialize;
use std::env;

#[derive(Deserialize)]
struct Request {}

struct Attendees {
    yes: Vec<String>,
    maybe: Vec<String>,
}

impl Attendees {
    fn total(&self) -> usize {
        self.yes.len() + self.maybe.len()
    }
}

struct Member {
    id: String,
    name: String,
    ride_status: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .without_time()
        .init();

    run(service_fn(attendance_report)).await
}

async fn attendance_report(_event: LambdaEvent<Request>) -> Result<(), Error> {
    let aws_config = load_defaults(BehaviorVersion::latest()).await;
    let ses_client = ses::Client::new(&aws_config);
    let ssm_client = ssm::Client::new(&aws_config);
    let ddb_client = ddb::Client::new(&aws_config);

    let members = read_all_members(&ddb_client).await?;
    let attendees = get_attendees(&members);

    if attendees.total() == 0 {
        return Ok(());
    }

    reset_member_statuses(&ddb_client, &members).await?;

    let recipients = get_recipients(&ssm_client).await?;
    send_emails(&ses_client, &recipients, &attendees).await?;

    Ok(())
}

async fn read_all_members(ddb_client: &ddb::Client) -> Result<Vec<Member>, Error> {
    let mailing_list_ddb_id =
        env::var("MAILING_LIST_TABLE_NAME").expect("MAILING_LIST_TABLE_NAME not set");

    let ddb_resp = ddb_client
        .scan()
        .table_name(mailing_list_ddb_id)
        .send()
        .await?
        .items
        .expect("No items found");

    let mut members: Vec<Member> = Vec::new();

    for item in ddb_resp {
        let id = match item.get("id") {
            Some(value) => match value.as_s() {
                Ok(value_str) => value_str.to_string(),
                Err(err) => {
                    println!("[ERROR] {:?}", err);
                    continue;
                }
            },
            None => {
                println!("[ERROR] No id found");
                continue;
            }
        };

        let name = match item.get("name") {
            Some(value) => match value.as_s() {
                Ok(value_str) => value_str.to_string(),
                Err(err) => {
                    println!("[ERROR] {:?}", err);
                    continue;
                }
            },
            None => {
                println!("[ERROR] No name found");
                continue;
            }
        };

        let ride_status = match item.get("rideStatus") {
            Some(value) => match value.as_s() {
                Ok(value_str) => value_str.to_string(),
                Err(err) => {
                    println!("[ERROR] {:?}", err);
                    continue;
                }
            },
            None => {
                println!("[ERROR] No rideStatus found");
                continue;
            }
        };

        members.push(Member {
            id,
            name,
            ride_status,
        });
    }

    Ok(members)
}

fn get_attendees(members: &Vec<Member>) -> Attendees {
    Attendees {
        yes: members
            .iter()
            .filter(|member| member.ride_status == "Y")
            .map(|member| member.name.clone())
            .collect(),
        maybe: members
            .iter()
            .filter(|member| member.ride_status == "M")
            .map(|member| member.name.clone())
            .collect(),
    }
}

async fn reset_member_statuses(
    ddb_client: &ddb::Client,
    members: &Vec<Member>,
) -> Result<(), Error> {
    let mailing_list_ddb_id =
        env::var("MAILING_LIST_TABLE_NAME").expect("MAILING_LIST_TABLE_NAME not set");

    for member in members {
        ddb_client
            .update_item()
            .table_name(&mailing_list_ddb_id)
            .key("id", ddb::types::AttributeValue::S(member.id.to_string()))
            .update_expression("SET rideStatus = :status")
            .expression_attribute_values(":status", ddb::types::AttributeValue::S("N".to_string()))
            .send()
            .await?;
    }
    Ok(())
}

async fn get_recipients(ssm_client: &ssm::Client) -> Result<Vec<String>, Error> {
    let admin_emails_ssm_id = env::var("ADMIN_EMAILS_SSM").expect("RECIPIENTS_SSM_ID not set");

    let recipients_param = ssm_client
        .get_parameter()
        .name(admin_emails_ssm_id)
        .with_decryption(true)
        .send()
        .await?;

    let recipients = recipients_param
        .parameter
        .expect("No recipients found")
        .value
        .expect("No recipients found");

    Ok(recipients.split(",").map(|s| s.to_string()).collect())
}

async fn send_emails(
    ses_client: &ses::Client,
    recipients: &Vec<String>,
    attendees: &Attendees,
) -> Result<(), Error> {
    for recipient in recipients {
        send_email(ses_client, recipient.to_string(), attendees).await?;
    }
    Ok(())
}

async fn send_email(
    ses_client: &ses::Client,
    recipient: String,
    attendees: &Attendees,
) -> Result<(), Error> {
    let mut destination: ses::types::Destination = ses::types::Destination::builder().build();
    destination.to_addresses = Some(vec![recipient]);

    let subject_content = ses::types::Content::builder()
        .data("This week's riders")
        .charset("UTF-8")
        .build()
        .expect("Unable to build subject content");

    let body_content = ses::types::Content::builder()
        .data(build_email_body(&attendees))
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

fn build_email_body(attendees: &Attendees) -> String {
    let template_body = include_str!("../../templates/attendance.html");

    let email_body = template_body
        .replace(
            "%ADMIN_EMAIL%",
            env!(
                "ADMIN_EMAIL",
                "No admin email was provided by the environment"
            ),
        )
        .replace("%YES_LIST%", &attendees.yes.join("\n"))
        .replace("%MAYBE_LIST%", &attendees.maybe.join("\n"));

    return email_body;
}
