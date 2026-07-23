use aws_config::{load_defaults, BehaviorVersion};
use aws_sdk_dynamodb as ddb;
use aws_sdk_sesv2 as ses;
use aws_sdk_ssm as ssm;
use ecc_lib::ddb as ddb_util;
use ecc_lib::email;
use ecc_lib::http::{json_message, parse_body};
use ecc_lib::ssm as ssm_util;
use ecc_lib::Route;
use lambda_http::{run, service_fn, Body, Error, Request, Response};
use reqwest::Client as ReqwestClient;
use serde::Deserialize;
use std::env;

#[derive(Deserialize)]
struct SetRouteRequest {
    access_token: String,
    id: String,
    name: String,
    message: String,
    is_private: String,
}

struct StravaRouteData {
    distance: f64,
    elevation_gain: f64,
    map_url: String,
}

struct EmailRecipient {
    email: String,
    id: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .without_time()
        .init();

    let aws_config = load_defaults(BehaviorVersion::latest()).await;
    let ddb_client = ddb::Client::new(&aws_config);
    let ses_client = ses::Client::new(&aws_config);
    let ssm_client = ssm::Client::new(&aws_config);
    let reqwest_client = ReqwestClient::new();

    run(service_fn(move |event| {
        let ddb_client = ddb_client.clone();
        let ses_client = ses_client.clone();
        let ssm_client = ssm_client.clone();
        let reqwest_client = reqwest_client.clone();
        async move {
            set_route(
                event,
                &ddb_client,
                &ses_client,
                &ssm_client,
                &reqwest_client,
            )
            .await
        }
    }))
    .await
}

async fn set_route(
    event: Request,
    ddb_client: &ddb::Client,
    ses_client: &ses::Client,
    ssm_client: &ssm::Client,
    reqwest_client: &ReqwestClient,
) -> Result<Response<Body>, Error> {
    let body = parse_body::<SetRouteRequest>(&event)?;

    if !check_if_admin(ssm_client, reqwest_client, &body.access_token).await? {
        return json_message(401, "Unauthorized");
    }

    let route_data = get_route_data(reqwest_client, &body.access_token, &body.id).await?;
    let route = Route {
        status: "ready".to_string(),
        id: body.id,
        name: body.name,
        message: body.message,
        distance: format!("{:.0}km", route_data.distance / 1000.0),
        elevation_gain: format!("{:.0}m", route_data.elevation_gain),
        map_url: route_data.map_url,
        is_private: body.is_private,
    };

    update_route_data(ssm_client, &route).await?;
    send_email_notifications(ddb_client, ses_client, &route).await?;

    json_message(200, "Route set successfully")
}

async fn check_if_admin(
    ssm_client: &ssm::Client,
    reqwest_client: &ReqwestClient,
    access_token: &str,
) -> Result<bool, Error> {
    let user_id = get_user_id(reqwest_client, access_token).await?;
    let admin_list = get_admin_list(ssm_client).await?;
    Ok(admin_list.contains(&user_id))
}

async fn get_user_id(reqwest_client: &ReqwestClient, access_token: &str) -> Result<String, Error> {
    #[derive(Deserialize)]
    struct StravaAthlete {
        id: i32,
    }

    let athlete: StravaAthlete = reqwest_client
        .get("https://www.strava.com/api/v3/athlete")
        .header("Authorization", format!("Bearer {access_token}"))
        .send()
        .await?
        .json()
        .await?;

    Ok(athlete.id.to_string())
}

async fn get_admin_list(ssm_client: &ssm::Client) -> Result<Vec<String>, Error> {
    ssm_util::get_csv_list(ssm_client, env::var("ADMIN_IDS_SSM")?).await
}

async fn get_route_data(
    reqwest_client: &ReqwestClient,
    access_token: &str,
    route_id: &str,
) -> Result<StravaRouteData, Error> {
    #[derive(Deserialize)]
    struct StravaMapUrls {
        retina_url: String,
    }
    #[derive(Deserialize)]
    struct StravaRoute {
        distance: f64,
        elevation_gain: f64,
        map_urls: StravaMapUrls,
    }

    let route: StravaRoute = reqwest_client
        .get(format!("https://www.strava.com/api/v3/routes/{route_id}"))
        .header("Authorization", format!("Bearer {access_token}"))
        .send()
        .await?
        .json()
        .await?;

    Ok(StravaRouteData {
        distance: route.distance,
        elevation_gain: route.elevation_gain,
        map_url: route.map_urls.retina_url,
    })
}

async fn update_route_data(ssm_client: &ssm::Client, route: &Route) -> Result<(), Error> {
    let route_data_ssm_id = env::var("ROUTE_DATA_SSM")?;
    let route_json = serde_json::to_string(route)?;
    ssm_util::put_parameter(ssm_client, route_data_ssm_id, route_json).await
}

async fn send_email_notifications(
    ddb_client: &ddb::Client,
    ses_client: &ses::Client,
    route: &Route,
) -> Result<(), Error> {
    let mailing_list = get_mailing_list(ddb_client).await?;
    for recipient in mailing_list {
        match send_email(ses_client, &recipient, route).await {
            Ok(()) => tracing::info!("Email sent to {}", recipient.email),
            Err(err) => tracing::error!("{:?}", err),
        }
    }
    Ok(())
}

async fn get_mailing_list(ddb_client: &ddb::Client) -> Result<Vec<EmailRecipient>, Error> {
    let table_name = env::var("MAILING_LIST_TABLE_NAME")?;

    let items = ddb_client
        .scan()
        .table_name(table_name)
        .filter_expression("#verified = :v")
        .expression_attribute_names("#verified", "verified")
        .expression_attribute_values(":v", ddb::types::AttributeValue::Bool(true))
        .send()
        .await?
        .items
        .unwrap_or_default();

    Ok(items
        .iter()
        .filter_map(|item| {
            let email = ddb_util::attr::<String>(item, "email")?;
            let id = ddb_util::attr::<String>(item, "id")?;
            let verified = ddb_util::attr::<bool>(item, "verified")?;
            verified.then_some(EmailRecipient { email, id })
        })
        .collect())
}

async fn send_email(
    ses_client: &ses::Client,
    recipient: &EmailRecipient,
    route: &Route,
) -> Result<(), Error> {
    email::send_html(
        ses_client,
        recipient.email.clone(),
        "This week's route",
        build_email_body(route, recipient),
    )
    .await
}

fn build_email_body(route: &Route, recipient: &EmailRecipient) -> String {
    if route.is_private() {
        include_str!("../../templates/update-private.html")
            .replace("%RECIPIENT_ID%", &recipient.id)
            .replace("$NEWLINE", "\n")
    } else {
        include_str!("../../templates/update.html")
            .replace("%ROUTE_NAME%", &route.name)
            .replace("%DESCRIPTION%", &route.message)
            .replace("%DISTANCE%", &route.distance)
            .replace("%ELEVATION_GAIN%", &route.elevation_gain)
            .replace("%MAP_URL%", &route.map_url)
            .replace("%RECIPIENT_ID%", &recipient.id)
            .replace("$NEWLINE", "\n")
    }
}
