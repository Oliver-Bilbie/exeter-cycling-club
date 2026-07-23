use aws_config::{load_defaults, BehaviorVersion};
use aws_sdk_dynamodb as ddb;
use aws_sdk_sesv2 as ses;
use aws_sdk_ssm as ssm;
use ecc_lib::ddb as ddb_util;
use ecc_lib::email;
use ecc_lib::ssm as ssm_util;
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

    fn from_members(members: &[Member]) -> Self {
        Self {
            yes: members
                .iter()
                .filter(|m| m.ride_status == "Y")
                .map(|m| m.name.clone())
                .collect(),
            maybe: members
                .iter()
                .filter(|m| m.ride_status == "M")
                .map(|m| m.name.clone())
                .collect(),
        }
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

    let aws_config = load_defaults(BehaviorVersion::latest()).await;
    let ses_client = ses::Client::new(&aws_config);
    let ssm_client = ssm::Client::new(&aws_config);
    let ddb_client = ddb::Client::new(&aws_config);

    run(service_fn(move |event| {
        let ses_client = ses_client.clone();
        let ssm_client = ssm_client.clone();
        let ddb_client = ddb_client.clone();
        async move { attendance_report(event, &ses_client, &ssm_client, &ddb_client).await }
    }))
    .await
}

async fn attendance_report(
    _event: LambdaEvent<Request>,
    ses_client: &ses::Client,
    ssm_client: &ssm::Client,
    ddb_client: &ddb::Client,
) -> Result<(), Error> {
    let members = read_all_members(ddb_client).await?;
    let attendees = Attendees::from_members(&members);

    if attendees.total() == 0 {
        return Ok(());
    }

    reset_member_statuses(ddb_client, &members).await?;

    let recipients = ssm_util::get_csv_list(ssm_client, env::var("ADMIN_EMAILS_SSM")?).await?;
    for recipient in &recipients {
        email::send_html(
            ses_client,
            recipient.clone(),
            "This week's riders",
            build_email_body(&attendees),
        )
        .await?;
    }

    Ok(())
}

async fn read_all_members(ddb_client: &ddb::Client) -> Result<Vec<Member>, Error> {
    let table_name = env::var("MAILING_LIST_TABLE_NAME")?;

    let items = ddb_client
        .scan()
        .table_name(table_name)
        .send()
        .await?
        .items
        .unwrap_or_default();

    Ok(items
        .iter()
        .filter_map(|item| {
            Some(Member {
                id: ddb_util::attr::<String>(item, "id")?,
                name: ddb_util::attr::<String>(item, "name")?,
                ride_status: ddb_util::attr::<String>(item, "rideStatus")?,
            })
        })
        .collect())
}

async fn reset_member_statuses(ddb_client: &ddb::Client, members: &[Member]) -> Result<(), Error> {
    let table_name = env::var("MAILING_LIST_TABLE_NAME")?;

    for member in members {
        ddb_client
            .update_item()
            .table_name(&table_name)
            .key("id", ddb::types::AttributeValue::S(member.id.clone()))
            .update_expression("SET rideStatus = :status")
            .expression_attribute_values(":status", ddb::types::AttributeValue::S("N".to_string()))
            .send()
            .await?;
    }

    Ok(())
}

fn build_email_body(attendees: &Attendees) -> String {
    include_str!("../../templates/attendance.html")
        .replace(
            "%ADMIN_EMAIL%",
            env!(
                "ADMIN_EMAIL",
                "No admin email was provided by the environment"
            ),
        )
        .replace("%YES_LIST%", &attendees.yes.join("\n"))
        .replace("%MAYBE_LIST%", &attendees.maybe.join("\n"))
}
